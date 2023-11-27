use std::slice::Iter;
use lexer::{Token, PError};
use super::{Op, Node, Args};
use super::Parser;

pub struct Call;

impl Parser for Call{
    fn consume(token:  &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Call, token.get_location());
        let (params, tok) = Args::consume(token, iter)?;
        let Some(t) = tok else {
            return Ok((tree,None));
        };
        match t{
            Token::RParen(..) => {
                tree.add(params);
                return Ok((tree, Some(t)));
            },
            _ => Err(PError::new(t.get_location(), format!("Unexpected token, missed semicolon?").as_str()))?,
        }
    }
}
