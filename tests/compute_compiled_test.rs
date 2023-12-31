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
            println!("{}", node);
            let bytes = compiler::Compiler::new().compile(&node).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            println!("{:?}", bytes);
            let mut vm = vm::VM::new(bytes);
            let val = vm.run().unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            assert_eq!(val, $o);
        }
    };
}

test_return_code!(vm_return_1, "1", 1);
test_return_code!(vm_compute_add, "1+2", 3);
test_return_code!(vm_compute_add_2, "1+2+2", 5);
test_return_code!(vm_compute_sub, "4-2", 2);
test_return_code!(vm_compute_complex, "(1+5)-(2+2)", 2);
test_return_code!(vm_compute_if0, "if(0){2}else{3}", 3);
test_return_code!(vm_compute_if1, "if(1){2}else{3}", 2);
test_return_code!(vm_compute_if2, "if(2){2}else{3}", 2);
test_return_code!(vm_compute_assign, "i=7", 7);
//test_return_code!(vm_compute_while, "i=0; while(i<2){i = i +1; i}", 2);
//test_return_code!(vm_compute_sub_overflow, "2-4", (-2i32) as u32);
test_return_code!(vm_compute_add_and_sub, "1+2-2", 1);
//test_return_code!(vm_compute_branch, "if(1){2}else{3}", 2);
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
