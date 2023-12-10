use std::{fmt::{Display, Formatter}};

use lexer::PError;
use parser::{Node, Op, Value};
use vm::{Instr, Instrs, OpCode};
struct Code {
    code: Vec<VInst>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            code: Vec::new(),
        }
    }
    pub fn push(&mut self, inst: VInst) -> usize {
        self.code.push(inst);
        self.code.len() - 1
    }
    pub fn append(&mut self, code: Code) {
        self.code.extend(code.code);
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    pub fn size(&self) -> usize {
        self.code.iter().map(|c| c.op_code.size()).sum()
    }
}

enum VArg {
    Reg(u8),
    Rel(i32),
    Abs(u32),
}

enum VState {
    Incomplete,
    Complete,
}

struct VInst {
    state: VState,
    label: String,
    op_code: OpCode,
    args: Vec<VArg>,
    inst: Option<Instr>
}

impl VInst {
    pub fn new(op_code: OpCode, args: Vec<VArg>) -> VInst {
        VInst {
            state: VState::Incomplete,
            label: String::new(),
            op_code,
            args,
            inst: None,
        }
    }
    pub fn build(&mut self) {
        self.inst = Some(
            match self.op_code {
                OpCode::Load => Instr::Load(self.args[0], self.args[1]),
                _ => panic!("Unknown op code: {}", self.op_code)
            }
        );
        self.state = VState::Complete;
    }
}

#[cfg(test)]
mod compiler_code_tests {
    use super::*;

    #[test]
    fn test_code() {
        let mut code = Code::new();
        code.push(VInst::new(OpCode::Load, vec![VArg::Reg(0), VArg::Abs(0)]));
        assert!(code.size() == 4);
    }
}

