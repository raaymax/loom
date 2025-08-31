use std::fmt::Display;

use lexer::{Location, PError};

use crate::{stack::StackFramePtr, vtype::VType};

pub struct Args<'a>(&'a Vec<VType>);
impl Display for Args<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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
    Trace,
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Builtin::Print => write!(f, "print"),
            Builtin::Pow => write!(f, "pow"),
            Builtin::Assert => write!(f, "assert"),
            Builtin::Trace => write!(f, "trace"),
        }
    }
}

impl Builtin {
    pub fn try_new(name: &str) -> Option<VType> {
        match name {
            "print" => Some(VType::Builtin(Builtin::Print)),
            "pow" => Some(VType::Builtin(Builtin::Pow)),
            "assert" => Some(VType::Builtin(Builtin::Assert)),
            "trace" => Some(VType::Builtin(Builtin::Trace)),
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
    pub fn call(&self, location: Location, args: &Vec<VType>, trace: &StackFramePtr) -> Result<VType, PError> {
        match self{
            Self::Print => {
                println!("{}", Args(args));
                Ok(VType::Undefined)
            },
            Self::Trace => {
                Ok(VType::String(trace.borrow().stacktrace()))
            },
            Self::Pow => {
                if args.len() != 2 {
                    return Err(PError::new(location, "Expected 2 arguments"));
                }
                let a = args[0].clone();
                let b = args[1].clone();
                match (a, b) {
                    (VType::Number(a), VType::Number(b)) => Ok(VType::Number(a.pow(b as u32))),
                    _ => Err(PError::new(location, "Expected numbers")),
                }
            },
            Self::Assert => {
                let a = args[0].clone();
                match a {
                    VType::Bool(b) => {
                        if !b {
                            return Err(PError::new(location, "Assertion failed"));
                        }
                        Ok(VType::Undefined)
                    },
                    _ => Err(PError::new(location, "Expected boolean")),
                }

            }
        }
    }   
}

