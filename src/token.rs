use std::fmt::Display;
use crate::loc::Location;

#[derive(Debug)]
pub enum Token {
    Number(Location, u32),
    Id(Location, String),
    Plus(Location),
    Eq(Location),
    Minus(Location),
    Star(Location),
    Slash(Location),
    LParen(Location),
    RParen(Location),
    Eof,
    Semi(Location),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(pos, n) => write!(f, "Number {} ( value: {} )", pos, n),
            Token::Id(pos, name) => write!(f, "Id {} ( name: {} )", pos, name),
            Token::Plus(pos) => write!(f, "Plus {}", pos),
            Token::Eq(pos) => write!(f, "Eq {}", pos),
            Token::Minus(pos) => write!(f, "Minus {}", pos),
            Token::Star(pos) => write!(f, "Star {}", pos),
            Token::Slash(pos) => write!(f, "Slash {}", pos),
            Token::LParen(pos) => write!(f, "LParen {}", pos),
            Token::RParen(pos) => write!(f, "RParen {}", pos),
            Token::Eof => write!(f, "EOF"),
            Token::Semi(pos) => write!(f, "Semi {}", pos),
        }
    }
}


pub struct TokenVec<'a>(pub &'a Vec<Token>);

impl Display for TokenVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from('\n');
        for token in self.0 {
            s.push_str(&format!("\t{}\n", token));
        }
        write!(f, "{}", s)
    }
}


