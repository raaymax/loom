//mod code;
use std::{fmt::{Display, Formatter}};

use lexer::PError;
use parser::{Node, Op, Value};
use vm::{Instr, Instrs, OpCode};


struct Code {
    code: Vec<Instr>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            code: Vec::new(),
        }
    }
    pub fn push(&mut self, inst: Instr) -> usize {
        self.code.push(inst);
        self.code.len() - 1
    }
    pub fn append(&mut self, code: Code) {
        self.code.extend(code.code);
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        Instrs(self.code.iter().map(|c| c.byte_description()).collect()).into()
    }

    pub fn size(&self) -> usize {
        self.code.iter().map(|c| c.size()).sum()
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


struct Stack {
    stackptr: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stackptr: 0,
        }
    }
    pub fn push(&mut self, val: u32) -> usize {
        let location = self.stackptr;
        self.stackptr += 4;
        location
    }
    pub fn pop(&mut self) {
        self.stackptr -= 4;
    }
}

pub enum Addr {
    Mem(usize),
    MemPlaceholder(String),
    Stack(usize),
    StackPlaceholder(String),
}

pub struct Compiler {
    stack: Stack,
    mem: Mem,
    code: Code,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            stack: Stack::new(),
            mem: Mem::new(),
            code: Code::new(),
        }
    }
    pub fn compile_node(&mut self, node: &Node) -> Result<Code, PError> {
        match node.op {
            Op::Add => {
                let mut code = Code::new();
                code.append(self.compile_node(&node.children[0])?);
                code.append(self.compile_node(&node.children[1])?);
                code.push(Instr::Pop(1));
                code.push(Instr::Pop(2));
                code.push(Instr::Add(0,1,2));
                code.push(Instr::Push(0));
                Ok(code)
            },
            Op::Sub => {
                let mut code = Code::new();
                code.append(self.compile_node(&node.children[0])?);
                code.append(self.compile_node(&node.children[1])?);
                code.push(Instr::Pop(1));
                code.push(Instr::Pop(2));
                code.push(Instr::Sub(0,2,1));
                Ok(code)
            },
            Op::Value => {
                let mut code = Code::new();
                match &node.value {
                    Some(Value::Number(0)) => code.push(Instr::Load0(0)),
                    Some(Value::Number(1)) => code.push(Instr::Load1(0)),
                    Some(Value::Number(val)) => {
                        code.push(Instr::Load(0, *val as u16))
                    },
                    Some(..) => {
                        panic!("Unknown value: {}", node)
                    },
                    None => panic!("No value"),
                };
                code.push(Instr::Push(0));
                Ok(code)
            },
            Op::Branch => {
                let mut code = Code::new();
                code.append(self.compile_node(&node.children[0])?);
                code.push(Instr::Pop(1));
                code.push(Instr::Load0(0));
                let then_body = self.compile_node(&node.children[1])?;
                let else_body = self.compile_node(&node.children[2])?;
                let jmp = Instr::Jmp((else_body.size() as u16) as i32);
                println!("thenBody: {} {}", then_body.size(), jmp.size());
                code.push(Instr::Beq(0, 1, then_body.size() as i16 + jmp.size() as i16));
                code.append(then_body);
                code.push(jmp);
                code.append(else_body);
                Ok(code)
            },
            Op::While => {
                let mut code = Code::new();
                let mut condition = self.compile_node(&node.children[0])?;
                let body = self.compile_node(&node.children[1])?;
                condition.push(Instr::Pop(1));
                condition.push(Instr::Load0(0));
                let beq = Instr::Beq(0, 1, body.size() as i16);
                let jmp = Instr::Jmp(-(condition.size() as i32+ beq.size() as i32 + body.size() as i32));
                code.append(condition);
                code.push(beq);
                code.append(body);
                code.push(jmp);
                Ok(code)
            },
            Op::Assign => {
                let mut code = Code::new();
                code.append(self.compile_node(node.right().unwrap())?);
                code.push(Instr::Pop(1));
                Ok(code)
            },
            Op::Paren => {
                self.compile_node(node.left().unwrap())
            },
            Op::Scope => {
                let mut code = Code::new();
                for child in &node.children {
                    println!("child: {}", child.op);
                    code.append(self.compile_node(child)?);
                }
                Ok(code)
            },
            _ => {
                panic!("Unknown op: {} {}", node, node.op);
            }
        }
    }
    pub fn compile(&mut self, node: &Node) -> Result<Vec<u8>, PError> {
        let code = self.compile_node(node)?;
        self.code.append(code);
        self.code.push(Instr::Exit);
        let code = self.code.to_bytes();
        Ok(code)
    }
}
