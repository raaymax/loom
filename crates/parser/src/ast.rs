use std::slice::Iter;
use crate::{Location, Token, PError};

use super::{ast_node::Node, block::Block};

pub fn build(iter: &mut Iter<Token>, level: usize, loc: Location ) -> Result<Node, PError> {
    let (node, ret) = Block::consume(&Token::Start, iter, level)?;
    if let Some(token) = ret {
        if let Token::Eof = token {
            return Ok(node);
        }
        return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()));
    }
    Ok(node)
}
