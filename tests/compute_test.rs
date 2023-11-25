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
            let value = interpreter::interpret(node).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            assert_eq!(value, $o.into());
        }
    };
}

#[macro_export]
macro_rules! test_compute_error{
    ( $name:ident, $i:expr, $o:expr ) => {
        #[test]
        fn $name() {
            let text = $i;
            let node = parse(text).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            let err = interpreter::interpret(node).unwrap_err();
            assert_eq!(err.format_error(text, "file.lum", false).to_string(), $o);
        }
    };
}

test_compute!(simple, "1 + 2", 3);
test_compute!(with_braces, "2 * (3 + 4) ", 14);
test_compute!(with_variable, "asd = 4; 2 * (3 + asd) ", 14);
test_compute!(with_two_variables, "qwe=3; asd = 4; 2 * (qwe + asd) ", 14);
test_compute!(conditional_positive, "if(1){5}else{7}", 5);
test_compute!(conditional_negative, "if(0){5}else{7}", 7);
test_compute!(strings_multiplication, "qwe='oko'; qwe*3", "okookooko");
test_compute!(while_loop, "a=0; while(a!=2) a = a + 1; a", 2);
//test_compute!(goal, "fn qwe(x) x*2; asd = 4; 2 * (qwe(5) + asd) ", 28);
//test_compute_error!(err_goal, "123 + 'text'", "123 + 'text'\n    ^       \nIncompatible types");
