use super::number::number;
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

    pub fn get_next(&mut self) -> Result<Token, PError> {
        while let Some((p, c)) = self.it.clone().next() {
            match c {
                '0'..='9' => {
                    let (size, num) = number(&mut self.it)?;
                    return Ok(Token::Number(p.set_range(size), num)); 
                },
                'a'..='z' | 'A'..='Z' => {
                    let (size, text) = id(&mut self.it)?;
                    return Ok(Token::Id(p.set_range(size), text)); 
                }
                '+' => {
                    return Ok(Token::Plus(self.get_location(1)));
                },
                '-' => {
                    return Ok(Token::Minus(self.get_location(1)));
                },
                '=' => {
                    return Ok(Token::Eq(self.get_location(1)));
                },
                '*' => {
                    return Ok(Token::Star(self.get_location(1)));
                },
                '/' => {
                    return Ok(Token::Slash(self.get_location(1)));
                },
                '(' => {
                    return Ok(Token::LParen(self.get_location(1)));
                },
                ')' => {
                    return Ok(Token::RParen(self.get_location(1)));
                },
                ';' => {
                    return Ok(Token::Semi(self.get_location(1)));
                },
                '\n' | ' ' => {
                    self.it.next();
                }
                _ => {
                    return Err(PError::new(p.to_point(), "Unexpected character"));
                }
            }
        }
        self.finished = true;
        Ok(Token::Eof)
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
