use super::number::number;
use super::string::string;
use super::id::id;
use crate::token::Token;
use crate::iter::LocationIterator;
use crate::loc::Location;
use crate::errors::PError;

pub struct Tokenizer<'a> {
    it: LocationIterator<'a>,
    finished: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            it: LocationIterator::new(text),
            finished: false,
        }
    }

    pub fn has_next(&self) -> bool {
        !self.finished
    }

    fn get_location(&mut self, length: usize) -> Location {
        let (p, _) = self.it.next().unwrap();
        p.set_range(length)
    }

    fn ignore_white_spaces(&mut self) {
        while let Some((_, c)) = self.it.clone().next() {
            if c == ' ' || c == '\n' {
                self.it.next();
            } else {
                break;
            }
        }
    }

    pub fn get_next(&mut self) -> Result<Token, PError> {
        let Some((p, c)) = self.it.clone().next() else {
            self.finished = true;
            return Ok(Token::Eof)
        };
        match c {
            '0'..='9' => {
                let (size, num) = number(&mut self.it)?;
                Ok(Token::Number(p.set_range(size), num))
            },
            'a'..='z' | 'A'..='Z' => {
                let (size, text) = id(&mut self.it)?;
                Ok(Token::Id(p.set_range(size), text))
            }
            '+' =>  Ok(Token::Plus(self.get_location(1))),
            '-' =>  Ok(Token::Minus(self.get_location(1))),
            '=' => Ok(Token::Eq(self.get_location(1))),
            '*' => Ok(Token::Star(self.get_location(1))),
            '/' => Ok(Token::Slash(self.get_location(1))),
            '(' => Ok(Token::LParen(self.get_location(1))),
            ')' => Ok(Token::RParen(self.get_location(1))),
            '{' => Ok(Token::LBrace(self.get_location(1))),
            '}' => Ok(Token::RBrace(self.get_location(1))),
            ';' => Ok(Token::Semi(self.get_location(1))),
            ':' => Ok(Token::Colon(self.get_location(1))),
            '\'' | '"' => {
                let (size, num) = string(&mut self.it)?;
                Ok(Token::String(p.set_range(size), num))
            },
            '\n' | ' ' => {
                self.ignore_white_spaces();
                self.get_next()
            }
            _ => Err(PError::new(p.to_point(), "Unexpected character"))
        }
    }

}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, PError>;


    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        Some(self.get_next())
    }
}
