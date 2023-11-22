use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Expression, Node};

pub struct Call;

impl Call{
    pub fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Call, token.get_location());
        let mut params = Node::new(Op::Paren, token.get_location());
        loop {
            let (node, tok) = Expression::consume(token, iter, level + 1)?;
            params.add(node);

            let Some(t) = tok else {
                return Ok((tree,None));
            };
            match t{
                Token::RParen(..) | Token::Eof => {
                    tree.add(params);
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
