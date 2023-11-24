mod vtype;
use std::{collections::HashMap, fmt::Display};

use parser::{Node, Op, Value};
use lexer::PError;
use vtype::{VType, Builtin};

struct Functions;
impl Functions {
    pub fn call(name: Builtin, args: Args) -> Result<VType, PError> {
        match name {
            Builtin::Print => {
                println!("{}", args);
                Ok(VType::Undefined)
            }
            _ => Err(PError::new(lexer::Location::zero(), "Not yet implemented")),
        }
    }   
}


struct Globals;
impl Globals {
    pub fn is_builtin(name: &str) -> bool {
        match name {
            "print" => true,
            _ => false,
        }
    }
    pub fn builtin(name: &str) -> Option<VType> {
        match name {
            "print" => Some(VType::Builtin(Builtin::Print)),
            _ => None,
        }
        
    }
}

struct Args(Vec<VType>);
impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut first = true;
        for (idx, arg) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        Ok(())
    }
}

pub fn extractNames(node: &Node) -> Result<Vec<String>,PError> {
    let mut names = Vec::new();
    for child in &node.children {
        if child.op != Op::Var {
            return Err(PError::new(child.location, format!("Expected argument name but found: {}", child.op).as_str()));
        }
        names.push(child.id.as_ref().unwrap().clone());
    }
    Ok(names)
}

pub fn compute(node: &Node, dict: &mut HashMap<String, VType>) -> Result<VType, PError> {
    match node.op {
        Op::Func => {
            let name = node.children.get(0).unwrap();
            let args = node.children.get(1).unwrap();
            let body = node.children.get(2).unwrap();
            let argNames = extractNames(args)?;
            let func = VType::Func(argNames, body.clone());
            dict.insert(name.id.as_ref().unwrap().clone(), func);
            Ok(VType::Undefined)
        },
        Op::While => {
            let mut last = VType::Undefined;
            while compute(node.children.get(0).unwrap(), dict)? != VType::Bool(false) {
                last = compute(node.children.get(1).unwrap(), dict)?;
            }
            Ok(last)
        },
        Op::Args => {
            let mut args = Vec::new();
            for child in &node.children {
                args.push(compute(child, dict)?);
            }
            Ok(VType::Args(args))
        },
        Op::Call => {
            let func = compute(node.children.get(0).unwrap(), dict)?;
            match func {
                VType::Builtin(name)  => {
                    let args = compute(node.children.get(1).unwrap(), dict)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    Functions::call(name, Args(args)) 
                },
                VType::Func(argNames, body) => {
                    let args = compute(node.children.get(1).unwrap(), dict)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let mut newDict = dict.clone();
                    for (idx, arg) in argNames.iter().enumerate() {
                        newDict.insert(arg.clone(), args.get(idx).unwrap().clone());
                    }
                    compute(&body, &mut newDict)
                },
                _ => {
                    return Err(PError::new(node.location, "Not a function"));
                }
            }
        },
        Op:: Branch=> {
            let cond = compute(node.children.get(0).unwrap(), dict)?;
            if let VType::Number(c) = cond {
                if c != 0 {
                    return compute(node.children.get(1).unwrap(), dict);
                } else {
                    return compute(node.children.get(2).unwrap(), dict);
                }
            } else {
                return Err(PError::new(node.location, "Invalid condition"));
            }
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, dict)?;
            }
            Ok(last)
        },
        Op::Scope | Op::Paren | Op::Loop => {
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, dict)?;
            }
            Ok(last)
        },
        Op::Add => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left + right)
        },
        Op::Sub => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left - right)
        },
        Op::Mul => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left * right)
        },
        Op::Div => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left / right)
        },
        Op::Mod => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left.modulo(&right))
        },
        Op::Eq => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left.equal(&right))
        },
        Op::Neq => {
            let left = compute(node.left().unwrap(), dict)?;
            let right = compute(node.right().unwrap(), dict)?;
            Ok(left.not_equal(&right))
        },
        Op::Not => {
            let left = compute(node.left().unwrap(), dict)?;
            Ok(left.not())
        },


        Op::Value => {
            let Some(val) = node.value.as_ref() else {
                return Err(PError::new(node.location, "No value for definition"));
            };
            Ok(val.clone().into())

        },
        Op::Var => {
            let Some(ref id) = node.id else {
                return Err(PError::new(node.location, "No id or value for definition"));
            };

            if let Some(v) = Globals::builtin(id) {
                return Ok(v);
            }

            let Some(val) = dict.get(id) else {
                return Err(PError::new(node.location, "Variable not defined"));
            };
            Ok(val.clone())
        },
        Op::Assign => {
            let Some(ref left) = node.left().unwrap().id else {
                return Err(PError::new(node.location, "Invalid LHS for assignment"));
            };
            let right = compute(node.right().unwrap(), dict)?;
            dict.insert(left.clone(), right.clone());
            Ok(right)
        }
        _ => {
            return Err(PError::new(node.location, "Not yet implemented"));
        }

    }
    
}

pub fn interpret(node: Node) -> Result<VType, PError> {
    let mut dict = HashMap::new();
    compute(&node, &mut dict)
}

