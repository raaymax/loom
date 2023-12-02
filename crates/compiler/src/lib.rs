mod vm;
mod op_code;
mod instr;

extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate enum_display;

use std::{fmt::{Display, Formatter}};

use lexer::PError;
use parser::{Node, Op, Value};
pub use vm::VM;
pub use instr::{Instr, Instrs};
pub use op_code::OpCode;


struct Instruction {
    op_code: OpCode,
    arg0: Option<u8>,
    arg1: Option<u8>,
    arg2: Option<u8>,
}


impl Instruction {
    pub fn new(op_code: OpCode) -> Instruction {
        Instruction {
            op_code,
            arg0: None,
            arg1: None,
            arg2: None,
        }
    }
    pub fn with_arg0(mut self, arg0: u8) -> Instruction {
        self.arg0 = Some(arg0);
        self
    }
    pub fn with_arg1(mut self, arg1: u8) -> Instruction {
        self.arg1 = Some(arg1);
        self
    }
    pub fn with_arg2(mut self, arg2: u8) -> Instruction {
        self.arg2 = Some(arg2);
        self
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.op_code as u8);
        if let Some(arg0) = self.arg0 {
            bytes.push(arg0);
        }
        if let Some(arg1) = self.arg1 {
            bytes.push(arg1);
        }
        if let Some(arg2) = self.arg2 {
            bytes.push(arg2);
        }
        bytes
    }
}


struct Code {
    code: Vec<Instruction>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            code: Vec::new(),
        }
    }
    pub fn push(&mut self, inst: Instruction) {
        self.code.push(inst);
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.code.iter().flat_map(|i| i.to_bytes()).collect::<Vec<_>>()
    }
}

struct Mem {    
    data: Vec<u8>,
    memptr: usize,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            data: Vec::new(),
            memptr: 0,
        }
    }
    pub fn write(&mut self, val: u32) -> usize {
        let location = self.memptr;
        for i in 0..4 {
            self.data[self.memptr + i] = (val >> (i * 8)) as u8;

        }
        self.memptr += 4;
        location
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
impl Display for Mem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mem: {:?}", self.data)
    }
}

pub fn compile_node(node: &Node, mut consts: &mut Mem, inst: &mut Code) -> Result<(), PError> {
    match node.op {
        Op::Add => {
            compile_node(&node.children[0], &mut consts, inst)?;
            compile_node(&node.children[1], &mut consts, inst)?;
            inst.push(Instruction::new(OpCode::Add));
        },
        Op::Value => {
            match &node.value {
                Some(Value::Number(0)) => inst.push(Instruction::new(OpCode::Load0)),
                Some(Value::Number(1)) => inst.push(Instruction::new(OpCode::Load1)),
                Some(Value::Number(val)) => {
                    let addr = consts.write(*val as u32);
                    if addr < 256 {
                        inst.push(Instruction::new(OpCode::Load).with_arg0(addr.try_into().unwrap()));
                    }
                },
                Some(..) => {
                    panic!("Unknown value: {}", node);
                },
                None => panic!("No value"),
            }
        },
        Op::Scope => {
            for child in &node.children {
                compile_node(child, &mut consts, inst)?;
            }
            inst.push(Instruction::new(OpCode::Exit));
        },
        _ => {
            panic!("Unknown op: {}", node);
        }
    }
    Ok(())
}

pub fn compile(node: &Node) -> Result<Vec<u8>, PError> {
    let mut inst = Code::new();
    let mut consts = Mem::new();
    compile_node(node, &mut consts, &mut inst)?;
    let code = inst.to_bytes();
    let mem = consts.to_bytes();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&code);
    bytes.extend_from_slice(&mem);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use parser::Node;
    use lexer::Location;

    use super::*;

    fn compile_simple() {
        let mut node = Node::new(Op::Scope, Location::Eof);
        node.add(Node::new(Op::Value, Location::Eof).set_value(1.into()));
        let bytes = compile(&node).unwrap();
        assert_eq!(bytes, vec![3, 5]);
    }

    fn compile_binary_op() {
        let mut node = Node::new(Op::Scope, Location::Eof);
        let mut add = Node::new(Op::Add, Location::Eof);
        add.add(Node::new(Op::Value, Location::Eof).set_value(1.into()));
        add.add(Node::new(Op::Value, Location::Eof).set_value(2.into()));
        node.add(add);
        let bytes = compile(&node).unwrap();
        assert_eq!(bytes, vec![3, 2, 0, 6, 5]);
    }
}

