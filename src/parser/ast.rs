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
                        let ret = build_paren(iter, level+1, *loc)?;
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
                        tree.next(Node::new_placeholder());
                        Ok(Self::Expr)
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


pub fn build_paren(iter: &mut Iter<Token>, level: usize, location: Location) -> Result<Node, PError> {
    let mut tree: Node = Node::new_paren(location);
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

    #[macro_export]
    macro_rules! test_ast{
        ( $name:ident, $i:expr, $o:expr ) => {
            #[test]
            fn $name() {
                let tokens = $i;
                let tree = build(&mut tokens.iter(), 0, Location::zero()).unwrap();
                assert_eq!(tree.to_string(), $o);
            }
        };
    }

    #[macro_export]
    macro_rules! test_ast2{
        ( $name:ident, $o:expr, [ $($token:ident($($arg:expr),*)),+ ] ) => {
            #[test]
            fn $name() {
                let tokens = vec![$(Token::$token(Location::zero(), $($arg),*)),+];
                let tree = build(&mut tokens.iter(), 0, Location::zero()).unwrap();
                assert_eq!(tree.to_string(), $o);
            }
        };
    }

    test_ast2!(ast_number, "{1}", [Number(1)]);
    test_ast2!(ast_op_add, "{(1 + 2)}", [Number(1), Plus(), Number(2)]);
    test_ast2!(ast_op_sub, "{(1 - 2)}", [Number(1), Minus(), Number(2)]);
    test_ast2!(ast_op_mul, "{(1 * 2)}", [Number(1), Star(), Number(2)]);
    test_ast2!(ast_op_div, "{(1 / 2)}", [Number(1), Slash(), Number(2)]);
    test_ast2!(ast_op_assign, "{(id = 2)}", [Id("id".to_string()), Eq(), Number(2)]);
    test_ast2!(ast_nesting, "{((1 + 2) + 3)}", [Number(1), Plus(), Number(2), Plus(), Number(3)]);
    test_ast2!(ast_nesting_mul, "{(1 + (2 * 3))}", [Number(1), Plus(), Number(2), Star(), Number(3)]);
    test_ast2!(ast_nesting_mul_2x, "{((1 * 2) * 3)}", [Number(1), Star(), Number(2), Star(), Number(3)]);
    test_ast2!(ast_complex, "{((123 / (2 + 123)) + ((23 * 8) * (32 - 4)))}", [
        Number(123), Slash(), LParen(), Number(2), Plus(), Number(123), RParen(), Plus(), LParen(),
        Number(23), Star(), Number(8), RParen(), Star(), LParen(), Number(32), Minus(), Number(4), RParen()
    ]);
    test_ast2!(ast_multiple_expressions, "{(a = 2);(a + 5)}", [
        Id("a".to_string()), Eq(),Number(2), Semi(), Id("a".to_string()), Plus(), Number(5)
    ]);
}
