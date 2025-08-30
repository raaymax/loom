use lexer::{Token,Tokenizer, PError};

fn parse(text: &str) -> Result<parser::Node, PError> {
    let tokens:Vec<Token> = Tokenizer::new(text).collect::<Result<Vec<Token>, PError>>()?;
    let mut iter = tokens.iter();
    parser::parse(&mut iter)
}

#[macro_export]
macro_rules! test_parser {
    ( $name:ident, $i:expr, $o:expr ) => {
        #[test]
        fn $name() {
            let node = parse($i).unwrap_or_else(|e| {
                panic!("\nError:\n{}\n", e.format_error($i, "file.lum", false));
            });
            assert_eq!(node.to_string(), $o);
        }
    };
}
#[macro_export]
macro_rules! test_parser_error {
    ( $name:ident, $i:expr, $o:expr ) => {
        #[test]
        fn $name() {
            let text = $i;
            let err = parse(text).unwrap_err();
            assert_eq!(err.format_error(text, "file.lum", false).to_string(), $o);
        }
    };
}

mod expressions {
    use super::*;
    test_parser!(variable_basic, "variable", "{variable}");
    test_parser!(variable_camel_case, "camel_case", "{camel_case}");
    test_parser!(variable_with_numbers, "var123", "{var123}");

    test_parser!(number_dec_8, "8", "{8}");
    test_parser!(number_dec_20, "20", "{20}");
    test_parser_error!(number_dec_invalid_characters, 
                 "123asd", 
                 "error: Invalid number\n --> file.lum:1:3\t|\n\t| 123asd\n\t|    ^^^\n");

    test_parser!(number_oct_8, "010", "{8}");
    test_parser!(number_oct_20, "024", "{20}");
    test_parser_error!(number_oct_with_hex_error, 
                 "012a", 
                 "error: Invalid number\n --> file.lum:1:3\t|\n\t| 012a\n\t|    ^\n");
    test_parser_error!(err_number_oct_with_dec_error, 
                 "0129", 
                 "error: Invalid number\n --> file.lum:1:3\t|\n\t| 0129\n\t|    ^\n");

    test_parser!(number_hex_8, "0x8", "{8}");
    test_parser!(number_hex_20, "0x14", "{20}");
    test_parser_error!(err_number_hex_wrong, 
                 "01x23 adw\nasd", 
                 "error: Invalid number\n --> file.lum:1:2\t|\n\t| 01x23 adw\n\t|   ^^     \n");

    test_parser!(expr_number_in_braces, 
                 "(1)", 
                 "{1}");

    test_parser!(expr_simple, 
                 "1 + 2", 
                 "{(1 + 2)}");

    test_parser!(expr_leq, 
                 "1 <= 2",
                 "{(1 <= 2)}");
    test_parser!(expr_and, 
                 "1 && 2",
                 "{(1 && 2)}");
    test_parser!(expr_or, 
                 "1 || 2",
                 "{(1 || 2)}");
    test_parser!(expr_lt, 
                 "1 < 2",
                 "{(1 < 2)}");
    test_parser!(expr_geq,
                 "1 >= 2",
                 "{(1 >= 2)}");
    test_parser!(expr_gt,
                 "1 > 2",
                 "{(1 > 2)}");
    test_parser!(expr_return,
                 "return 1",
                 "{(return 1)}");
    test_parser!(expr_function_with_return,
                 "fn a() {return 1}",
                 "{fn a() {(return 1)}}");

    test_parser!(expr_variables, 
                 "num1 + num2", 
                 "{(num1 + num2)}");
    test_parser!(expr_complex_logical, 
                 "1 < 2 && 1 + 4 > 4 || 234 >= 234", 
                 "{(((1 < 2) && ((1 + 4) > 4)) || (234 >= 234))}");
    test_parser!(expr_complex, 
                 "0x123 / ( 2 + 123 ) + 23 * 010 * ( num - 4 )", 
                 "{((291 / (2 + 123)) + ((23 * 8) * (num - 4)))}");
    test_parser!(expr_complex_no_spaces, 
                 "0x123/(2+123)+23*010*(num-4)", 
                 "{((291 / (2 + 123)) + ((23 * 8) * (num - 4)))}");
    test_parser!(expr_complex_with_conditional, 
                 "0x123 / ( 2 + 123 ) + { asd = 123; if(1) {asd + 2} } * 010 * ( num - 4 )", 
                 "{((291 / (2 + 123)) + (({(asd = 123);if(1){(asd + 2)} else ()} * 8) * (num - 4)))}");
    test_parser!(no_return_from_file,
                 "123;",
                 "{123}");
    test_parser_error!(err_expr_incomplete, 
                 "12+", 
                 "error: Unexpected end of file\n --> file.lum");
    test_parser_error!(err_expr_incomplete_with_brace, 
                 "12+(", 
                 "error: Unexpected end of file\n --> file.lum");
    test_parser_error!(err_expr_incomplete_with_brace_and_number, 
                 "12+(123", 
                 "error: Unexpected end of file, parentheses not closed [3:1]\n --> file.lum");
    test_parser_error!(err_expr_incomplete_with_brace_and_number_and_plus, 
                 "12+(123+", 
                 "error: Unexpected end of file\n --> file.lum");
    test_parser_error!(err_expr_only_operator,
                 "+", 
                 "error: Unexpected token Plus [0:1]\n --> file.lum:1:0\t|\n\t| +\n\t| ^\n");
    test_parser_error!(err_expr_missing_operator,
                 "123 321", 
                 "error: Unexpected token, missed semicolon?\n --> file.lum:1:4\t|\n\t| 123 321\n\t|     ^^^\n");
    test_parser_error!(err_expr_with_nested_braces,
                 "((((123)))", 
                 "error: Unexpected end of file, parentheses not closed [0:1]\n --> file.lum");

    test_parser!(expr_simple_call,
                 "func(1,2,3,'asd')",
                 "{func(1,2,3,'asd')}");

    test_parser!(expr_while_loop,
                 "while(1==1) { 1 + 2 }",
                 "{while ((1 == 1)) {(1 + 2)}}");

    test_parser!(expr_while_loop_complex,
                 "a=0; while(a == 2) a = a + 1; a",
                 "{(a = 0);while ((a == 2)) (a = (a + 1));a}");

    /*
    test_parser_error!(err_expr_just_braces_instead_of_operand,
                 "123 + ()", 
                 "123 + ()\n       ^\nInvalid expression, expected ID or Number");
    test_parser_error!(err_expr_just_braces,
                 "()", 
                 "()\n ^\nInvalid expression, expected ID or Number");
     */
}
