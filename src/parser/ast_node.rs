use super::value::Value;
use std::{fmt::Display, collections::HashMap};
use crate::loc::Location;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Scope,
    Branch,
    Loop,
    Add,
    Sub,
    Mul,
    Div,
    Value,
    Var,
    Assign,
}

impl Op {
    pub fn priority(&self) -> u32 {
        match self {
            Op::Assign=> 5,
            Op::Add => 4,
            Op::Sub => 4,
            Op::Mul => 3,
            Op::Div => 3,
            Op::Value => 2,
            Op::Var=> 2,
            Op::Branch => 1,
            Op::Loop => 1,
            Op::Scope => 0,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Assign => write!(f, "="),
            Op::Add => write!(f, "+"),
            Op::Value => write!(f, ""),
            Op::Var => write!(f, "var:"),
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Sub => write!(f, "-"),
            Op::Branch => write!(f, ""),
            Op::Loop => write!(f, ""),
            Op::Scope => write!(f, ""),
        }
    }
}

#[derive(Debug)]
struct OptionalNode<'a>(Option<&'a Node>);

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
    defs: HashMap<String, Node>,
    op: Op,
    location: Location,
    value: Option<Value>,
    id: Option<String>,
    children: Vec<Node>,
}

impl Node {
    pub fn new_op(op: Op, location: Location) -> Self {
        Self {
            op,
            location,
            value: None,
            defs: HashMap::new(),
            id: None,
            children: Vec::new(),
        }
    }
    pub fn new_value(value: Value, location: Location) -> Self {
        Self {
            op: Op::Value,
            value: Some(value),
            location,
            defs: HashMap::new(),
            id: None,
            children: Vec::new(),
        }
    }
    pub fn new_var(id: &str, location: Location) -> Self {
        Self {
            op: Op::Var,
            id: Some(id.to_string()),
            location,
            value: None,
            defs: HashMap::new(),
            children: Vec::new(),
        }
    }
    pub fn new_scope(location: Location) -> Self {
        Self{
            op: Op::Scope,
            id:None,
            location,
            value: None,
            defs: HashMap::new(),
            children: Vec::new(),
        }
    }
    pub fn new_loop(location: Location) -> Self {
        Self{
            op: Op::Loop,
            id:None,
            location,
            value: None,
            defs: HashMap::new(),
            children: Vec::new(),
        }
    }
    pub fn new_branch(location: Location) -> Self {
        Self{
            op: Op::Loop,
            id:None,
            location,
            value: None,
            defs: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn priority(&self) -> i32 {
        self.op.priority() as i32
    }

    pub fn last(&mut self) -> Option<&mut Node> {
        self.children.last_mut()
    }

    pub fn add(&mut self, node: Node) {
        if let Some(ref mut right) = self.last() {
            let score = node.priority() - right.priority();
            if score < 0 {
                right.add(node);
            }else if score == 0 {
                self.children.push(node);
            }else{
                let mut node = node;
                if let Some(left) = self.children.pop() {
                    println!("left: {} <- {}", left, node);
                    node.add(left);
                }
                self.children.push(node);
            }
        } else {
            self.children.push(node);
        } 
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            Op::Add => {
                write!(f,"({} + {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Sub=> {
                write!(f,"({} - {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Mul=> {
                write!(f,"({} * {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Div=> {
                write!(f,"({} / {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Assign=> {
                write!(f,"({} = {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Value => {write!(f, "{}", self.value.as_ref().unwrap())?;},
            Op::Var => {write!(f, "{}", self.id.as_ref().unwrap())?;},
            Op::Scope => {
                write!(f,"{{")?;
                for node in self.children.iter() {
                    write!(f,"{}", node)?;
                }
                write!(f,"}}")?;
            },
            Op::Loop => {
                if let Some(condition) = self.children.get(0) {
                    writeln!(f,"while {}", condition)?;
                } else {
                    writeln!(f,"loop")?;
                }

                if let Some(body) = self.children.get(1){
                    writeln!(f,"{}", body)?;
                }
            },
            Op::Branch => {
                if let Some(condition) = self.children.get(0) {
                    writeln!(f,"if {}", condition)?;
                } else {
                    writeln!(f,"if ?")?;
                }

                if let Some(if_body) = self.children.get(1) {
                    writeln!(f,"{}", if_body)?;
                }

                if let Some(else_body) = self.children.get(2) {
                    writeln!(f,"else {}", else_body)?;
                }
            },
            _ => write!(f,"N/A")?, 
        }
        Ok(())
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(value: Node) -> Self {
        Some(Box::new(value))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::loc::Location;

    #[test]
    fn test_ast() {
        let mut root = Node::new_op(Op::Scope, Location::zero());
        let mut node = Node::new_op(Op::Add, Location::zero());
        node.add(Node::new_value(Value::Number(1), Location::zero()));
        node.add(Node::new_value(Value::Number(2), Location::zero()));
        root.add(node);
        assert_eq!(format!("{}", root), "{(1 + 2)}");
    }
}
