use crate::parser::{Node, Op};
use crate::errors::PError;

pub fn interpret(node: &Node) -> Result<i32, PError> {
    match node.op {
        Op::Root => {
            interpret(node.right.as_ref().unwrap())
        },
        Op::Add => {
            let left = interpret(node.left.as_ref().unwrap())?;
            let right = interpret(node.right.as_ref().unwrap())?;
            Ok(left + right)
        },
        Op::Sub => {
            let left = interpret(node.left.as_ref().unwrap())?;
            let right = interpret(node.right.as_ref().unwrap())?;
            Ok(left - right)
        },
        Op::Mul => {
            let left = interpret(node.left.as_ref().unwrap())?;
            let right = interpret(node.right.as_ref().unwrap())?;
            Ok(left * right)
        },
        Op::Div => {
            let left = interpret(node.left.as_ref().unwrap())?;
            let right = interpret(node.right.as_ref().unwrap())?;
            Ok(left / right)
        },
        Op::Def => {
            let Some(val) = node.value else {
                let Some(ref id) = node.id else {
                    return Err(PError::new(node.location, "No id or value for definition"));
                };
                return Err(PError::new(node.location, "variables are not implemented yet"));
            };
            Ok(node.value.unwrap() as i32)
        },

    }
    
}

