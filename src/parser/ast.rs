use std::slice::Iter;
use crate::loc::Location;
use crate::token::Token;
use crate::errors::PError;

use super::ast_node::{Node, Op};

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
                        tree.add(Node::new_value((*n).into(), *loc));
                        Ok(Self::Operator)
                    },
                    Token::String(loc, n) => {
                        tree.add(Node::new_value(n.into(), *loc));
                        Ok(Self::Operator)
                    },
                    Token::Id(loc, ref n) => {
                        tree.add(Node::new_var(n, *loc));
                        Ok(Self::Operator)
                    },
                    Token::LParen(loc) => {
                        let ret = build(iter, level+1, *loc)?;
                        tree.add(ret);
                        Ok(Self::Operator)
                    },
                    Token::Eof => Err(PError::new(token.get_location(), "Unexpected end of file")),
                    _ => Err(PError::new(token.get_location(), "Invalid expression, expected ID or Number")),
                }
            },
            Self::Operator => {
                match token {
                    Token::Eq(loc) => {
                        tree.add(Node::new_op(Op::Assign, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Plus(loc) => {
                        tree.add(Node::new_op(Op::Add, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Minus(loc) => {
                        tree.add(Node::new_op(Op::Sub, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Star(loc) => {
                        tree.add(Node::new_op(Op::Mul, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Slash(loc) => {
                        tree.add(Node::new_op(Op::Div, *loc));
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



pub fn build(iter: &mut Iter<Token>, level: usize, loc: Location ) -> Result<Node, PError> {
    let mut tree: Node = Node::new_scope(loc);
    let mut state = State::Expr;
    loop {
        state = state.expect(&mut tree, iter, level)?;
        match state {
            State::End => {
                return Ok(tree);
            },
            State::Eof => {
                return Ok(tree);
            },
            _ => {}
        }
        if let State::End = state {
            return Ok(tree);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let tokens = vec![Token::Number(Location::zero(), 1)];
        let tree = build(&mut tokens.iter(), 0, Location::zero()).unwrap();
        assert_eq!(tree.to_string(), "{1}");
    }
    #[test]
    fn test_operator_plus() {
        let tokens = vec![Token::Number(Location::zero(), 2), Token::Plus(Location::zero()), Token::Number(Location::zero(), 1), Token::Semi(Location::zero())];
        let tree = build(&mut tokens.iter(), 0, Location::zero()).unwrap();
        assert_eq!(tree.to_string(), "{(2 + 1)}");
    }

    #[test]
    fn test_operator_assign() {
        let tokens = vec![Token::Id(Location::zero(), "variable".to_string()), Token::Eq(Location::zero()), Token::Number(Location::zero(), 1), Token::Semi(Location::zero())];
        let tree = build(&mut tokens.iter(), 0, Location::zero()).unwrap();
        assert_eq!(tree.to_string(), "{(variable = 1)}");
    }
    
}
