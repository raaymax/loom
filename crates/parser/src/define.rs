use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Expression, Node};
use lexer::Location;
use crate::Parser;
use crate::{expect, accept};

pub struct Define;

impl Parser for Define {
    fn consume(token:  &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let name = accept!(iter, Id);
        let Token::Id(_loc, id) = name else {
            panic!("Unexpected token");
        };
    
        let mut tree = Node::new(Op::DefineVar, token.get_location()).set_id(id.clone());

        let Some(tok) = iter.next() else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"));
        };

        match tok {
            Token::Assign(_loc) => {
                let mut assign = Node::new(Op::Assign, name.get_location());
                assign.add(Node::new(Op::Variable, name.get_location()).set_id(id.to_string()));
                let (exp, tok) = Expression::consume(token, iter)?;
                expect!(tok.clone(), Semi);
                assign.add(exp);
                tree.add(assign);
                Ok((tree, tok))
            },
            Token::Semi(_loc) => {
                Ok((tree, Some(tok.clone())))
            },
            _ => {
                Err(PError::new(token.get_location(), "Unexpected token"))
            }
        }
    }
}
