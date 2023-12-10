mod vtype;
mod context;
mod builtins;
use std::rc::Rc;

use parser::{Node, Op};
use lexer::PError;
use builtins::Builtin;
use vtype::VType;
use context::Context;


pub fn extractNames(node: &Node) -> Result<Vec<String>,PError> {
    let mut names = Vec::new();
    for child in &node.children {
        if child.op != Op::Variable {
            return Err(PError::new(child.location, format!("Expected argument name but found: {}", child.op).as_str()));
        }
        names.push(child.id.as_ref().unwrap().clone());
    }
    Ok(names)
}
macro_rules! compute_unary{
    ($node:ident, $ctx: ident, $op:ident) => {
        {
            let left = compute($node.left().unwrap(), $ctx)?;
            Ok(left.$op())
        }
    };
}
macro_rules! compute_binary {
    ($node:ident, $ctx: ident, $op:ident) => {
        {
            let left = compute($node.left().unwrap(), $ctx)?;
            let right = compute($node.right().unwrap(), $ctx)?;
            Ok(left.$op(&right))
        }
    };
}

macro_rules! compute_binop {
    ($node:ident, $ctx: ident, $op:tt) => {
        {
            let left = compute($node.left().unwrap(), $ctx)?;
            let right = compute($node.right().unwrap(), $ctx)?;
            Ok(left $op right)
        }
    };
}

pub fn compute(node: &Node, context: &mut Rc<Context>) -> Result<VType, PError> {
    if let Some(v) = context.get_var("!return") {
        //println!("Ret: {} {}", v, node);
        return Ok(v.clone())
    }
    match node.op {
        Op::Return => {
            let value = compute(node.children.get(0).unwrap(), context)?;
            Rc::get_mut(context).unwrap().set_var("!return", value.clone());
            Ok(value)
        },
        Op::DefineFunc => {
            let name = node.children.get(0).unwrap();
            let args = node.children.get(1).unwrap();
            let body = node.children.get(2).unwrap();
            let argNames = extractNames(args)?;
            let func = VType::Func(argNames, body.clone());
            Rc::get_mut(context).unwrap().set_var(name.id.as_ref().unwrap(), func);
            //dict.insert(name.id.as_ref().unwrap().clone(), func);
            Ok(VType::Ref(name.id.as_ref().unwrap().clone()))
        },
        Op::DefineVar => {
            Rc::get_mut(context).unwrap().define_var(node.id.as_ref().unwrap());
            if !node.children.is_empty() {
                compute(node.children.get(0).unwrap(), context)?;
            }
            Ok(VType::Undefined)
        },
        Op::While => {
            let mut last = VType::Undefined;
            while context.get_var("!return").is_none() && compute(node.children.get(0).unwrap(), context)? != VType::Bool(false) {
                last = compute(node.children.get(1).unwrap(), context)?;
            }
            Ok(last)
        },
        Op::Args => {
            let mut args = Vec::new();
            for child in &node.children {
                args.push(compute(child, context)?);
            }
            Ok(VType::Args(args))
        },
        Op::Call => {
            let func = compute(node.children.get(0).unwrap(), context)?;
            match func {
                VType::Builtin(func)  => {
                    let args = compute(node.children.get(1).unwrap(), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    func.call(&args)
                },
                VType::Ref(name) => {
                    let args = compute(node.children.get(1).unwrap(), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let func = context.get_var(&name).unwrap();
                    match func {
                        VType::Func(argNames, body) => {
                            let mut childContext = Context::new_child(&name, node.location, context);
                            for (idx, arg) in argNames.iter().enumerate() {
                                Rc::get_mut(&mut childContext).unwrap().set_var(arg, args.get(idx).unwrap().clone());
                                //newDict.insert(arg.clone(), args.get(idx).unwrap().clone());
                            }
                            compute(&body, &mut childContext)
                        },
                        _ => {
                            return Err(PError::new(node.location, "Not a function"));
                        }
                    }
                },
                VType::Func(argNames, body) => {
                    let args = compute(node.children.get(1).unwrap(), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let mut childContext = Context::new_child(node.id.as_ref().unwrap_or(&"".to_string()).as_str(), node.location, context);
                    for (idx, arg) in argNames.iter().enumerate() {
                        Rc::get_mut(&mut childContext).unwrap().set_var(arg, args.get(idx).unwrap().clone());
                        //newDict.insert(arg.clone(), args.get(idx).unwrap().clone());
                    }
                    compute(&body, &mut childContext)
                },
                _ => {
                    return Err(PError::new(node.location, "Not a function"));
                }
            }
        },
        Op:: Branch=> {
            let cond = compute(node.children.get(0).unwrap(), context)?;
            if let VType::Number(c) = cond {
                if c != 0 {
                    return compute(node.children.get(1).unwrap(), context);
                } else {
                    return compute(node.children.get(2).unwrap(), context);
                }
            } if let VType::Bool(b) = cond {
                if b {
                    return compute(node.children.get(1).unwrap(), context);
                } else {
                    return compute(node.children.get(2).unwrap(), context);
                }
            } else {
                return Err(PError::new(node.location, format!("Invalid condition {}", cond).as_str()));
            }
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, context)?;
            }
            Ok(last)
        },
        Op::Scope | Op::Paren | Op::Loop => {
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, context)?;
            }
            Ok(last)
        },
        Op::Add => compute_binop!(node, context, +),
        Op::Sub => compute_binop!(node, context, -),
        Op::Mul => compute_binop!(node, context, *),
        Op::Div => compute_binop!(node, context, /),
        Op::Mod => compute_binary!(node, context, modulo),
        Op::Eq => compute_binary!(node, context, equal),
        Op::Neq => compute_binary!(node, context, not_equal),
        Op::Not => compute_unary!(node, context, not),
        Op::Gt => compute_binary!(node, context, gt),
        Op::Lt => compute_binary!(node, context, lt),
        Op::Geq => compute_binary!(node, context, geq),
        Op::Leq => compute_binary!(node, context, leq),


        Op::Value => {
            let Some(val) = node.value.as_ref() else {
                return Err(PError::new(node.location, "No value for definition"));
            };
            Ok(val.clone().into())

        },
        Op::Variable => {
            let Some(ref id) = node.id else {
                return Err(PError::new(node.location, "No id or value for definition"));
            };

            if let Some(v) = Builtin::try_new(id) {
                return Ok(v);
            }

            let Some(val) = context.get_var(id) else {
                return Err(PError::new(node.location, "Variable not defined"));
            };
            /*
            let Some(val) = dict.get(id) else {
                return Err(PError::new(node.location, "Variable not defined"));
            };
            */
            Ok(val.clone())
        },
        Op::Assign => {
            let Some(ref left) = node.left().unwrap().id else {
                return Err(PError::new(node.location, "Invalid LHS for assignment"));
            };
            let right = compute(node.right().unwrap(), context)?;
            Rc::get_mut(context).unwrap().set_var(left, right.clone());
            //dict.insert(left.clone(), right.clone());
            Ok(right)
        }
        _ => {
            return Err(PError::new(node.location, "Not yet implemented"));
        }

    }
    
}

pub fn interpret(node: Node) -> Result<VType, PError> {
    let mut context = Context::new("global", node.location);
    compute(&node, &mut context)
}

