use std::slice::Iter;
use lexer::{Location,Token, PError};
use super::{Node, Op, Block, Branch, Call, Loop, Func};

enum ExpressionState {
    Start,
    Noun,
    Op,
}

pub struct Expression;

impl Expression {
    fn node_from(token: &Token) -> Node{
        match token {
            Token::Number(loc, n) => Node::new(Op::Value, *loc).set_value((*n).into()),
            Token::String(loc, n) => Node::new(Op::Value, *loc).set_value(n.into()),
            Token::Id(loc, id) => Node::new(Op::Variable, *loc).set_id(id.clone()),
            Token::Plus(loc) => Node::new(Op::Add, *loc),
            Token::Minus(loc) => Node::new(Op::Sub, *loc),
            Token::Star(loc) => Node::new(Op::Mul, *loc),
            Token::Slash(loc) => Node::new(Op::Div, *loc),
            Token::Mod(loc) => Node::new(Op::Mod, *loc),
            Token::Not(loc) => Node::new(Op::Not, *loc),
            Token::Neq(loc) => Node::new(Op::Neq, *loc),
            Token::Eq(loc) => Node::new(Op::Eq, *loc),
            Token::Assign(loc) => Node::new(Op::Assign, *loc),
            Token::LBrace(loc) => Node::new(Op::Scope, *loc),
            Token::LParen(loc) => Node::new(Op::Paren, *loc),
            Token::If(loc) => Node::new(Op::Branch, *loc),
            Token::Return(loc) => Node::new(Op::Return, *loc),
            Token::Lt(loc) => Node::new(Op::Lt, *loc),
            Token::Leq(loc) => Node::new(Op::Leq, *loc),
            Token::Gt(loc) => Node::new(Op::Gt, *loc),
            Token::Geq(loc) => Node::new(Op::Geq, *loc),
            _ => {
                panic!("Unexpected token to build expression: {}", token);
            }
        }
    }


    pub fn consume(tok:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new(Op::Paren, tok.get_location());
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
                            Token::While(..) => {
                                let (block, tok) = Loop::consume(token, iter, level + 1)?;
                                tree.add(block);
                                return Ok((tree, tok));
                            }
                            Token::Fn(..) => {
                                let (block, tok) = Func::consume(token, iter, level + 1)?;
                                tree.add(block);
                                return Ok((tree, tok));
                            }
                            Token::Return(..)=> {
                                let mut ret = Expression::node_from(token);
                                let (expr, tok) = Expression::consume(token, iter, level + 1)?;
                                ret.add(expr);
                                tree.add(ret);
                                return Ok((tree, tok));
                            },
                            _ => {
                                return Err(PError::new(token.get_location(), format!("Unexpected token {}", token).as_str()));
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
                    } else if let Token::LParen(..) = token {
                        let (block, tok) = Call::consume(token, iter, level + 1)?;
                        if let Some(Token::RParen(..)) = tok {
                            tree.add(block);
                            state = ExpressionState::Op;
                        } else {
                            return Err(PError::new(Location::Eof, format!("Unexpected end of file, parentheses not closed {}", token.get_location()).as_str()));
                        }
                    }else {
                        return Ok((tree,Some(token.clone())));
                    }
                }
            }
        }
        
    }

}
