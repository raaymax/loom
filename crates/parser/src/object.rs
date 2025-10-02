use std::slice::Iter;
use lexer::{Location, PError, Token};
use super::{Op, Expression, Node};
use super::Parser;

pub struct Object;
use crate::{accept};

impl Parser for Object {
    fn consume(token:  &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Object, token.get_location());
        loop {
            let Token::Id(loc, id) = &token else {
                return Err(PError::new(token.get_location(), "Expected identifier for object name"));
            };
            let mut def = Node::new(Op::DefineVar, *loc).set_id(id.clone());
            accept!(iter, Colon);
            let mut assign = Node::new(Op::Assign, token.get_location());
            assign.add(Node::new(Op::Variable, token.get_location()).set_id(id.clone()));
            let (exp, tok) = Expression::consume(token, iter)?;

            assign.add(exp);
            def.add(assign);
            tree.add(def);

            let Some(t) = tok else {
                return Err(PError::new(Location::Eof, format!("Unexpected EOF").as_str()))
            };
            match t {
                Token::RBrace(..) => {
                    return Ok((tree, Some(t)));
                },
                Token::Comma(..) => {
                    continue;
                },
                _ => Err(PError::new(t.get_location(), format!("Unexpected token, missed comma?").as_str()))?,
            }
        }
    }
}

impl Object {
    pub fn detect(token: &Token, iter: &mut Iter<Token>) -> bool {
        let Token::Id(_, _) = &token else {
            return false;
        };
        let token = iter.next();
        let Some(token) = token else {
            return false;
        };
        if !matches!(token, Token::Colon(..)) {
            return false;
        }
        return true;
    }
}

