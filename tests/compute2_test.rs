use lexer::{Token,PError,Tokenizer};

fn parse(text: &str) -> Result<parser::Node, PError> {
    let tokens:Vec<Token> = Tokenizer::new(text).collect::<Result<Vec<Token>, PError>>()?;
    let mut iter = tokens.iter();
    parser::parse(&mut iter)
}

#[macro_export]
macro_rules! test_compute{
    ( $name:ident, $i:expr, $o:expr ) => {
        #[test]
        fn $name() {
            let text = $i;
            let node = parse(text).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            //println!("{}", node);
            let value = interpreter2::interpret(node).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e)
            });
            assert_eq!(value, $o);
        }
    };
}


test_compute!(simple, "1 + 2", 3);
test_compute!(simple2, "1 + 2 * 5 - 2 / 1", 14);
test_compute!(with_braces, "2 * (3 + 4) ", 14);
test_compute!(with_variable, "let asd = 4; 2 * (3 + asd) ", 14);
test_compute!(with_two_variables, "let qwe=3; let asd = 4; 2 * (qwe + asd) ", 14);
test_compute!(conditional_positive, "if(1){5}else{7}", 5);
test_compute!(conditional_negative, "if(0){5}else{7}", 7);
//test_compute!(strings_multiplication, "let qwe='oko'; qwe*3", "okookooko");
test_compute!(while_loop, "let a=0; while(a!=2) a = a + 1; a", 2);
test_compute!(while_loop_return, "let a=0; while(a<5) if (a == 3) { return a } else { a = a + 1; a}", 3);
test_compute!(functions_and_variables, "fn qwe(x) x*2; let asd = 4; 2 * (qwe(5) + asd)", 28);
test_compute!(recursive_functions, "fn fact(x) if (x <= 1) { 1 } else { x * fact(x - 1) }; fact(5)", 120);
test_compute!(lambda_functions, "let qwe = fn(x) x*2; qwe(14)", 28);
//test_compute!(goal_recursive_fib, "fn fib(x) if (x == 1 || x == 2) { 1 } else { fib(x - 1) + fib(x - 2) }; fib(10)", 55);
//test_compute_error!(err_goal, "123 + 'text'", "123 + 'text'\n    ^       \nIncompatible types");

