use std::fmt::Display;
use crate::loc::Location;

#[derive(Debug, Clone)]
pub enum Token {
    Start,
    Number(Location, u32),
    Id(Location, String),
    If(Location),
    Else(Location),
    Loop(Location),
    Break(Location),
    Plus(Location),
    Assign(Location),
    Eq(Location),
    Neq(Location),
    Minus(Location),
    Star(Location),
    Slash(Location),
    LParen(Location),
    RParen(Location),
    LBrace(Location),
    RBrace(Location),
    Eof,
    Semi(Location),
    String(Location, String),
    Colon(Location),
    Comma(Location),
    Mod(Location),
    Not(Location),
    While(Location),
    Fn(Location),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Start => write!(f, "Start"),
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
            Token::String(pos, v) => write!(f, "String {} ( value: \"{}\")", pos, v),
            Token::Colon(pos) => write!(f, "Colon {}", pos),
            Token::LBrace(pos) => write!(f, "LBrace {}", pos),
            Token::RBrace(pos) => write!(f, "RBrace {}", pos),
            Token::If(pos) => write!(f, "if {}", pos),
            Token::Else(pos) => write!(f, "else {}", pos),
            Token::Loop(pos) => write!(f, "loop {}", pos),
            Token::Break(pos) => write!(f, "break {}", pos),
            Token::Colon(pos) => write!(f, "Colon {}", pos),
            Token::Comma(pos) => write!(f, "Comma {}", pos),
            Token::Assign(pos) => write!(f, "Assign {}", pos),
            Token::Neq(pos) => write!(f, "Neq {}", pos),
            Token::Mod(pos) => write!(f, "Mod {}", pos),
            Token::Not(pos) => write!(f, "Not {}", pos),
            Token::While(pos) => write!(f, "While {}", pos),
            Token::Fn(pos) => write!(f, "Fn {}", pos),
        }
    }
}


impl Token {
    pub fn get_location(&self) -> Location {
        match self {
            Token::Start => Location::zero(),
            Token::Number(pos, ..) => *pos,
            Token::Id(pos, ..) => *pos,
            Token::Plus(pos) => *pos,
            Token::Eq(pos) => *pos,
            Token::Minus(pos) => *pos,
            Token::Star(pos) => *pos,
            Token::Slash(pos) => *pos,
            Token::LParen(pos) => *pos,
            Token::RParen(pos) => *pos,
            Token::Eof => Location::Eof,
            Token::Semi(pos) => *pos,
            Token::String(pos, _) => *pos,
            Token::Colon(pos) => *pos,
            Token::LBrace(pos) => *pos,
            Token::RBrace(pos) => *pos,
            Token::If(pos) => *pos,
            Token::Else(pos) => *pos,
            Token::Loop(pos) => *pos,
            Token::Break(pos) => *pos,
            Token::Colon(pos) => *pos,
            Token::Comma(pos) => *pos,
            Token::Neq(pos) => *pos,
            Token::Assign(pos) => *pos,
            Token::Mod(pos) => *pos,
            Token::Not(pos) => *pos,
            Token::While(pos) => *pos,
            Token::Fn(pos) => *pos,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Plus(..) => true,
            Token::Eq(..) => true,
            Token::Minus(..) => true,
            Token::Star(..) => true,
            Token::Slash(..) => true,
            Token::Neq(..) => true,
            Token::Assign(..) => true,
            Token::Mod(..) => true,
            _ => false,
        }
    }
    pub fn is_modifier(&self) -> bool {
        match self {
            Token::Not(..) => true,
            _ => false,
        }
    }
    pub fn is_block(&self) -> bool {
        match self {
            Token::LParen(..) => true,
            Token::LBrace(..) => true,
            Token::If(..) => true,
            Token::While(..) => true,
            Token::Loop(..) => true,
            Token::Fn(..) => true,
            _ => false,
        }
    }

    pub fn is_noun(&self) -> bool {
        match self {
            Token::Number(..) => true,
            Token::Id(..) => true,
            Token::String(..) => true,
            _ => false,
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

