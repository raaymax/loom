use std::collections::HashMap;

use parser::{Node, Op, Value};
use lexer::PError;

pub fn compute(node: &Node, dict: &mut HashMap<String, Value>) -> Result<Value, PError> {
    match node.op {
        Op::Call => {
            panic!("Not implemented yet");
        },
        Op:: Branch=> {
            let cond = compute(node.children.get(0).unwrap(), dict)?;
            if let Value::Number(c) = cond {
                if c != 0 {
                    return compute(node.children.get(1).unwrap(), dict);
                } else {
                    return compute(node.children.get(2).unwrap(), dict);
                }
            } else {
                return Err(PError::new(node.location, "Invalid condition"));
            }
            let mut last = Value::Undefined;
            for child in &node.children {
                last = compute(child, dict)?;
            }
            Ok(last)
        },
        Op::Scope | Op::Paren | Op::Loop => {
            let mut last = Value::Undefined;
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
            Ok(val.clone())

        },
        Op::Var => {
            let Some(_) = node.id else {
                return Err(PError::new(node.location, "No id or value for definition"));
            };
            let Some(val) = dict.get(node.id.as_ref().unwrap()) else {
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

pub fn interpret(node: Node) -> Result<Value, PError> {
    let mut dict = HashMap::new();
    compute(&node, &mut dict)
}

