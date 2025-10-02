mod vtype;
mod context;
mod builtins;
mod stack;
use std::{collections::VecDeque, rc::Rc};
use std::cell::RefCell;

use parser::{Node, Op};
use lexer::PError;
use builtins::Builtin;
pub use vtype::VType;
use context::Context;

use crate::{context::StackError, stack::{StackFrame, StackFramePtr}};


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
    ($node:ident, $ctx: ident, $stk: ident, $op:ident) => {
        {
            let left = compute($node.left().unwrap(), $ctx, $stk)?;
            Ok(left.$op())
        }
    };
}

macro_rules! compute_binary {
    ($node:ident, $ctx: ident, $stk: ident, $op:ident) => {
        {
            let left = compute($node.left().unwrap(), $ctx, $stk)?;
            let right = compute($node.right().unwrap(), $ctx, $stk)?;
            Ok(left.$op(&right))
        }
    };
}

macro_rules! compute_binop {
    ($node:ident, $ctx: ident, $stk: ident, $op:tt) => {
        {
            let left = compute($node.left().unwrap(), $ctx, $stk)?;
            let right = compute($node.right().unwrap(), $ctx, $stk)?;
            Ok(left $op right)
        }
    };
}

pub fn compute(node: &Node, context: &Rc<RefCell<Context>>, stack: &StackFramePtr) -> Result<VType, PError> {
    if stack.borrow().has_returned() {
        let ret = stack.borrow().get_return_value().unwrap_or(VType::Undefined).clone();
        return Ok(ret);
    }
    match node.op {
        Op::Return => {
            let value = compute(node.get_child(0), context, stack)?;
            stack.borrow_mut().set_return_value(value.clone());
            Ok(value)
        },
        Op::DefineFunc => {
            let name = node.get_child(0);
            let args = node.get_child(1);
            let body = node.get_child(2);
            let arg_names = extract_names(args)?;
            let func = VType::Func(arg_names, body.clone(), format!("{}", name));
            let mut id = name.id.as_ref().unwrap_or(&"".to_string()).clone();
            if id.is_empty() {
                id = format!("_anon_{}", node.location);
            }
            context.borrow_mut().define_var(id.as_str(), func);
            Ok(VType::Ref(id))
        },
        Op::DefineVar => {
            context.borrow_mut().define_var(node.id.as_ref().unwrap(), VType::Undefined);
            if !node.children.is_empty() {
                compute(node.get_child(0), context, stack)?;
            }
            Ok(VType::Undefined)
        },
        Op::While => {
            let mut last = VType::Undefined;
            while !stack.borrow().has_returned() && compute(node.get_child(0), context, stack)? != VType::Bool(false) {
                last = compute(node.get_child(1), context, stack)?;
            }
            Ok(last)
        },
        Op::Args => {
            let mut args = Vec::new();
            for child in &node.children {
                args.push(compute(child, context, stack)?);
            }
            Ok(VType::Args(args))
        },
        Op::Call => {
            let func = compute(node.get_child(0), context, stack)?;
            match func {
                VType::Builtin(func)  => {
                    let next_stack = StackFrame::new(format!("{}", func).as_str(), node.location, Some(stack.clone()));
                    let args = compute(node.get_child(1), context, stack)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    func.call(node.location, &args, &next_stack)
                },
                VType::Ref(name) => {
                    let next_stack = StackFrame::new(name.as_str(), node.location, Some(stack.clone()));
                    let args = compute(node.get_child(1), context, stack)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let func = context.borrow().get_var(&name).unwrap();
                    match func {
                        VType::Func(arg_names, body, name) => {
                            let child_context = Context::new_child(&name, node.location, context);
                            for (idx, arg) in arg_names.iter().enumerate() {
                                child_context.borrow_mut().define_var(arg, args.get(idx).unwrap().clone())
                            }
                            let val = compute(&body, &child_context, &next_stack)?;
                            let ret = next_stack.borrow().get_return_value().unwrap_or(VType::Undefined).clone();

                            if let VType::Undefined = ret {
                                Ok(val)
                            } else {
                                Ok(ret)
                            }
                        },
                        _ => {
                            return Err(PError::new(node.location, "Not a function"));
                        }
                    }
                },
                VType::Func(arg_names, body, name) => {
                    let next_stack = StackFrame::new(name.as_str(), node.location, Some(stack.clone()));
                    let args = compute(node.get_child(1), context, stack)?;
                    let VType::Args(args) = args else {
                        return Err(PError::new(node.location, "Invalid arguments"));
                    };
                    let child_context = Context::new_child(node.id.as_ref().unwrap_or(&"".to_string()).as_str(), node.location, context);
                    for (idx, arg) in arg_names.iter().enumerate() {
                        child_context.borrow_mut().define_var(arg, args.get(idx).unwrap().clone())
                    }
                    let val = compute(&body, &child_context, &next_stack)?;
                    let ret = next_stack.borrow().get_return_value().unwrap_or(VType::Undefined).clone();
                    if let VType::Undefined = ret {
                        Ok(val)
                    } else {
                        Ok(ret)
                    }
                },
                _ => {
                    return Err(PError::new(node.location, "Not a function"));
                }
            }
        },
        Op::Branch => {
            let cond = compute(node.get_child(0), context, stack)?;
            if let VType::Number(c) = cond {
                if c != 0 {
                    return compute(node.get_child(1), context, stack);
                } else {
                    return compute(node.get_child(2), context, stack);
                }
            } if let VType::Bool(b) = cond {
                if b {
                    return compute(node.get_child(1), context, stack);
                } else {
                    return compute(node.get_child(2), context, stack);
                }
            } else {
                return Err(PError::new(node.location, format!("Invalid condition {}", cond).as_str()));
            }
        },
        Op::Paren => {
            let mut last = VType::Undefined;
            for child in &node.children {
                last = compute(child, context, stack)?;
            }
            Ok(last)
        },
        Op::Scope | Op::Loop => {
            let mut last = VType::Undefined;
            let child_context = Context::new_child("child", node.location, context);
            for (i, child) in node.children.iter().enumerate() {
                last = compute(child, &child_context, stack)?;
            }
            Ok(last)
        },
        Op::Add => compute_binop!(node, context, stack, +),
        Op::Sub => compute_binop!(node, context, stack, -),
        Op::Mul => compute_binop!(node, context, stack, *),
        Op::Div => compute_binop!(node, context, stack, /),
        Op::Mod => compute_binary!(node, context, stack, modulo),
        Op::Eq => compute_binary!(node, context, stack, equal),
        Op::Neq => compute_binary!(node, context, stack, not_equal),
        Op::Not => compute_unary!(node, context, stack, not),
        Op::Gt => compute_binary!(node, context, stack, gt),
        Op::Lt => compute_binary!(node, context, stack, lt),
        Op::Geq => compute_binary!(node, context, stack, geq),
        Op::Leq => compute_binary!(node, context, stack, leq),
        Op::And => compute_binary!(node, context, stack, and),
        Op::Or => compute_binary!(node, context, stack, or),

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

            Ok(val.clone())
        },
        Op::Assign => {
            let Some(ref left) = node.left().unwrap().id else {
                return Err(PError::new(node.location, "Invalid LHS for assignment"));
            };
            if !context.borrow().exists(left) {
                return Err(PError::new(node.location, format!("Variable {} not defined", left).as_str()));
            }
            let right = compute(node.right().unwrap(), context, stack)?;
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
            Ok(right)
        },
        Op::Access => todo!("Access not implemented"),
    }
}


pub fn interpret(node: Node) -> Result<VType, PError> {
    let context = Context::new("global", node.location);
    let stack = StackFrame::new("global", node.location, None);

    compute(&node, &context, &stack)
}

