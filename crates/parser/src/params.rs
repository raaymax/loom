use std::slice::Iter;
use lexer::{Token, PError, Location};
use super::{Op, Expression, Node};

pub struct Params;

macro_rules! accept {
    ( $iter:ident, $token:ident ) => {
        {
            let Some(token) = $iter.next() else {
                return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };
            if !matches!(token, Token::$token(..)) {
                return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()))
            }
            token
        }
    };
}

impl Params{
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Args, token.get_location());
        loop {
            let Token::Id(loc, id) = accept!(iter, Id) else {
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
