mod errors;
mod token;
mod tokenizer;
mod parser;
mod loc;
mod iter;


use token::{Token, TokenVec};
use tokenizer::Tokenizer;
use errors::PError;

fn tokenize(text: &str) -> Result<Vec<Token>, PError> {
    Tokenizer::new(text).collect()
}

fn main() {
    let text = if let Some(path) = std::env::args().nth(1) {
        std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
    } else {
        let name = std::env::args().next().unwrap();
        eprintln!("Usage: {} <filename>", name);
        std::process::exit(1);
    };

    println!("\nINPUT:\t{}", text);
    let mut tokens = tokenize(&text).unwrap_or_else(|e| {
        eprintln!("Error: {}", e.format_error(&text));
        std::process::exit(1);
    });
    println!("\nTOKENS:\t{}", TokenVec(&tokens));
    let node = parser::parse(&mut tokens).unwrap_or_else(|e| {
        eprintln!("Error: {}", e.format_error(&text));
        std::process::exit(1);
    });
    println!("\nTREE:\t{}", node);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<parser::Node, PError> {
        let mut tokens = tokenize(text)?;
        parser::parse(&mut tokens)
    }

    #[macro_export]
    macro_rules! test_parser {
        ( $name:ident, $i:expr, $o:expr ) => {
            #[test]
            fn $name() {
                let node = parse($i).unwrap();
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
                assert_eq!(err.format_error(text), $o);
            }
        };
    }


    test_parser!(variable_basic, "variable", "variable");
    test_parser!(variable_camel_case, "camel_case", "camel_case");
    test_parser!(variable_with_numbers, "var123", "var123");

    test_parser!(number_dec_8, "8", "8");
    test_parser!(number_dec_20, "20", "20");
    test_parser_error!(number_dec_invalid_characters, 
                 "123asd", 
                 "123asd\n   ^^^\nInvalid number");

    test_parser!(number_oct_8, "010", "8");
    test_parser!(number_oct_20, "024", "20");
    test_parser_error!(number_oct_with_hex_error, 
                 "012a", 
                 "012a\n   ^\nInvalid number");
    test_parser_error!(number_oct_with_dec_error, 
                 "0129", 
                 "0129\n   ^\nInvalid number");

    test_parser!(number_hex_8, "0x8", "8");
    test_parser!(number_hex_20, "0x14", "20");
    test_parser_error!(number_hex_wrong, 
                 "01x23 adw\nasd", 
                 "01x23 adw\n  ^^     \nInvalid number");


    test_parser!(expr_sumple, 
                 "1 + 2", 
                 "(1 + 2)");

    test_parser!(expr_variables, 
                 "num1 + num2", 
                 "(num1 + num2)");
    test_parser!(expr_complex, 
                 "0x123 / ( 2 + 123 ) + 23 * 010 * ( num - 4 )", 
                 "((291 / (2 + 123)) + ((23 * 8) * (num - 4)))");
    test_parser!(expr_complex_no_spaces, 
                 "0x123/(2+123)+23*010*(num-4)", 
                 "((291 / (2 + 123)) + ((23 * 8) * (num - 4)))");
}
