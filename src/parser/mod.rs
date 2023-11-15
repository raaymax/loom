mod value;
pub use value::Value;
use std::fmt::Display;
use std::slice::Iter;

use crate::loc::Location;
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
    Assign,
}

impl Op {
    fn priority(&self) -> u32 {
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


enum State {
    Expr,
    Operator,
    End,
    Eof
}

impl State {
    fn expect(&self, tree:  &mut Node, iter: &mut Iter<Token>, level: usize) -> Result<State, PError> {
        let Some(token) = iter.next() else {
            return Ok(State::Eof);
        };
        match self {
            Self::Expr => {
                match token {
                    Token::Number(loc, n) => {
                        tree.add(Node::new(Op::Def, Some((*n).into()), *loc));
                        Ok(Self::Operator)
                    },
                    Token::String(loc, n) => {
                        tree.add(Node::new(Op::Def, Some(n.into()), *loc));
                        Ok(Self::Operator)
                    },
                    Token::Id(loc, ref n) => {
                        tree.add(Node::new_var(Op::Def, n, *loc));
                        Ok(Self::Operator)
                    },
                    Token::LParen(loc) => {
                        let ret = build(iter, level+1, *loc)?;
                        match ret {
                            Some(node) => {
                                tree.add(node);
                                return Ok(Self::Operator);
                            },
                            None => {
                                return Err(PError::new(token.get_location(), "Unexpected end of file"))
                            },
                        }
                    },
                    Token::Eof => {
                        Err(PError::new(token.get_location(), "Unexpected end of file"))
                    }
                    _ => {
                        Err(PError::new(token.get_location(), "Invalid expression, expected ID or Number"))
                    }
                }
            },
            Self::Operator => {
                match token {
                    Token::Eq(loc) => {
                        tree.add(Node::new(Op::Assign, None, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Plus(loc) => {
                        tree.add(Node::new(Op::Add, None, *loc));
                        Ok(Self::Expr)
                    },
                    Token::Minus(loc) => {
                        tree.add(Node::new(Op::Sub, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Star(loc) => {
                        tree.add(Node::new(Op::Mul, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::Slash(loc) => {
                        tree.add(Node::new(Op::Div, None, *loc));
                        Ok(Self::Expr)
                    }
                    Token::RParen(_) => {
                        Ok(Self::End)
                    },
                    Token::Semi(_) => {
                        if level > 0 {
                            return Err(PError::new(token.get_location(), "Unexpected end of expression"));
                        }
                        Ok(Self::End)
                    },
                    Token::Eof => {
                        if level > 0 {
                            return Err(PError::new(token.get_location(), "Unexpected end of file"));
                        }
                        Ok(Self::End)
                    },
                    _ => {
                        Err(PError::new(token.get_location(), "Invalid expression, expected operator or semicolon"))
                    }
                }

            },
            Self::End => Ok(Self::End),
            Self::Eof => Ok(Self::Eof)
        }
    }
}



pub fn build(iter: &mut Iter<Token>, level: usize, loc: Location ) -> Result<Option<Node>, PError> {
    let mut tree: Node = Node::new(Op::Root, None, loc);
    let mut state = State::Expr;
    loop {
        state = state.expect(&mut tree, iter, level)?;
        match state {
            State::End => {
                return Ok(Some(tree));
            },
            State::Eof => {
                return Ok(None);
            },
            _ => {}
        }
        if let State::End = state {
            return Ok(Some(tree));
        }
    }
}
pub fn parse(iter: &mut Iter<Token>) -> Result<Vec<Node>, PError> {
    let mut arr = Vec::new();
    
    loop {
        let ret = build(iter, 0, Location::new_point(0,0,0));
        match ret {
            Ok(Some(v)) => arr.push(v),
            Ok(None) => return Ok(arr),
            Err(e) => return Err(e)
        }
    }
}
