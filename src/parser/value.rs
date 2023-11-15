use std::fmt::Display;
use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(i32),
    String(String),
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::Number(n as i32)
    }
}

impl From<&String> for Value {
    fn from(n: &String) -> Self {
        Value::String(n.clone())
    }
}
impl From<&str> for Value {
    fn from(n: &str) -> Self {
        Value::String(n.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            _ => panic!("Type not yet implemented"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            (Value::String(s1), Value::Number(s2)) => Value::String(s1 + &s2.to_string()),
            (Value::Number(s1), Value::String(s2)) => Value::String(s1.to_string() + &s2),
            (Value::String(s1), Value::String(s2)) => Value::String(s1 + &s2),
            _ => panic!("Operation not yet implemented"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            _ => panic!("Cannot subtract non-numbers"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            (Value::String(n1), Value::Number(n2)) => Value::String({
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

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
            _ => panic!("Cannot divide non-numbers"),
        }
    }
}
