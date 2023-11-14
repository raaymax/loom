use std::fmt::Display;
use std::slice::Iter;

use crate::token::Token;
use crate::errors::PError;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Root,
    Add,
    Sub,
    Mul,
    Div,
    Def,
}

impl Op {
    fn priority(&self) -> u32 {
        match self {
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
            Op::Add => write!(f, "+"),
            Op::Def => write!(f, "="),
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
    op: Op,
    id: Option<String>,
    value: Option<u32>,
    left: Option<Box<Node>>, 
    right: Option<Box<Node>>
}

impl Node {
    pub fn new(op: Op, value: Option<u32>) -> Self {
        Self {
            op,
            id: None,
            value,
            left: None,
            right: None,
        }
    }
    pub fn new_var(op: Op, id: &str) -> Self {
        Self {
            op,
            id: Some(id.to_string()),
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
        }else if let Some(x) = self.value {
            write!(f,"{}", x)?;
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

pub fn build(iter: &mut Iter<Token>) -> Result<Node, PError> {
    let mut tree: Node = Node::new(Op::Root, None);
    while let Some(token) = iter.next() {
        match token {
            Token::Number(_, n) => tree.add(Node::new(Op::Def, Some(*n))),
            Token::Id(_, ref n) => tree.add(Node::new_var(Op::Def, n)),
            Token::Plus(_) => tree.add(Node::new(Op::Add, None)),
            Token::Minus(_) => tree.add(Node::new(Op::Sub, None)),
            Token::Star(_) => tree.add(Node::new(Op::Mul, None)),
            Token::Slash(_) => tree.add(Node::new(Op::Div, None)),
            Token::LParen(_) => tree.add(build(iter)?),
            Token::RParen(_) => break,
            _ => {}
        }
    }
    Ok(tree)
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Node, PError> {
    let mut iter = tokens.iter();
    build(&mut iter)
}

