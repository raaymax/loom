mod value;
mod node;
mod ast_node;
mod ast;
pub use value::Value;
use std::slice::Iter;

use crate::loc::Location;
use crate::token::Token;
use crate::errors::PError;
pub use node::{Node, Op};

enum State {
    Expr,
    Operator,
    End,
    Eof
}

impl State {
    fn expect(&self, tree:  &mut Node, iter: &mut Iter<Token>, level: usize) -> Result<State, PError> {
        let Some(token) = iter.next() else {
            return Ok(State::Eof);
        };
        match self {
            Self::Expr => {
                match token {
                    Token::Number(loc, n) => {
                        tree.add(Node::new(Op::Def, Some((*n).into()), *loc));
                        Ok(Self::Operator)
                    },
                    Token::String(loc, n) => {
                        tree.add(Node::new(Op::Def, Some(n.into()), *loc));
                        Ok(Self::Operator)
                    },
                    Token::Id(loc, ref n) => {
                        tree.add(Node::new_var(Op::Def, n, *loc));
                        Ok(Self::Operator)
                    },
                    Token::LParen(loc) => {
                        let ret = build(iter, level+1, *loc)?;
                        match ret {
                            Some(node) => {
                                tree.add(node);
                                Ok(Self::Operator)
                            },
                            None => Err(PError::new(token.get_location(), "Unexpected end of file")),
                        }
                    },
                    Token::Eof => Err(PError::new(token.get_location(), "Unexpected end of file")),
                    _ => Err(PError::new(token.get_location(), "Invalid expression, expected ID or Number")),
                }
            },
            Self::Operator => {
                match token {
                    Token::Eq(loc) => {
                        tree.add(Node::new(Op::Assign, None, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Plus(loc) => {
                        tree.add(Node::new(Op::Add, None, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Minus(loc) => {
                        tree.add(Node::new(Op::Sub, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Star(loc) => {
                        tree.add(Node::new(Op::Mul, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Slash(loc) => {
                        tree.add(Node::new(Op::Div, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::RParen(_) => {
                        Ok(Self::End)
                    },
                    Token::Semi(_) => {
                        if level > 0 {
                            return Err(PError::new(token.get_location(), "Unexpected end of expression"));
                        }
                        Ok(Self::End)
                    },
                    Token::Eof => {
                        if level > 0 {
                            return Err(PError::new(token.get_location(), "Unexpected end of file"));
                        }
                        Ok(Self::End)
                    },
                    _ => {
                        Err(PError::new(token.get_location(), "Invalid expression, expected operator or semicolon"))
                    }
                }

            },
            Self::End => Ok(Self::End),
            Self::Eof => Ok(Self::Eof)
        }
    }
}



pub fn build(iter: &mut Iter<Token>, level: usize, loc: Location ) -> Result<Option<Node>, PError> {
    let mut tree: Node = Node::new(Op::Root, None, loc);
    let mut state = State::Expr;
    loop {
        state = state.expect(&mut tree, iter, level)?;
        match state {
            State::End => {
                return Ok(Some(tree));
            },
            State::Eof => {
                return Ok(None);
            },
            _ => {}
        }
        if let State::End = state {
            return Ok(Some(tree));
        }
    }
}
pub fn parse(iter: &mut Iter<Token>) -> Result<Vec<Node>, PError> {
    let mut arr = Vec::new();
    
    loop {
        let ret = build(iter, 0, Location::new_point(0,0,0));
        match ret {
            Ok(Some(v)) => arr.push(v),
            Ok(None) => return Ok(arr),
            Err(e) => return Err(e)
        }
    }
}
