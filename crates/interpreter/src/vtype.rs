use std::fmt::Display;
use std::ops::{Add, Mul, Sub, Div};

use parser::{Value, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    Print,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VType {
    Func(Vec<String>, Node),
    Builtin(Builtin),
    Bool(bool),
    Args(Vec<VType>),
    Number(i32),
    String(String),
    Undefined,
}
impl From<Value> for VType {
    fn from(n: Value) -> Self {
        match n {
            Value::Number(n) => VType::Number(n),
            Value::String(s) => VType::String(s),
            _ => panic!("Type not yet implemented"),
        }
    }
}

impl From<u32> for VType {
    fn from(n: u32) -> Self {
        VType::Number(n as i32)
    }
}

impl From<&String> for VType {
    fn from(n: &String) -> Self {
        VType::String(n.clone())
    }
}
impl From<&str> for VType {
    fn from(n: &str) -> Self {
        VType::String(n.to_string())
    }
}

impl Display for VType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VType::Number(n) => write!(f, "{}", n),
            VType::String(s) => write!(f, "{}", s),
            VType::Undefined => write!(f, "undefined"),
            _ => panic!("Type not yet implemented"),
        }
    }
}

// TODO: those ops should be in interpreter?
impl Add for VType {
    type Output = VType;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (VType::Number(n1), VType::Number(n2)) => VType::Number(n1 + n2),
            (VType::String(s1), VType::Number(s2)) => VType::String(s1 + &s2.to_string()),
            (VType::Number(s1), VType::String(s2)) => VType::String(s1.to_string() + &s2),
            (VType::String(s1), VType::String(s2)) => VType::String(s1 + &s2),
            _ => panic!("Operation not yet implemented"),
        }
    }
}

impl Sub for VType {
    type Output = VType;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (VType::Number(n1), VType::Number(n2)) => VType::Number(n1 - n2),
            _ => panic!("Cannot subtract non-numbers"),
        }
    }
}

impl Mul for VType {
    type Output = VType;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (VType::Number(n1), VType::Number(n2)) => VType::Number(n1 * n2),
            (VType::String(n1), VType::Number(n2)) => VType::String({
                let mut s = String::new();
                for _ in 0..n2 {
                    s += &n1;
                }
                s
            }),
            _ => panic!("Cannot multiply non-numbers"),
        }
    }
}

impl Div for VType {
    type Output = VType;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (VType::Number(n1), VType::Number(n2)) => VType::Number(n1 / n2),
            _ => panic!("Cannot divide non-numbers"),
        }
    }
}

impl  VType {
    pub fn modulo(&self, other: &Self) -> VType {
        match (self, other) {
            (VType::Number(n1), VType::Number(n2)) => VType::Number(n1 % n2),
            _ => panic!("Cannot mod non-numbers"),
        }
    }
    pub fn equal(&self, other: &Self) -> VType {
        match (self, other) {
            (VType::Number(n1), VType::Number(n2)) => VType::Bool(n1 == n2),
            (VType::Bool(n1), VType::Bool(n2)) => VType::Bool(n1 == n2),
            _ => panic!("Cannot compare non-numbers"),
        }
    }
    pub fn not_equal(&self, other: &Self) -> VType {
        match (self, other) {
            (VType::Number(n1), VType::Number(n2)) => VType::Bool(n1 != n2),
            (VType::Bool(n1), VType::Bool(n2)) => VType::Bool(n1 != n2),
            _ => panic!("Cannot compare non-numbers"),
        }
    }

    pub fn not(&self) -> VType {
        match self {
            VType::Number(n1) => VType::Bool(*n1 > 0),
            VType::Bool(n1) => VType::Bool(!n1),
            _ => panic!("Cannot not non-bools"),
        }
    }
}

