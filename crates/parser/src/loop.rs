use std::slice::Iter;
use lexer::{Location,Token, PError};
use super::{Node, Expression, Op};
use crate::Parser;
use crate::{expect, accept};

pub struct Loop;

impl Parser for Loop {
    fn consume(token: &Token, iter: &mut Iter<Token>) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::While, token.get_location());
        accept!(iter, LParen);
        let (condition, ret) = Expression::consume(token, iter)?;
        expect!(ret, RParen);
        tree.add(condition);
        let (body, ret) = Expression::consume(token, iter)?;
        tree.add(body);
        Ok((tree, ret))
    }
}

