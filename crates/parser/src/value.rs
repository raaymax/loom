use std::fmt::Display;

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
            Value::String(s) => write!(f, "'{}'", s),
            _ => panic!("Type not yet implemented"),
        }
    }
}
