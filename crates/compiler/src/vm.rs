use crate::OpCode;
use lexer::PError;
macro_rules! make_instr {
    ($c:expr; $($name:ident[$a:expr;$b:expr]),+; $($n:ident = $dec:expr),*) => {
        {
            $(
                let $n = $dec;
            )*
            let mut result: u32 = 0;
            $(
                result |= {
                    let mask: u32 = (1 << ($a)-($b)+1) - 1;
                    (mask & $name as u32) << $b
                };
            )+
            ($c, result.to_be_bytes())
        }
    };
}

macro_rules! make_instr_set {
    ($p:ident; $($inst:ident($($arg:ty),*; $c:expr; op[$a0:expr;$b0:expr] = $opv:expr; $($name:ident[$a:expr;$b:expr]),*; $($n:ident = $dec:expr),*)),+ ) => {
        enum $p {
            $(
                $inst($($arg),*)
            ),+
        }
        impl $p{
            fn get_op(&self) -> u8 {
                match self {
                    $(
                        $p::$inst(..) => $opv,
                    )+
                }
            }

            fn byte_description(self) -> (u8, [u8;4]) {
                match self {
                    $(
                        $p::$inst($($name),*) => {
                            let op = $opv;
                            make_instr!($c; op[$a0;$b0], $($name[$a;$b]),*; $($n = $dec),*)
                        },
                    )+
                }
            }
        }
    };
}

make_instr_set!(
    Lum32;
    Load(u8,u16; 4; op[29;24]= 0b11000001; target[23;16], val[15;0];),
    Load0(u8; 2; op[31;24] = 0b01000001; target[23;16];),
    Load1(u8; 2; op[31;24] = 0b01000010; target[23;16];),
    Exit(u8; 1; op[31;24] = 0b00000000; empty[1;0];),
    Store(u8; 2; op[31;24] = 0b01000011; target[23;16];)
);


enum Instr {
    Load(u8,u16),
    Load0(u8),
    Load1(u8),
    Store(u8),
    Exit,
}

/*
macro_rules! make_instr_parser {
    ($fn_name:ident; $($name:ident[$a:expr;$b:expr]),+) => {
            $(
                let $name: u32 = {
                    let mask: u32 = (1 << ($a)-($b)+1) - 1;
                    let c = val >> $b;
                    c & mask;
                };
            )+
            match op {
                0b11000001 => Instr::Load(target, val),
                0b01000001 => Instr::Load0(target),
                0b01000010 => Instr::Load1(target),
                0b00000000 => Instr::Exit,
                0b01000011 => Instr::Store(target),
                _ => panic!("Unknown op code: {}", op),
            }
        }
    };
}
*/
impl Instr {
    fn byte_description(self) -> (u8, [u8;4]) {
        match self {
            Instr::Load(target, val) => make_instr!(4; op[29;24], target[23;16], val[15;0]; op = 0b11000001),
            Instr::Load0(target) => make_instr!(2; op[31;24], target[23;16]; op = 0b01000001),
            Instr::Load1(target) => make_instr!(2; op[31;24], target[23;16]; op = 0b01000010),
            Instr::Exit => make_instr!(1; op[31;24]; op = 0b00000000),
            Instr::Store(target) => make_instr!(2; op[31;24], target[23;16]; op = 0b01000011),
        }
    }
}

struct Instrs (Vec<(u8,[u8;4])>);

impl Into<Vec<u8>> for Instrs {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (count, instr) in self.0.iter() {
            for b in 0..(*count as usize) {
                bytes.push(instr[b]);
            }
        }
        bytes
    }
}


pub struct VM {
    regs: Vec<u32>,
    stack: Vec<u32>,
    pc: usize,
}


impl VM {
    pub fn new() -> VM {
        VM {
            regs: Vec::new(),
            stack: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self, bytes: Vec<u8>) -> Result<u32, PError> {
        let byte = bytes[self.pc];

        let op_code = OpCode::from(byte);
        match op_code {
            OpCode::Load1 => {
                self.regs[0] = 1;
            },
            OpCode::Exit => {
                return Ok(self.regs[0]);
            },
            _ => {
                panic!("Unknown op code: {}", op_code);
            }
        }
        
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
                        Lum32::$x$(($($arg),*))?.byte_description()
                    ),+
                ]);
                c.into()
            }
        };
    }

    #[test]
    fn test_instr() {
        let i0:(u8,[u8;4]) = Instr::Load(1, 1).byte_description();
        let i1:(u8,[u8;4]) = Instr::Load0(1).byte_description();
        let i2:(u8,[u8;4]) = Instr::Load1(1).byte_description();
        let i3:(u8,[u8;4]) = Instr::Exit.byte_description();
        assert_eq!(i0, (4, [193,1,0,1]));
        assert_eq!(i1, (2, [65,1,0,0]));
        assert_eq!(i2, (2, [66,1,0,0]));
        assert_eq!(i3, (1, [0,0,0,0]));
    }

    #[test]
    fn test_code() {
        let bytecode: Vec<u8> = code![Load(1,1), Load0(1), Load1(1), Exit(1)];
        assert_eq!(bytecode, vec![1,1,0,1,2,1,3,1,4]);

    }

    #[test]
    fn test_vm() {
        let mut vm = VM::new();
        let bytes:Vec<u8> = code![Load(1,1), Load0(1), Load1(1), Exit(1)];
        let result = vm.run(bytes);
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_stack_1() {
        let mut vm = VM::new();
        let bytes:Vec<u8> = code![Load1(1), Store(1), Exit(1)];
        let result = vm.run(bytes);
        assert_eq!(vm.stack, vec![1]);
    }
}
