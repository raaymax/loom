use std::slice::Iter;
use lexer::{Location,Token, PError};
use super::{Node, Expression, Op};
use crate::Parser;

pub struct Branch;

impl Branch {
    fn expect_brace_close(ret: &Option<Token>) -> Result<(), PError> {
        let Some(token) = ret else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"))
        };
        if matches!(token, Token::RParen(..)) {
            Ok(())
        } else {
            Err(PError::new(token.get_location(),format!("Unexpected token: {}", token).as_str()))
        }
    }
}
impl Parser for Branch {
    fn consume(token: &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Branch, token.get_location());
        let Some(token) = iter.next() else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"));
        };
        if !matches!(token, Token::LParen(..)) {
            return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()))
        }
        let (condition, ret) = Expression::consume(token, iter)?;
        Branch::expect_brace_close(&ret)?;
        tree.add(condition);

        let (body, ret) = Expression::consume(token, iter)?;
        tree.add(body);
        
        let Some(next_token) = ret else {
            return Ok((tree, None));
        };

        match next_token {
            Token::Else(..) => {
                let (else_body,ret3) = Expression::consume(token, iter)?;
                tree.add(else_body);
                Ok((tree, ret3))
            },
            _ => {
                tree.add(Node::new(Op::Paren, next_token.get_location()));
                Ok((tree, Some(next_token)))
            },
        }
    }
}

