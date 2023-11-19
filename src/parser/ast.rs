use std::slice::Iter;
use crate::loc::Location;
use crate::token::Token;
use crate::errors::PError;

use super::ast_node::{Node, Op};

struct Block;

impl Block {
    fn consume(token:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new_scope(token.get_location());
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

enum ExpressionState {
    Start,
    Noun,
    Op,
}

struct Expression;

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


    fn consume(tok:  &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
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

struct Branch;

impl Branch {
    fn expect_brace_close(ret: &Option<Token>) -> Result<(), PError> {
        let Some(token) = ret else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"))
        };
        if matches!(token, Token::RParen(..)) {
            Ok(())
        } else {
            Err(PError::new(token.get_location(),format!("Unexpected token: {}", token).as_str()))
        }
    }
    fn consume(token: &Token, iter: &mut Iter<Token>, level: usize) -> Result<(Node, Option<Token>), PError> {
        let mut tree = Node::new_branch(token.get_location());
        let Some(token) = iter.next() else {
            return Err(PError::new(Location::Eof, "Unexpected end of file"));
        };
        if !matches!(token, Token::LParen(..)) {
            return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()))
        }
        let (condition, ret) = Expression::consume(token, iter, level+1)?;
        Branch::expect_brace_close(&ret)?;
        tree.add(condition);

        let (body, ret) = Expression::consume(token, iter, level+1)?;
        tree.add(body);
        
        let Some(next_token) = ret else {
            return Ok((tree, None));
        };

        match next_token {
            Token::Else(..) => {
                let (else_body,ret3) = Expression::consume(token, iter, level+1)?;
                tree.add(else_body);
                Ok((tree, ret3))
            },
            _ => Ok((tree, Some(next_token))),
        }
    }
}

pub fn build(iter: &mut Iter<Token>, level: usize, loc: Location ) -> Result<Node, PError> {
    let (node, ret) = Block::consume(&Token::Start, iter, level)?;
    if let Some(token) = ret {
        if let Token::Eof = token {
            return Ok(node);
        }
        return Err(PError::new(token.get_location(), format!("Unexpected token: {}", token).as_str()));
    }
    Ok(node)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! test_ast_expr{
        ( $name:ident, $o:expr, [ $($token:ident($($arg:expr),*)),+ ] ) => {
            #[test]
            fn $name() {
                let tokens = vec![$(Token::$token(Location::zero(), $($arg),*)),+];
                let mut iter = tokens.iter();
                let tok = Token::Start;
                let (scope, _) = Block::consume(&tok, &mut iter, 0).unwrap();
                assert_eq!(scope.to_string(), $o);
            }
        };
    }
    #[macro_export]
    macro_rules! tokens{
        [ $($token:ident($($arg:expr),*)),+ ] => {
            vec![$(Token::$token(Location::zero(), $($arg),*)),+]
        };
    }

    test_ast_expr!(ast_number, "{1}", [Number(1)]);
    test_ast_expr!(ast_op_add, "{(1 + 2)}", [Number(1), Plus(), Number(2)]);
    test_ast_expr!(ast_op_sub, "{(1 - 2)}", [Number(1), Minus(), Number(2)]);
    test_ast_expr!(ast_op_mul, "{(1 * 2)}", [Number(1), Star(), Number(2)]);
    test_ast_expr!(ast_op_div, "{(1 / 2)}", [Number(1), Slash(), Number(2)]);
    test_ast_expr!(ast_op_assign, "{(id = 2)}", [Id("id".to_string()), Eq(), Number(2)]);
    test_ast_expr!(ast_nesting, "{((1 + 2) + 3)}", [Number(1), Plus(), Number(2), Plus(), Number(3)]);
    test_ast_expr!(ast_nesting_mul, "{(1 + (2 * 3))}", [Number(1), Plus(), Number(2), Star(), Number(3)]);
    test_ast_expr!(ast_nesting_mul_2x, "{((1 * 2) * 3)}", [Number(1), Star(), Number(2), Star(), Number(3)]);
    test_ast_expr!(ast_complex, "{((123 / (2 + 123)) + ((23 * 8) * (32 - 4)))}", [
        Number(123), Slash(), LParen(), Number(2), Plus(), Number(123), RParen(), Plus(), LParen(),
        Number(23), Star(), Number(8), RParen(), Star(), LParen(), Number(32), Minus(), Number(4), RParen()
    ]);
    test_ast_expr!(ast_multiple_expressions, "{(a = 2);(a + 5)}", [
        Id("a".to_string()), Eq(),Number(2), Semi(), Id("a".to_string()), Plus(), Number(5)
    ]);
    test_ast_expr!(ast_assign_block_to_var, "{(asd = {(zxc = 123);5})}", [
        Id("asd".to_string()), Eq(), LBrace(), Id("zxc".to_string()), Eq(), Number(123), Semi(), Number(5), RBrace()
    ]);
    test_ast_expr!(ast_simple_conditional, "{if(5){(asd = 6)} else {5}}", [
        If(), LParen(), Number(5), RParen(), LBrace(), Id("asd".to_string()), Eq(), Number(6), RBrace(), Else(), LBrace(), Number(5), RBrace()
    ]);
    test_ast_expr!(ast_complex_conditional, "{(zxc = {if(5){(asd = 6)} else {5}})}", [
        Id("zxc".to_string()), Eq(), LBrace(), 
        If(), LParen(), Number(5), RParen(),
        LBrace(), Id("asd".to_string()), Eq(), Number(6), RBrace(), 
        Else(), LBrace(), Number(5), RBrace(),
        RBrace()
    ]);
    test_ast_expr!(ast_nested_scopes, "{(zxc = {{{5}}})}", [
        Id("zxc".to_string()), Eq(), LBrace(), LBrace(),
        LBrace(), Number(5), RBrace(),
        RBrace(), RBrace()
    ]);
    test_ast_expr!(ast_scopes_in_line, "{{()};{()};{()}}", [
        LBrace(), RBrace(), Semi(),
        LBrace(), RBrace(), Semi(),
        LBrace(), RBrace()
    ]);
    #[test]
    fn test_return_of_expr() {
        let vec = tokens![Number(6), RBrace()];
        let mut iter = vec.iter();
        let start = Token::Start;
        let (_, Some(Token::RBrace(..))) = Expression::consume(&start, &mut iter, 0).unwrap() else {
            panic!("Expected return value");
        };
    }
    #[test]
    fn test_return_of_branch() {
        let vec = tokens![LParen(), Number(5), RParen(), LBrace(), Number(5), RBrace(), Else(), LBrace(), Number(6), RBrace(), RBrace()];
        let start = Token::Start;
        let mut iter = vec.iter();
        let (_, Some(Token::RBrace(..))) = Branch::consume(&start, &mut iter, 0).unwrap() else {
            panic!("Expected return value");
        };
    }
}

