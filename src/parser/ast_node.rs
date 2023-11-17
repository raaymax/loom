use super::value::Value;
use std::{fmt::Display, collections::HashMap};
use crate::loc::Location;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Op {
    Placeholder,
    Scope,
    Paren,
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
            Op::Placeholder=> 10,
            Op::Scope => 7,
            Op::Branch => 6,
            Op::Loop => 6,
            Op::Assign=> 5,
            Op::Add => 4,
            Op::Sub => 4,
            Op::Mul => 3,
            Op::Div => 3,
            Op::Value => 0,
            Op::Var=> 0,
            Op::Paren => 0,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Placeholder => write!(f, "??"),
            Op::Assign => write!(f, "="),
            Op::Add => write!(f, "+"),
            Op::Value => write!(f, "X"),
            Op::Var => write!(f, "var"),
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Sub => write!(f, "-"),
            Op::Branch => write!(f, "if"),
            Op::Loop => write!(f, "loop"),
            Op::Scope => write!(f, "{{}}"),
            Op::Paren => write!(f, "()"),
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
    pub defs: HashMap<String, Node>,
    pub op: Op,
    pub location: Location,
    pub value: Option<Value>,
    pub id: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new_placeholder() -> Self {
        Self {
            op: Op::Placeholder,
            location: Location::zero(),
            value: None,
            defs: HashMap::new(),
            id: None,
            children: Vec::new(),
        }
    }
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

    pub fn new_paren(location: Location) -> Self {
        Self{
            op: Op::Paren,
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

    pub fn is_leaf(&self) -> bool {
        self.op.priority() == 0
    }

    pub fn last(&mut self) -> Option<&mut Node> {
        self.children.last_mut()
    }
    pub fn right_mut(&mut self) -> Option<&mut Node> {
        self.children.get_mut(1)
    }
    pub fn left_mut(&mut self) -> Option<&mut Node> {
        self.children.get_mut(0)
    }
    pub fn right(&self) -> Option<&Node> {
        self.children.get(1)
    }
    pub fn left(&self) -> Option<&Node> {
        self.children.get(0)
    }

    pub fn is_complete(&self) -> bool {
        match self.op {
            Op::Scope | Op::Paren => {
                for child in &self.children {
                    if !child.is_complete() {
                        return false;
                    }
                }
                true
            },
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Assign => {
                let Some(left) = self.left() else {
                    return false;
                };
                let Some(right) = self.right() else {
                    return false;
                };
                left.is_complete() && right.is_complete()
            },
            Op::Branch | Op::Loop => true,
            Op::Placeholder => false,
            _ => true,
        }
    }

    pub fn next(&mut self, node: Node) {
        if self.op != Op::Scope {
            return;
        }
        self.children.push(node)
    }

    pub fn add(&mut self, node: Node) {
        //println!("self: {} << {}", self, node);
        match self.op {
            Op::Scope | Op::Assign | Op::Paren => {
                if let Some(right) = self.last() {
                    if right.op == Op::Placeholder {
                        *right = node;
                        return;
                    }
                    if node.priority() < right.priority() {
                        //println!("right: {} -> {}", right, node);
                        if right.is_leaf() {
                            self.children.push(node);
                            return;
                        }
                        right.add(node);
                    }else{
                        //println!("left: {} <- {}", right, node);
                        let mut node = node;
                        if node.is_leaf() {
                            //println!("leaf: {}", right);
                            self.children.push(node);
                            return;
                        }
                        if let Some(left) = self.children.pop() {
                            node.add(left);
                        }
                        self.children.push(node);
                    }
                } else {
                    self.children.push(node);
                } 
            },
            Op::Add | Op::Sub | Op::Mul | Op::Div => {
                if let Some(ref mut right) = self.right_mut() {
                    if node.priority() < right.priority() {
                        //println!("right: {} -> {}", right, node);
                        if right.is_leaf() {
                            self.children.push(node);
                            return;
                        }
                        right.add(node);
                    }else{
                        //println!("left: {} <- {}", right, node);
                        let mut node = node;
                        if node.is_leaf() {
                            //println!("leaf: {}", right);
                            self.children.push(node);
                            return;
                        }
                        if let Some(left) = self.children.pop() {
                            node.add(left);
                        }
                        self.children.push(node);
                    }
                } else {
                    self.children.push(node);
                } 
            },
            _ => panic!("not implemented"),
        }
        //println!("self: {} out", self);
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
                for (idx, node) in self.children.iter().enumerate() {
                    if idx > 0 { write!(f,";")? };
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
            Op::Paren => {
                if let Some(body) = self.children.get(0) {
                    write!(f,"{}", body)?;
                } else {
                    write!(f,"()")?;
                }
            }
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
