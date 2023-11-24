use std::slice::Iter;
use lexer::{Token, PError, Location};
use super::{Op, Expression, Node};

pub struct Args;

impl Args{
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Args, token.get_location());
        loop {
            let (node, tok) = Expression::consume(token, iter, level + 1)?;
            tree.add(node);

            let Some(t) = tok else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            match t{
                Token::RParen(..)  => {
                    return Ok((tree, Some(t)));
                },
                Token::Comma(..) => {
                    continue;
                },
                Token::Eof => {
                    return Err(PError::new(t.get_location(), format!("Unexpected end of file").as_str()))?;
                },
                _ => Err(PError::new(t.get_location(), format!("Unexpected token, missed semicolon?").as_str()))?,
            }
        }
    }
}
