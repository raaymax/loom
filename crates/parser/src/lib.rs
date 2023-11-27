mod value;
mod node;
mod block;
mod expr;
mod branch;
mod call;
mod r#loop;
mod args;
mod func;
mod params;

use std::slice::Iter;

use lexer::{Token, PError};

pub use value::Value;
pub use self::node::{Node, Op};


pub use block::Block;
pub use expr::Expression;
pub use branch::Branch;
pub use call::Call;
pub use r#loop::Loop;
pub use args::Args;
pub use func::Func;
pub use params::Params;


pub fn parse(iter: &mut Iter<Token> ) -> Result<Node, PError> {
    let (node, ret) = Block::consume(&Token::Start, iter, 0)?;
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
    test_ast_expr!(ast_op_eq, "{(1 == 2)}", [Number(1), Eq(), Number(2)]);
    test_ast_expr!(ast_op_eq_var, "{(a == 2)}", [LParen(), Id("a".to_string()), Eq(), Number(2), RParen()]);
    test_ast_expr!(ast_op_neq, "{(1 != 2)}", [Number(1), Neq(), Number(2)]);
    test_ast_expr!(ast_op_mod, "{(1 % 2)}", [Number(1), Mod(), Number(2)]);
    test_ast_expr!(ast_op_assign, "{(id = 2)}", [Id("id".to_string()), Assign(), Number(2)]);
    test_ast_expr!(ast_nesting, "{((1 + 2) + 3)}", [Number(1), Plus(), Number(2), Plus(), Number(3)]);
    test_ast_expr!(ast_nesting_mul, "{(1 + (2 * 3))}", [Number(1), Plus(), Number(2), Star(), Number(3)]);
    test_ast_expr!(ast_nesting_mul_2x, "{((1 * 2) * 3)}", [Number(1), Star(), Number(2), Star(), Number(3)]);
    test_ast_expr!(ast_complex, "{((123 / (2 + 123)) + ((23 * 8) * (32 - 4)))}", [
        Number(123), Slash(), LParen(), Number(2), Plus(), Number(123), RParen(), Plus(), LParen(),
        Number(23), Star(), Number(8), RParen(), Star(), LParen(), Number(32), Minus(), Number(4), RParen()
    ]);
    test_ast_expr!(ast_multiple_expressions, "{(a = 2);(a + 5)}", [
        Id("a".to_string()), Assign(),Number(2), Semi(), Id("a".to_string()), Plus(), Number(5)
    ]);
    test_ast_expr!(ast_assign_block_to_var, "{(asd = {(zxc = 123);5})}", [
        Id("asd".to_string()), Assign(), LBrace(), Id("zxc".to_string()), Assign(), Number(123), Semi(), Number(5), RBrace()
    ]);
    test_ast_expr!(ast_simple_conditional, "{if(5){(asd = 6)} else {5}}", [
        If(), LParen(), Number(5), RParen(), LBrace(), Id("asd".to_string()), Assign(), Number(6), RBrace(), Else(), LBrace(), Number(5), RBrace()
    ]);
    test_ast_expr!(ast_complex_conditional, "{(zxc = {if(5){(asd = 6)} else {5}})}", [
        Id("zxc".to_string()), Assign(), LBrace(), 
        If(), LParen(), Number(5), RParen(),
        LBrace(), Id("asd".to_string()), Assign(), Number(6), RBrace(), 
        Else(), LBrace(), Number(5), RBrace(),
        RBrace()
    ]);
    test_ast_expr!(ast_nested_scopes, "{(zxc = {{{5}}})}", [
        Id("zxc".to_string()), Assign(), LBrace(), LBrace(),
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

