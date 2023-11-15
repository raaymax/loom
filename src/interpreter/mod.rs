use std::collections::HashMap;

use crate::parser::{Node, Op, Value};
use crate::errors::PError;

pub fn compute(node: &Node, dict: &mut HashMap<String, Value>) -> Result<Value, PError> {
    match node.op {
        Op::Root => {
            compute(node.right.as_ref().unwrap(), dict)
        },
        Op::Add => {
            let left = compute(node.left.as_ref().unwrap(), dict)?;
            let right = compute(node.right.as_ref().unwrap(), dict)?;
            Ok(left + right)
        },
        Op::Sub => {
            let left = compute(node.left.as_ref().unwrap(), dict)?;
            let right = compute(node.right.as_ref().unwrap(), dict)?;
            Ok(left - right)
        },
        Op::Mul => {
            let left = compute(node.left.as_ref().unwrap(), dict)?;
            let right = compute(node.right.as_ref().unwrap(), dict)?;
            Ok(left * right)
        },
        Op::Div => {
            let left = compute(node.left.as_ref().unwrap(), dict)?;
            let right = compute(node.right.as_ref().unwrap(), dict)?;
            Ok(left / right)
        },
        Op::Def => {
            let val = match &node.value {
                Some(val) => {
                    val
                },
                _ => {
                    let Some(_) = node.id else {
                        return Err(PError::new(node.location, "No id or value for definition"));
                    };
                    let Some(val) = dict.get(node.id.as_ref().unwrap()) else {
                        return Err(PError::new(node.location, "Variable not defined"));
                    };
                    return Ok(val.clone());
                }
            };
            Ok(val.clone())
        },
        Op::Assign => {
            let Some(ref left) = node.left.as_ref().unwrap().id else {
                return Err(PError::new(node.location, "Invalid LHS for assignment"));
            };
            let right = compute(node.right.as_ref().unwrap(), dict)?;
            dict.insert(left.clone(), right.clone());
            Ok(right)
        }

    }
    
}

pub fn interpret(nodes: &Vec<Node>) -> Result<Value, PError> {
    let mut dict = HashMap::new();
    let mut last = None;
    for node in nodes {
        last = Some(compute(node, &mut dict)?)
    }
    Ok(last.unwrap())
}

