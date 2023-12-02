use lexer::{Token,PError,Tokenizer};

fn parse(text: &str) -> Result<parser::Node, PError> {
    let tokens:Vec<Token> = Tokenizer::new(text).collect::<Result<Vec<Token>, PError>>()?;
    let mut iter = tokens.iter();
    parser::parse(&mut iter)
}

#[macro_export]
macro_rules! test_return_code{
    ( $name:ident, $i:expr, $o:expr ) => {
        #[test]
        fn $name() {
            let text = $i;
            let node = parse(text).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            //println!("{}", node);
            let bytes = compiler::compile(&node).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });

            let mut vm = vm::VM::new(bytes);
            let val = vm.run().unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            assert_eq!(val, $o);
        }
    };
}

//test_return_code!(simple, "1", 1);
/*
test_return_code!(simple, "1 + 2", 3);
test_return_code!(with_braces, "2 * (3 + 4) ", 14);
test_return_code!(with_variable, "asd = 4; 2 * (3 + asd) ", 14);
test_return_code!(with_two_variables, "qwe=3; asd = 4; 2 * (qwe + asd) ", 14);
test_return_code!(conditional_positive, "if(1){5}else{7}", 5);
test_return_code!(conditional_negative, "if(0){5}else{7}", 7);
test_return_code!(strings_multiplication, "qwe='oko'; qwe*3; 7", 7);
test_return_code!(while_loop, "a=0; while(a!=2) a = a + 1; a", 2);
*/
