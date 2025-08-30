use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Expression, Node};
use super::Parser;

pub struct Block;

impl Parser for Block {
    fn consume(token:  &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Scope, token.get_location());
        loop {
            if let Some(Token::Eof) = iter.clone().next(){
                return Ok((tree, None));
            }
            let (node, tok) = Expression::consume(token, iter)?;
            tree.add(node);

            let Some(t) = tok else {
                return Ok((tree,None));
            };
            match t {
                Token::RBrace(..) | Token::Eof => {
                    return Ok((tree, Some(t)));
                },
                Token::Semi(..) => {
                    continue;
                },
                _ => Err(PError::new(t.get_location(), format!("Unexpected token, missed semicolon?").as_str()))?,
            }
        }
    }
}
