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

    pub fn get_next(&mut self) -> Result<Token, PError> {
        let Some((p, c)) = self.it.clone().next() else {
            self.finished = true;
            return Ok(Token::Eof)
        };
        match c {
            x if x.is_numeric() => {
                let (size, num) = number(&mut self.it)?;
                Ok(Token::Number(p.set_range(size), num))
            },
            x if x.is_alphabetic() => {
                let (size, text) = id(&mut self.it)?;
                match text.as_str() {
                    "if" => return Ok(Token::If(p.set_range(size))),
                    "else" => return Ok(Token::Else(p.set_range(size))),
                    "loop" => return Ok(Token::Loop(p.set_range(size))),
                    "break" => return Ok(Token::Break(p.set_range(size))),
                    _ => {}
                }
                Ok(Token::Id(p.set_range(size), text))
            }
            '+' =>  Ok(Token::Plus(self.get_location(1))),
            '-' =>  Ok(Token::Minus(self.get_location(1))),
            '=' => Ok(Token::Eq(self.get_location(1))),
            '*' => Ok(Token::Star(self.get_location(1))),
            '/' => {
                if self.accept("//") {
                    self.skip_until("\n");
                    return self.get_next();
                }
                if self.accept("/*") {
                    self.skip_until("*/");
                    return self.get_next();
                }
                Ok(Token::Slash(self.get_location(1)))
            },
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
    fn skip_until(&mut self, expected: &str) {
        while !self.accept(expected) {
            self.it.next();
        };
        //self.it.nth(expected.len()-1);
    }

    fn accept(&mut self, expected: &str) -> bool {
        let mut it = self.it.clone();
        for exp_c in expected.chars() {
            if let Some((_, c)) = it.next() {
                if c != exp_c {
                    return false
                }
            } 
        }
        self.it.nth(expected.len()-1);
        true
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
