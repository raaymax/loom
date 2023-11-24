use std::slice::Iter;
use lexer::{Location,Token, PError};
use super::{Node, Expression, Op};


pub struct Loop;

macro_rules! accept {
    ( $iter:ident, $token:ident ) => {
        let Some(token) = $iter.next() else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"));
        };
        if !matches!(token, Token::$token(..)) {
            return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()))
        }
    };
}
macro_rules! expect {
    ( $ret:ident, $token:ident ) => {
        let Some(token) = $ret else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"))
        };
        if !matches!(token, Token::$token(..)) {
            return Err(PError::new(token.get_location(),format!("Unexpected token: {}", token).as_str()))
        }
    };
}

impl Loop {
    pub fn consume(token: &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::While, token.get_location());
        accept!(iter, LParen);
        let (condition, ret) = Expression::consume(token, iter, level+1)?;
        expect!(ret, RParen);
        tree.add(condition);
        let (body, ret) = Expression::consume(token, iter, level+1)?;
        tree.add(body);
        Ok((tree, ret))
    }
}

