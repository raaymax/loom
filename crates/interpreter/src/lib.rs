mod vtype;
mod context;
mod builtins;
use std::rc::Rc;
use std::cell::RefCell;

use parser::{Node, Op};
use lexer::PError;
use builtins::Builtin;
pub use vtype::VType;
use context::Context;

use crate::context::StackError;


pub fn extract_names(node: &Node) -> Result<Vec<String>,PError> {
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

pub fn compute(node: &Node, context: &Rc<RefCell<Context>>) -> Result<VType, PError> {
    if let Some(v) = context.borrow().get_var("!return") {
        //println!("Ret: {} {}", v, node);
        return Ok(v.clone())
    }
    match node.op {
        Op::Return => {
            let value = compute(node.get_child(0), context)?;
            context.borrow_mut().define_var("!return", value.clone());
            Ok(value)
        },
        Op::DefineFunc => {
            let name = node.get_child(0);
            let args = node.get_child(1);
            let body = node.get_child(2);
            let arg_names = extract_names(args)?;
            let func = VType::Func(arg_names, body.clone());
            context.borrow_mut().define_var(name.id.as_ref().unwrap(), func);
            //dict.insert(name.id.as_ref().unwrap().clone(), func);
            Ok(VType::Ref(name.id.as_ref().unwrap().clone()))
        },
        Op::DefineVar => {
            context.borrow_mut().define_var(node.id.as_ref().unwrap(), VType::Undefined);
            if !node.children.is_empty() {
                compute(node.get_child(0), context)?;
            }
            Ok(VType::Undefined)
        },
        Op::While => {
            let mut last = VType::Undefined;
            while context.borrow().get_var("!return").is_none() && compute(node.get_child(0), context)? != VType::Bool(false) {
                last = compute(node.get_child(1), context)?;
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
            let func = compute(node.get_child(0), context)?;
            match func {
                VType::Builtin(func)  => {
                    let args = compute(node.get_child(1), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    func.call(&args)
                },
                VType::Ref(name) => {
                    let args = compute(node.get_child(1), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let func = context.borrow().get_var(&name).unwrap();
                    match func {
                        VType::Func(arg_names, body) => {
                            let child_context = Context::new_child(&name, node.location, context);
                            for (idx, arg) in arg_names.iter().enumerate() {
                                child_context.borrow_mut().define_var(arg, args.get(idx).unwrap().clone())
                            }
                            compute(&body, &child_context)
                        },
                        _ => {
                            return Err(PError::new(node.location, "Not a function"));
                        }
                    }
                },
                VType::Func(arg_names, body) => {
                    let args = compute(node.get_child(1), context)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let child_context = Context::new_child(node.id.as_ref().unwrap_or(&"".to_string()).as_str(), node.location, context);
                    for (idx, arg) in arg_names.iter().enumerate() {
                        child_context.borrow_mut().define_var(arg, args.get(idx).unwrap().clone())
                    }
                    compute(&body, &child_context)
                },
                _ => {
                    return Err(PError::new(node.location, "Not a function"));
                }
            }
        },
        Op::Branch=> {
            let cond = compute(node.get_child(0), context)?;
            if let VType::Number(c) = cond {
                if c != 0 {
                    return compute(node.get_child(1), context);
                } else {
                    return compute(node.get_child(2), context);
                }
            } if let VType::Bool(b) = cond {
                if b {
                    return compute(node.get_child(1), context);
                } else {
                    return compute(node.get_child(2), context);
                }
            } else {
                return Err(PError::new(node.location, format!("Invalid condition {}", cond).as_str()));
            }
        },
        Op::Paren => {
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, context)?;
            }
            Ok(last)
        },
        Op::Scope | Op::Loop => {
            let mut last = VType::Undefined;
            let child_context = Context::new_child("child", node.location, context);
            for child in &node.children {
                last = compute(child, &child_context)?;
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
        Op::And => compute_binary!(node, context, and),
        Op::Or => compute_binary!(node, context, or),


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
            if !context.borrow().exists(id) {
                return Err(PError::new(node.location, format!("Variable {} not defined", id).as_str()));
            }

            let Some(val) = context.borrow().get_var(id) else {
                return Err(PError::new(node.location, format!("Variable {} not defined", id).as_str()));
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
            if !context.borrow().exists(left) {
                return Err(PError::new(node.location, format!("Variable {} not defined", left).as_str()));
            }
            let right = compute(node.right().unwrap(), context)?;
            if let Err(err) = context.borrow_mut().set_var(left, right.clone()) {
                context.borrow().print_stack();
                match err {
                    StackError::ValueError => {
                        return Err(PError::new(node.location, format!("Value error setting var {}", left).as_str()));
                    },
                    StackError::RefError => {
                        return Err(PError::new(node.location, format!("Ref error setting var {}", left).as_str()));
                    }
                }
            }
            //dict.insert(left.clone(), right.clone());
            Ok(right)
        }
    }
}

pub fn interpret(node: Node) -> Result<VType, PError> {
    let context = Context::new("global", node.location);
    compute(&node, &context)
}

