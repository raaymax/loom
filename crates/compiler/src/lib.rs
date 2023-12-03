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
    pub fn push(&mut self, inst: Instr) {
        self.code.push(inst);
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        Instrs(self.code.iter().map(|c| c.byte_description()).collect()).into()
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
            inst.push(Instr::Mov(2, 0));
            compile_node(&node.children[1], &mut consts, inst)?;
            inst.push(Instr::Mov(1, 0));
            inst.push(Instr::Add(0,1,2));
        },
        Op::Sub => {
            compile_node(&node.children[0], &mut consts, inst)?;
            inst.push(Instr::Mov(2, 0));
            compile_node(&node.children[1], &mut consts, inst)?;
            inst.push(Instr::Mov(1, 0));
            inst.push(Instr::Sub(0,2,1));
        },
        Op::Value => {
            match &node.value {
                Some(Value::Number(0)) => inst.push(Instr::Load0(0)),
                Some(Value::Number(1)) => inst.push(Instr::Load1(0)),
                Some(Value::Number(val)) => {
                    inst.push(Instr::Load(0, *val as u16));
                },
                Some(..) => {
                    panic!("Unknown value: {}", node);
                },
                None => panic!("No value"),
            }
        },
        Op::Scope | Op::Paren => {
            for child in &node.children {
                println!("child: {}", child.op);
                compile_node(child, &mut consts, inst)?;
            }
            inst.push(Instr::Exit);
        },
        _ => {
            panic!("Unknown op: {} {}", node, node.op);
        }
    }
    Ok(())
}

pub fn compile(node: &Node) -> Result<Vec<u8>, PError> {
    let mut inst = Code::new();
    let mut consts = Mem::new();
    compile_node(node, &mut consts, &mut inst)?;
    let code = inst.to_bytes();
    Ok(code)
}

#[cfg(test)]
mod tests {
    use parser::Node;
    use lexer::Location;

    use super::*;

    #[test]
    fn compile_simple() {
        let mut node = Node::new(Op::Scope, Location::Eof);
        node.add(Node::new(Op::Value, Location::Eof).set_value(1.into()));
        let bytes = compile(&node).unwrap();
        assert_eq!(bytes, vec![OpCode::Load1.into(), 0, OpCode::Exit.into()]);
    }

}

