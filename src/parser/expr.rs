use std::slice::Iter;
use crate::loc::Location;
use crate::token::Token;
use crate::errors::PError;

use super::ast_node::{Node, Op};
use super::block::Block;
use super::branch::Branch;

enum ExpressionState {
    Start,
    Noun,
    Op,
}

pub struct Expression;

impl Expression {
    fn node_from(token: &Token) -> Node{
        match token {
            Token::Number(loc, n) => Node::new_value((*n).into(), *loc),
            Token::String(loc, n) => Node::new_value(n.into(), *loc),
            Token::Id(loc, id) => Node::new_var(id, *loc),
            Token::Plus(loc) => Node::new_op(Op::Add, *loc),
            Token::Minus(loc) => Node::new_op(Op::Sub, *loc),
            Token::Star(loc) => Node::new_op(Op::Mul, *loc),
            Token::Slash(loc) => Node::new_op(Op::Div, *loc),
            Token::Eq(loc) => Node::new_op(Op::Assign, *loc),
            Token::LBrace(loc) => Node::new_scope(*loc),
            Token::LParen(loc) => Node::new_paren(*loc),
            Token::If(loc) => Node::new_branch(*loc),
            _ => {
                panic!("Unexpected token to build expression: {}", token);
            }
        }
    }


    pub fn consume(tok:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new_paren(tok.get_location());
        let mut state = ExpressionState::Start;
        loop {
            let Some(token) = iter.next() else {
                return Ok((tree, None));
                //return Err(PError::new(Location::Eof, "Unexpected end of file"));
            };

            match state {
                ExpressionState::Noun | ExpressionState::Start => {
                    if token.is_noun() {
                        let node = Expression::node_from(token);
                        tree.add(node);
                        state = ExpressionState::Op;
                    } else if token.is_block() {
                        match token {
                            Token::LBrace(..) => {
                                let (block, tok) = Block::consume(token, iter, level + 1)?;
                                if let Some(Token::RBrace(..)) = tok {
                                    tree.add(block);
                                    state = ExpressionState::Op;
                                } else {
                                    
                                    return Err(PError::new(Location::Eof, format!("Unexpected end of file, brace not closed {}", token.get_location()).as_str()));
                                }
                            },
                            Token::LParen(..) => {
                                let (block, tok) = Expression::consume(token, iter, level + 1)?;
                                if let Some(Token::RParen(..)) = tok {
                                    tree.add(block);
                                    state = ExpressionState::Op;
                                } else {
                                    return Err(PError::new(Location::Eof,format!("Unexpected end of file, parentheses not closed {}", token.get_location()).as_str()));
                                }
                            },
                            Token::If(..) => {
                                let (block, tok) = Branch::consume(token, iter, level + 1)?;
                                tree.add(block);
                                return Ok((tree, tok));
                            }
                            _ => {
                                panic!("Unexpected token");
                            }
                        }
                    } else {
                        match token {
                            Token::RBrace(..) | Token::RParen(..) => {
                                return Ok((tree, Some(token.clone())));
                            },
                            Token::Eof => {
                                return Err(PError::new(token.get_location(), format!("Unexpected end of file").as_str()));
                            },
                            _ => {
                                return Err(PError::new(token.get_location(), format!("Unexpected token {}", token).as_str()));
                            }
                        }
                    }
                },
                ExpressionState::Op => {
                    if token.is_operator() {
                        let node = Expression::node_from(token);
                        tree.add(node);
                        state = ExpressionState::Noun;
                    }else {
                        return Ok((tree,Some(token.clone())));
                    }
                }
            }
        }
        
    }

}
