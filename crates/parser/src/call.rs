use std::slice::Iter;
use lexer::{Token, PError};
use super::Op;
use super::expr::Expression;
use super::ast_node::Node;

pub struct Call;

impl Call{
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Call, token.get_location());
        loop {
            let (node, tok) = Expression::consume(token, iter, level + 1)?;
            tree.add(node);

            let Some(t) = tok else {
                return Ok((tree,None));
            };
            match t{
                Token::RParen(..) | Token::Eof => {
                    return Ok((tree, Some(t)));
                },
                Token::Comma(..) => {
                    continue;
                },
                _ => Err(PError::new(t.get_location(), format!("Unexpected token, missed semicolon?").as_str()))?,
            }
        }
    }
}
