use super::value::Value;
use std::{fmt::Display, collections::HashMap};
use crate::loc::Location;


#[derive(Debug, Copy, Clone)]
pub enum Op {
    Root,
    Add,
    Sub,
    Mul,
    Div,
    Def,
    Assign,
}

impl Op {
    pub fn priority(&self) -> u32 {
        match self {
            Op::Assign=> 4,
            Op::Add => 3,
            Op::Sub => 3,
            Op::Mul => 2,
            Op::Div => 2,
            Op::Def => 1,
            Op::Root=> 0,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Assign => write!(f, "="),
            Op::Add => write!(f, "+"),
            Op::Def => write!(f, "def:"),
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Sub => write!(f, "-"),
            Op::Root => write!(f, ""),
        }
    }
}

#[derive(Debug)]
struct OptionalNode<'a>(&'a Option<Box<Node>>);

impl Display for OptionalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.0 {
            write!(f, "{}", node)
        } else {
            write!(f, "N/A")
        }
    }
}
#[derive(Debug)]
pub struct Node {
    pub op: Op,
    pub location: Location,
    pub id: Option<String>,
    pub value: Option<Value>,
    pub left: Option<Box<Node>>, 
    pub right: Option<Box<Node>>
}

impl Node {
    pub fn new(op: Op, value: Option<Value>, location: Location) -> Self {
        Self {
            op,
            id: None,
            location,
            value,
            left: None,
            right: None,
        }
    }
    pub fn new_var(op: Op, id: &str, location: Location) -> Self {
        Self {
            op,
            id: Some(id.to_string()),
            location,
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, node: Node) {
        if let Some(ref mut right) = self.right {
            if node.op.priority() < right.op.priority() {
                right.add(node);
            }else{
                let mut node = node;
                let left = self.right.take();
                node.left = left;
                self.right = node.into();
            }
        } else {
            self.right = node.into();
        } 
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Op::Root = self.op {
            write!(f,"{}", OptionalNode(&self.right))?;
        }else if let Some(Value::Number(x)) = self.value {
            write!(f,"{}", x)?;
        }else if let Some(Value::String(ref x)) = self.value {
            write!(f,"\"{}\"", x)?;
        }else if let Some(ref n) = self.id {
            write!(f,"{}", n)?;
        }else{
            write!(f,"({} {} {})", OptionalNode(&self.left),self.op,  OptionalNode(&self.right))?;
        }
        Ok(())
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(value: Node) -> Self {
        Some(Box::new(value))
    }
}

