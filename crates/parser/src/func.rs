use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Expression, Node, Params};
use lexer::Location;

pub struct Func;

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

impl Func{
    fn node_from(token: &Token) -> Node{
        match token {
            Token::Id(loc, id) => Node::new(Op::Var, *loc).set_id(id.clone()),
            _ => {
                panic!("Unexpected token in function builder: {}", token);
            }
        }
    }
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Func, token.get_location());
        let name = accept!(iter, Id);
        tree.add(Func::node_from(name));
        accept!(iter, LParen);
        let (params, tok) = Params::consume(token, iter, level + 1)?;
        tree.add(params);
        expect!(tok, RParen);
        let (expr, tok2) = Expression::consume(token, iter, level + 1)?;
        tree.add(expr);
        return Ok((tree, tok2));
    }
}
