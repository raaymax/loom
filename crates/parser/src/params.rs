use std::slice::Iter;
use lexer::{Token, PError, Location};
use super::{Op, Node, Parser};

pub struct Params;

impl Parser for Params{
    fn consume(token:  &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
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
            let id_node = Node::new(Op::Variable, *loc).set_id(id.clone());
            tree.add(id_node);

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
                    return Err(PError::new(t.get_location(), "Unexpected end of file"))?;
                },
                _ => Err(PError::new(t.get_location(), "Unexpected token, missed semicolon?"))?,
            }
        }
    }
}
