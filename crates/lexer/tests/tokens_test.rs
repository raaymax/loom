use lexer::{Token,Tokenizer, PError};

#[macro_export]
macro_rules! test_token_positions{
    ( $name:ident, $i:expr, [$($o:expr),+] ) => {
        #[test]
        fn $name() {
            let Ok(tokens) = Tokenizer::new($i).collect::<Result<Vec<Token>, PError>>() else {
                panic!("Error");
            };
            let locations = tokens.iter().map(|t| t.get_location().pos()).collect::<Vec<usize>>();
            assert_eq!(locations, vec![$($o),+]);
        }
    };
}

mod expressions {
    use super::*;
    test_token_positions!(basic_positions, 
                 "0x123 / ( 2 + 123 ) + 23 * 010 * ( num - 4 )", 
                [0, 6, 8, 10, 12, 14, 18, 20, 22, 25, 27, 31, 33, 35, 39, 41, 43, usize::MAX]);
    test_token_positions!(while_token_positions, 
                 "while(a==5){5}", 
                [0, 5, 6, 7, 9, 10, 11, 12, 13, usize::MAX]);
}
