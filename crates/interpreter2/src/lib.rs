mod types;
use std::fmt::Display;
use std::{collections::VecDeque, rc::Rc};
use std::cell::RefCell;

use parser::{Node, Op, Value};
use lexer::PError;


#[derive(Debug, Clone, PartialEq, Eq)]
enum VType {
    Func(Vec<String>, Node, String),
    Ref(String),
    Bool(bool),
    Args(Vec<VType>),
    Number(i32),
    String(String),
    Undefined,
}


impl VType {
    fn unwrap_number(&self) -> Result<i32, RuntimeError> {
        if let VType::Number(n) = self {
            Ok(*n)
        } else {
            Err(RuntimeError::new("Expected a number", None))
        }
    }
}

impl From<Value> for VType {
    fn from(n: Value) -> Self {
        match n {
            Value::Number(n) => VType::Number(n),
            Value::String(s) => VType::String(s),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    location: Option<lexer::Location>,
}

impl RuntimeError {
    fn new(message: &str, location: Option<lexer::Location>) -> Self {
        RuntimeError {
            message: message.to_string(),
            location,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{} at {:?}", self.message, loc)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

struct StackFrame<'a> {
    node: &'a Node,
    values: Vec<VType>,
    parent: Option<Rc<RefCell<StackFrame<'a>>>>,

    // You can add more fields as needed, such as local variables, return address, etc.
}

impl StackFrame<'_> {
    fn get_depth(&self) -> usize {
        if let Some(parent) = &self.parent {
            1 + parent.borrow().get_depth()
        } else {
            1
        }
    }
}

struct Stack<'a> {
    frames: VecDeque<Rc<RefCell<StackFrame<'a>>>>,
    current_frame: Option<Rc<RefCell<StackFrame<'a>>>>,
}

pub fn exec(node: &Node) -> Result<VType, RuntimeError> {
    //println!("Executing node: {:?}", node);
    let root = Rc::new(RefCell::new(StackFrame {
        node,
        values: Vec::new(),
        parent: None,
    }));
    let mut stack = Stack {
        frames: VecDeque::from([root.clone()]),
        current_frame: Some(root.clone()),
    };
    while stack.current_frame.is_some() {
        let mut binding = &mut stack.current_frame.clone().unwrap();
        let mut frame = binding.borrow_mut();
        let count = frame.values.len();
        let node_count = frame.node.children_count();

        println!("At node: {:?}, count: {}, node_count: {} depth: {}", frame.node.op, count, node_count, frame.get_depth());

        let ret = compute(&frame.node, &frame.values)?;
        if let Some(value) = ret {
            stack.frames.pop_back();
            if frame.parent.is_none() {
                println!("Final result: {:?}", value);
                return Ok(value);
            }
            stack.current_frame = frame.parent.clone();
            frame.parent.clone().unwrap().borrow_mut().values.push(value);
        } else {
            let next_frame = Rc::new(RefCell::new(StackFrame {
                node: frame.node.get_child(count),
                values: Vec::new(),
                parent: stack.current_frame.clone(),
            }));
            stack.frames.push_back(next_frame.clone());
            stack.current_frame = Some(next_frame);
        }
    }

    Ok(VType::Undefined)
}

fn compute(node: &Node, values: &Vec<VType>) -> Result<Option<VType>, RuntimeError> {
    let node_count = node.children_count();

    match node.op {
        Op::Add => {
                if values.len() != 2 { return Ok(None) }
                match (&values[0], &values[1]) {
                    (VType::Number(a), VType::Number(b)) => Ok(Some(VType::Number(a + b))),
                    (VType::String(a), VType::String(b)) => Ok(Some(VType::String(format!("{}{}", a, b)))),
                    _ => Err(RuntimeError::new("Add operation requires both operands to be numbers or strings", None)),
                }
            },
        Op::Mod => {
            if values.len() < 2 { return Ok(None) }
            match (&values[0], &values[1]) {
                (VType::Number(a), VType::Number(b)) => {
                    if *b == 0 {
                        Err(RuntimeError::new("Division by zero in modulus operation", None))
                    } else {
                        Ok(Some(VType::Number(a % b)))
                    }
                },
                _ => Err(RuntimeError::new("Modulus operation requires both operands to be numbers", None)),
            }
        },
        Op::Sub => {
            if values.len() < 2 { return Ok(None) }
            match (&values[0], &values[1]) {
                (VType::Number(a), VType::Number(b)) => Ok(Some(VType::Number(a - b))),
                _ => Err(RuntimeError::new("Sub operation requires both operands to be numbers", None)),
            }
        },
        Op::Mul => {
            if values.len() < 2 { return Ok(None) }
            match (&values[0], &values[1]) {
                (VType::Number(a), VType::Number(b)) => Ok(Some(VType::Number(a * b))),
                _ => Err(RuntimeError::new("Mul operation requires both operands to be numbers", None)),
            }
        },
        Op::Div => {
            if values.len() < 2 { return Ok(None) }
            match (&values[0], &values[1]) {
                (VType::Number(a), VType::Number(b)) => {
                    if *b == 0 {
                        Err(RuntimeError::new("Division by zero", None))
                    } else {
                        Ok(Some(VType::Number(a / b)))
                    }
                },
                _ => Err(RuntimeError::new("Div operation requires both operands to be numbers", None)),
            }
        },
        Op::Scope => {
                if values.len() < node_count {
                    return Ok(None);
                }
                Ok(Some(values.last().unwrap_or(&VType::Undefined).clone()))
            }
        Op::Paren => {
                if values.len() < node_count {
                    return Ok(None);
                }
                Ok(Some(values.last().unwrap_or(&VType::Undefined).clone()))
            }
        Op::Args => {
                if values.len() < node_count {
                    return Ok(None);
                }
                Ok(Some(VType::Args(values.clone())))
            },
        Op::Branch => todo!("Branch not implemented"),
        Op::Loop => todo!("Loop not implemented"),
        Op::Value => {
            if let Some(val) = &node.value {
                return Ok(Some(val.clone().into()));
            }
            Ok(Some(VType::Undefined))
        },
        Op::Variable => todo!("Variable not implemented"),
        Op::Assign => todo!("Assign not implemented"),
        Op::Call => todo!("Call not implemented"),
        Op::While => todo!("While not implemented"),
        Op::DefineFunc => todo!("DefineFunc not implemented"),
        Op::DefineVar => todo!("DefineVar not implemented"),
        Op::Return => todo!("Return not implemented"),
        Op::Eq => {
                if values.len() != 2 { return Ok(None) }
                Ok(Some(VType::Bool(values.len() == 2 && values[0] == values[1])))
            }
        Op::Neq => {
                if values.len() != 2 { return Ok(None) }
                Ok(Some(VType::Bool(values.len() == 2 && values[0] != values[1])))
            }
        Op::Leq => todo!("Leq not implemented"),
        Op::Lt => todo!("Lt not implemented"),
        Op::Geq => todo!("Geq not implemented"),
        Op::Gt => todo!("Gt not implemented"),
        Op::Not => todo!("Not not implemented"),
        Op::And => {
            if values.len() < 1 { return Ok(None) }
            else if values.len() == 1 {
                if values[0] == VType::Bool(false) {
                    return Ok(Some(VType::Bool(false)));
                }else {
                    return Ok(None);
                }
            } else {
                return Ok(Some(VType::Bool(values[0] == VType::Bool(true) && values[1] == VType::Bool(true))));
            }
        }
        Op::Or => todo!("Or not implemented"),
        Op::Access => todo!("Access not implemented"),
    }
}


pub fn interpret(node: Node) -> Result<i32, RuntimeError> {

    exec(&node).unwrap().unwrap_number()
}

