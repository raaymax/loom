use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Expression, Node};

pub struct Block;

impl Block {
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Scope, token.get_location());
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
