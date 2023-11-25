use std::slice::Iter;
use lexer::{Token, PError, Location};
use super::{Op, Expression, Node};

pub struct Params;

impl Params{
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Args, token.get_location());
        loop {
            let Some(token) = iter.next() else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            if matches!(token, Token::RParen(..)) {
                return Ok((tree, Some(token.clone())));
            }
            let Token::Id(loc, id) = token else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            let idNode = Node::new(Op::Var, *loc).set_id(id.clone());
            tree.add(idNode);

            let tok = iter.next();
            let Some(t) = tok else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            match t{
                Token::RParen(..)  => {
                    return Ok((tree, Some(t.clone())));
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
