mod vtype;
use std::collections::HashMap;

use parser::{Node, Op, Value};
use lexer::PError;
use vtype::VType;


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
            "print" => Some(VType::BuiltinFunction("print".to_string())),
            _ => None,
        }
        
    }
}

pub fn compute(node: &Node, dict: &mut HashMap<String, VType>) -> Result<VType, PError> {
    match node.op {
        Op::Call => {
            let func = compute(node.children.get(1).unwrap(), dict)?;
            match func {
                VType::BuiltinFunction(name) if name == "print" => {
                    println!("{}", compute(node.children.get(0).unwrap(), dict)?);
                },
                _ => {
                    return Err(PError::new(node.location, "Not a function"));
                }
            }
            Ok(VType::Undefined)
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

    }
    
}

pub fn interpret(node: Node) -> Result<VType, PError> {
    let mut dict = HashMap::new();
    compute(&node, &mut dict)
}

