use super::value::Value;
use std::{fmt::Display, collections::HashMap};
use lexer::Location;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Op {
    Scope,
    Paren,
    Args,
    Eq,
    Neq,
    Mod,
    Not,
    Branch,
    Loop,
    Add,
    Sub,
    Mul,
    Div,
    Value,
    Variable,
    Assign,
    Call,
    While,
    DefineFunc,
    DefineVar,
    Return,
    Leq,
    Lt,
    Geq,
    Gt,
}

impl Op {
    pub fn priority(&self) -> u32 {
        match self {
            Op::Scope => 0,
            Op::DefineVar => 6,
            Op::Return => 6,
            Op::Branch => 6,
            Op::Loop => 6,
            Op::While => 6,
            Op::Assign=> 5,
            Op::Lt=> 5,
            Op::Leq=> 5,
            Op::Gt=> 5,
            Op::Geq=> 5,
            Op::Neq=> 5,
            Op::Eq=> 5,
            Op::Add => 4,
            Op::Sub => 4,
            Op::Mul => 3,
            Op::Div => 3,
            Op::Mod => 2,
            Op::Call => 1,
            Op::Args => 1,
            Op::Not=> 1,
            Op::DefineFunc=> 0,
            Op::Value => 0,
            Op::Variable=> 0,
            Op::Paren => 0,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Assign => write!(f, "="),
            Op::Lt => write!(f, "<"),
            Op::Leq => write!(f, "<="),
            Op::Gt => write!(f, ">"),
            Op::Geq => write!(f, ">="),
            Op::Add => write!(f, "+"),
            Op::Value => write!(f, "X"),
            Op::Variable => write!(f, "var"),
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Sub => write!(f, "-"),
            Op::Branch => write!(f, "if"),
            Op::Loop => write!(f, "loop"),
            Op::Scope => write!(f, "{{}}"),
            Op::Paren => write!(f, "()"),
            Op::Call => write!(f, "fn"),
            Op::Args => write!(f, "..."),
            Op::Eq => write!(f, "=="),
            Op::Neq => write!(f, "!="),
            Op::Not => write!(f, "!"),
            Op::Mod => write!(f, "%"),
            Op::While => write!(f, "while"),
            Op::DefineFunc => write!(f, "Fn"),
            Op::Return => write!(f, "return"),
            Op::DefineVar => write!(f, "let"),
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub defs: HashMap<String, Node>,
    pub op: Op,
    pub location: Location,
    pub value: Option<Value>,
    pub id: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(op: Op, location: Location) -> Self {
        Self {
            op,
            location,
            value: None,
            defs: HashMap::new(),
            id: None,
            children: Vec::new(),
        }
    }

    pub fn set_value(mut self, value: Value) -> Self {
        if Op::Value != self.op {
            panic!("Cannot set value on non-value node");
        }
        self.value = Some(value);
        self
    }
    pub fn set_id(mut self, id: String) -> Self {
        if matches!(self.op, Op::Variable | Op::DefineVar) {
            self.id = Some(id);
        } else {
            panic!("Cannot set id on non-var node");
        }
        self
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

    pub fn add(&mut self, node: Node) {
        //println!("self: {} << {}", self, node);
        match self.op {
            Op::Call | Op::DefineVar => {
                self.children.insert(0, node);
            }
            Op::Scope | Op::Branch | Op::Args | Op::Not | Op::While | Op::DefineFunc | Op::Return => {
                self.children.push(node);
            },
            Op::Assign | Op::Paren => {
                if let Some(right) = self.last() {
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
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Eq | Op::Neq
                | Op::Mod | Op::Geq | Op::Gt | Op::Leq | Op::Lt => {
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
            _ => panic!("not implemented {:?}", self.op),
        }
        //println!("self: {} out", self);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            Op::Return => {
                write!(f,"(return {})", OptionalNode(self.children.get(0)))?;
            },
            Op::DefineFunc=> {
                write!(f,"fn {}{} {}", OptionalNode(self.children.get(0)),OptionalNode(self.children.get(1)), OptionalNode(self.children.get(2)))?;
            },
            Op::While => {
                write!(f,"while ({}) {}", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Call=> {
                write!(f,"{}{}", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Lt => {
                write!(f,"({} < {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Gt => {
                write!(f,"({} > {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Leq => {
                write!(f,"({} <= {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Geq => {
                write!(f,"({} >= {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Eq => {
                write!(f,"({} == {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Neq => {
                write!(f,"({} != {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
            Op::Mod => {
                write!(f,"({} % {})", OptionalNode(self.children.get(0)), OptionalNode(self.children.get(1)))?;
            },
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
            Op::Variable => {write!(f, "{}", self.id.as_ref().unwrap())?;},
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
                    write!(f,"if({})", condition)?;
                } else {
                    write!(f,"if ?")?;
                }

                if let Some(if_body) = self.children.get(1) {
                    write!(f,"{}", if_body)?;
                }else {
                    write!(f," ??")?;
                }

                if let Some(else_body) = self.children.get(2) {
                    write!(f," else {}", else_body)?;
                }
            },
            Op::Paren | Op::Args => {
                if self.children.len() != 1 { write!(f,"(")? };
                for (i, child) in self.children.iter().enumerate() {
                    if i > 0 {
                        write!(f,",")?;
                    }
                   write!(f,"{}", OptionalNode(Some(child)))?;
                }
                if self.children.len() != 1 { write!(f,")")? };
            },
            Op::DefineVar => {
                if self.children.is_empty() { 
                    return write!(f,"let {}",self.id.as_ref().unwrap() )
                };
                write!(f,"let {}; {}", self.id.as_ref().unwrap(), OptionalNode(self.children.get(0)))?;
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
    use lexer::Location;

    #[test]
    fn test_ast() {
        let mut root = Node::new(Op::Scope, Location::zero());
        let mut node = Node::new(Op::Add, Location::zero());
        node.add(Node::new(Op::Value, Location::zero()).set_value(1.into()));
        node.add(Node::new(Op::Value, Location::zero()).set_value(2.into()));
        root.add(node);
        assert_eq!(format!("{}", root), "{(1 + 2)}");
    }
}
