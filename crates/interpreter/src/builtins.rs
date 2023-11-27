use std::fmt::Display;

use lexer::PError;

use crate::vtype::VType;

pub struct Args<'a>(&'a Vec<VType>);
impl Display for Args<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut first = true;
        for (idx, arg) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    Print,
    Pow,
    Assert,
}

impl Builtin {
    pub fn try_new(name: &str) -> Option<VType> {
        match name {
            "print" => Some(VType::Builtin(Builtin::Print)),
            "pow" => Some(VType::Builtin(Builtin::Pow)),
            "assert" => Some(VType::Builtin(Builtin::Assert)),
            _ => None,
        }
    }
    pub fn includes(name: &str) -> bool {
        match name {
            "print" => true,
            "pow" => true,
            "assert" => true,
            _ => false,
        }
    }
    pub fn call(&self,  args: &Vec<VType>) -> Result<VType, PError> {
        match self{
            Self::Print => {
                println!("{}", Args(args));
                Ok(VType::Undefined)
            },
            Self::Pow => {
                if args.len() != 2 {
                    return Err(PError::new(lexer::Location::zero(), "Expected 2 arguments"));
                }
                let a = args[0].clone();
                let b = args[1].clone();
                match (a, b) {
                    (VType::Number(a), VType::Number(b)) => Ok(VType::Number(a.pow(b as u32))),
                    _ => Err(PError::new(lexer::Location::zero(), "Expected numbers")),
                }
            },
            Self::Assert => {
                let a = args[0].clone();
                match a {
                    VType::Bool(b) => {
                        if !b {
                            return Err(PError::new(lexer::Location::zero(), "Assertion failed"));
                        }
                        Ok(VType::Undefined)
                    },
                    _ => Err(PError::new(lexer::Location::zero(), "Expected boolean")),
                }

            }
            _ => Err(PError::new(lexer::Location::zero(), "Not yet implemented")),
        }
    }   
}

