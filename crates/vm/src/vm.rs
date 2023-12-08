use std::fmt::Display;

use lexer::PError;
use crate::{OpCode, Instr, Instrs};


macro_rules! parse_instr {
    ($val:ident; target value -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let mask2: u32 = (1 << 16) - 1;
            let target = (($val >> 16) & mask1) as u8;
            let value = ($val & mask2) as u16;
            Some(Instr::$name(target, value))
        }
    };
    ($val:ident; target -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let target = (($val >> 16) & mask1) as u8;
            Some(Instr::$name(target))
        }

    };
    ($val:ident; target v1 v2 -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let target = (($val >> 16) & mask1) as u8;
            let v1= (($val >> 8) & mask1) as u8;
            let v2 = ($val & mask1) as u8;
            Some(Instr::$name(target,v1, v2))
        }
    };
    ($val:ident; target v1 -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let target = (($val >> 16) & mask1) as u8;
            let v1= (($val >> 8) & mask1) as u8;
            Some(Instr::$name(target,v1))
        }
    };
}

pub struct VM {
    prog: Vec<u8>,
    regs: Vec<u32>,
    stack: Vec<u32>,
    pc: usize,
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VM: regs: {:?}, stack: {:?}", self.regs, self.stack)
    }
}



impl VM {
    pub fn new(prog: Vec<u8>) -> VM {
        VM {
            prog,
            regs: vec![0; 16],
            stack: Vec::new(),
            pc: 0,
        }
    }


    fn next(&mut self) -> Option<Instr> {
        println!("next: {} {:?} size: {}", self.pc, Instr::from_bytes(&self.prog, self.pc), Instr::from_bytes(&self.prog, self.pc).unwrap().size());

        Instr::from_bytes(&self.prog, self.pc).map(|i| {
            self.pc += i.size();
            i
        })
    }

    pub fn run(&mut self) -> Result<u32, PError> {
        let mut pc = 0;
        println!("prog len: {}", self.prog.len());
        while pc < self.prog.len() {
            let Some(inst) = Instr::from_bytes(&self.prog, pc) else {
                break;
            };
            println!("{}: {}", pc, inst);
            pc += inst.size();
        }
        while let Some(inst) = self.next() {
            println!("inst: {} {}", self.pc - inst.size(), inst);
            match inst {
                Instr::Sub(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] - self.regs[v2 as usize];
                },
                Instr::Add(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] + self.regs[v2 as usize];
                },
                Instr::Mul(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] * self.regs[v2 as usize];
                },
                Instr::Div(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] / self.regs[v2 as usize];
                },
                Instr::Mod(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] % self.regs[v2 as usize];
                },
                Instr::And(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] & self.regs[v2 as usize];
                },
                Instr::Or(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] | self.regs[v2 as usize];
                },
                Instr::Xor(target, v1, v2) => {
                    self.regs[target as usize] = self.regs[v1 as usize] ^ self.regs[v2 as usize];
                },
                Instr::Not(target, v1) => {
                    self.regs[target as usize] = !self.regs[v1 as usize];
                },

                Instr::Load(target, val) => {
                    self.regs[target as usize] = val as u32;
                },
                Instr::Load0(target) => {
                    self.regs[target as usize] = 0;
                },
                Instr::Load1(target) => {
                    self.regs[target as usize] = 1;
                },
                Instr::Exit => {
                    println!("Exit: {}", self);
                    return Ok(self.regs[0]);
                },
                Instr::Mov(target, v1) => {
                    self.regs[target as usize] = self.regs[v1 as usize];
                },
                Instr::Jmp(adr) => {
                    self.pc += adr as usize;
                },
                Instr::Beq(r1, r2, adr) => {
                    if self.regs[r1 as usize] == self.regs[r2 as usize] {
                        self.pc += adr as usize;
                    }
                },
                Instr::Bne(r1, r2, adr) => {
                    if self.regs[r1 as usize] != self.regs[r2 as usize] {
                        self.pc = adr as usize;
                    }
                },
                Instr::Push(r1) => {
                    self.stack.push(self.regs[r1 as usize]);
                },
                Instr::Pop(r1) => {
                    self.regs[r1 as usize] = self.stack.pop().unwrap();
                },
                Instr::Movs(r1, sp) => {
                    self.regs[r1 as usize] = self.stack[sp as usize];
                },
                _ => {
                    panic!("Unknown op code: {}", inst.op_code());
                }
            }
            println!("regs: {:?}", self.regs);
            println!("stack: {:?}\n", self.stack);
        }
        println!("{}", self.pc);
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! code {
        [$($x:ident$(($($arg:expr),*))?),+] => {
            {
                let c = Instrs(vec![
                    $(
                        Instr::$x$(($($arg),*))?.byte_description()
                    ),+
                ]);
                c.into()
            }
        };
    }

    #[test]
    fn instr() {
        let i0:(u8,[u8;4]) = Instr::Load(1, 1).byte_description();
        let i1:(u8,[u8;4]) = Instr::Load0(1).byte_description();
        let i2:(u8,[u8;4]) = Instr::Load1(1).byte_description();
        let i3:(u8,[u8;4]) = Instr::Exit.byte_description();
        assert_eq!(i3, (1, [0,0,0,0]));
        assert_eq!(i1, (2, [0b01000001,1,0,0]));
        assert_eq!(i2, (2, [0b01000010,1,0,0]));
        assert_eq!(i0, (4, [0b11000001,1,0,1]));
    }

    #[test]
    fn code() {
        let bytecode: Vec<u8> = code![Load(1,3), Load0(1), Load1(1), Exit];
        assert_eq!(bytecode, vec![0xC1,1,0,3,0x41,1,0x42,1,0x00]);

    }

    #[test]
    fn vm() {
        let bytes:Vec<u8> = code![Load(0,7) , Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn add() {
        let bytes:Vec<u8> = code![Load(1, 3), Load(2, 4), Add(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn mov() {
        let bytes:Vec<u8> = code![Load(1, 7), Mov(0, 1), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn sub() {
        let bytes:Vec<u8> = code![Load(1, 3), Load(2, 2), Sub(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn mul() {
        let bytes:Vec<u8> = code![Load(1, 3), Load(2, 2), Mul(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn div() {
        let bytes:Vec<u8> = code![Load(1, 6), Load(2, 2), Div(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn modulo() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(2, 2), Mod(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn and() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(2, 2), And(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn or() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(2, 2), Or(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn xor() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(2, 2), Xor(0,1,2), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn not() {
        let bytes:Vec<u8> = code![Load(1, 7), Not(0,1), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), !7);
    }

    #[test]
    fn beq_neg() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(0,1), Beq(0,1,16), Add(0,0,1), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 8);
    }
    #[test]
    fn beq_pos() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(0,7), Beq(0,1,4), Add(0,0,1), Exit];
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }
    #[test]
    fn jmp() {
        let bytes:Vec<u8> = code![Load(1, 7), Load(0,1), Jmp(4), Add(0,0,1), Exit];
        println!("{:?}", bytes);
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn push_pop() {
        let bytes:Vec<u8> = code![Load(1, 7), Push(1), Pop(0), Exit];
        println!("{:?}", bytes);
        let mut vm = VM::new(bytes);
        let result = vm.run();
        assert_eq!(result.unwrap(), 7);
    }
}
