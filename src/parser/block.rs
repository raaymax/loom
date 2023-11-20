use std::slice::Iter;
use crate::token::Token;
use crate::errors::PError;
use super::expr::Expression;
use super::ast_node::Node;

pub struct Block;

impl Block {
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new_scope(token.get_location());
        loop {
            let (node, tok) = Expression::consume(token, iter, level + 1)?;
            tree.add(node);

            let Some(t) = tok else {
                return Ok((tree,None));
            };
            match t{
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
