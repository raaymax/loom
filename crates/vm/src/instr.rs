use std::fmt::Display;

use crate::op_code::OpCode;

macro_rules! make_instr {
    ($c:expr; $($name:ident[$a:expr;$b:expr]),+ $(,)?; $($n:ident = $dec:expr),*) => {
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

macro_rules! parse_instr {
    ($val:ident; tr value -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let mask2: u32 = (1 << 16) - 1;
            let tr = (($val >> 16) & mask1) as u8;
            let value = ($val & mask2) as u16;
            Some(Instr::$name(tr, value))
        }
    };
    ($val:ident; tr -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let tr = (($val >> 16) & mask1) as u8;
            Some(Instr::$name(tr))
        }

    };
    ($val:ident; tr r1 r2 -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let tr = (($val >> 16) & mask1) as u8;
            let r1= (($val >> 8) & mask1) as u8;
            let r2 = ($val & mask1) as u8;
            Some(Instr::$name(tr,r1, r2))
        }
    };
    ($val:ident; tr r1 -> $name:ident) => {
        {
            let mask1: u32 = (1 << 8) - 1;
            let tr = (($val >> 16) & mask1) as u8;
            let v1= (($val >> 8) & mask1) as u8;
            Some(Instr::$name(tr,v1))
        }
    };
    ($val:ident; r1 r2 addr -> $name:ident) => {
        {
            let mask_r: u32 = (1 << 4) - 1;
            let mask16: u32 = (1 << 16) - 1;
            let r1 = (($val >> 20) & mask_r) as u8;
            let r2 = (($val >> 16) & mask_r) as u8;
            let addr = ($val & mask16) as i16;
            Some(Instr::$name(r1, r2, addr))
        }
    };
    ($val:ident; adr -> $name:ident) => {
        {
            let neg_mask: u32 = 1 << 23;
            let neg: u32 = (-1i32 << 23) as u32;
            let mask1: u32 = (1 << 24) - 1;


            //FIXME: this is terrible but I don't know how to handle it better
            let adr= ($val & mask1) as u32;
            let adr = if (adr & neg_mask) != 0 {
                (neg | adr) as i32
            } else {
                adr as i32
            };
            Some(Instr::$name(adr))
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instr {
    Load(u8,u16),
    Add(u8,u8,u8),
    Load0(u8),
    Load1(u8),
    Store(u8),
    Exit,
    Mov(u8,u8),
    Sub(u8,u8,u8),
    Mul(u8,u8,u8),
    Div(u8,u8,u8),
    Mod(u8,u8,u8),
    And(u8,u8,u8),
    Or(u8,u8,u8),
    Xor(u8,u8,u8),
    Not(u8,u8),
    Beq(u8,u8,i16),
    Bne(u8,u8,i16),
    Jmp(i32),
}

impl From<&Instr> for OpCode {
    fn from(val: &Instr) -> Self {
        match val {
            Instr::Load(..) => OpCode::Load,
            Instr::Load0(..) => OpCode::Load0,
            Instr::Load1(..) => OpCode::Load1,
            Instr::Exit => OpCode::Exit,
            Instr::Store(..) => OpCode::Store,
            Instr::Add(..) => OpCode::Add,
            Instr::Mov(..) => OpCode::Mov,
            Instr::Sub(..) => OpCode::Sub,
            Instr::Mul(..) => OpCode::Mul,
            Instr::Div(..) => OpCode::Div,
            Instr::Mod(..) => OpCode::Mod,
            Instr::And(..) => OpCode::And,
            Instr::Or(..) => OpCode::Or,
            Instr::Xor(..) => OpCode::Xor,
            Instr::Not(..) => OpCode::Not,
            Instr::Beq(..) => OpCode::Beq,
            Instr::Bne(..) => OpCode::Bne,
            Instr::Jmp(..) => OpCode::Jmp,
        }
    }
}

impl Instr {
    pub fn byte_description(self) -> (u8, [u8;4]) {
        match self {
            Instr::Load(tr, val) => make_instr!(4; op[31;24], tr[23;16], val[15;0]; op = OpCode::Load),
            Instr::Load0(tr) => make_instr!(2; op[31;24], tr[23;16]; op = OpCode::Load0),
            Instr::Load1(tr) => make_instr!(2; op[31;24], tr[23;16]; op = OpCode::Load1),
            Instr::Exit => make_instr!(1; op[31;24]; op = OpCode::Exit),
            Instr::Store(tr) => make_instr!(2; op[31;24], tr[23;16]; op = OpCode::Store),
            Instr::Add(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Add),
            Instr::Mov(tr, r1) => make_instr!(3; op[31;24], tr[23;16], r1[15;8]; op = OpCode::Mov),
            Instr::Sub(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Sub),
            Instr::Mul(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Mul),
            Instr::Div(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Div),
            Instr::Mod(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Mod),
            Instr::And(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::And),
            Instr::Or (tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Or),
            Instr::Xor(tr, r1, r2) => make_instr!(4; op[31;24], tr[23;16], r1[15;8], r2[7;0]; op = OpCode::Xor),
            Instr::Not(tr, r1) => make_instr!(3; op[31;24], tr[23;16], r1[15;8]; op = OpCode::Not),
            Instr::Beq(r1,r2,addr) => make_instr!(4; op[31;24], r1[23;20], r2[19;16], addr[15;0]; op = OpCode::Beq),
            Instr::Bne(r1,r2,addr) => make_instr!(4; op[31;24], r1[23;20], r2[19;16], addr[15;0]; op = OpCode::Bne),
            Instr::Jmp(addr) => make_instr!(4; op[31;24], addr[23;0]; op = OpCode::Jmp),
        }
    }

    pub fn op_code(&self) -> OpCode {
        self.into()
    }

    pub fn from_bytes(data: &[u8], pc: usize) -> Option<Self> {
        let op = data[pc];
        let code: OpCode = op.into();
        let mut bytes = vec![0;4];
        bytes[0] = op;
        for i in 1..code.size() {
            bytes[i] = data[pc+i];
        }

        let val = u32::from_be_bytes(bytes.try_into().unwrap());
        match code {
            OpCode::Load => parse_instr!(val; tr value -> Load),
            OpCode::Load0 => parse_instr!(val; tr -> Load0),
            OpCode::Load1 => parse_instr!(val; tr -> Load1),
            OpCode::Exit=> Some(Instr::Exit),
            OpCode::Store => parse_instr!(val; tr -> Store),
            OpCode::Add=> parse_instr!(val; tr r1 r2 -> Add),
            OpCode::Mov=> parse_instr!(val; tr r1 -> Mov),
            OpCode::Sub=> parse_instr!(val; tr r1 r2 -> Sub),
            OpCode::Mul=> parse_instr!(val; tr r1 r2 -> Mul),
            OpCode::Div=> parse_instr!(val; tr r1 r2 -> Div),
            OpCode::Mod=> parse_instr!(val; tr r1 r2 -> Mod),
            OpCode::And=> parse_instr!(val; tr r1 r2 -> And),
            OpCode::Or=> parse_instr!(val; tr r1 r2 -> Or),
            OpCode::Xor=> parse_instr!(val; tr r1 r2 -> Xor),
            OpCode::Not=> parse_instr!(val; tr r1 -> Not),
            OpCode::Beq=> parse_instr!(val; r1 r2 addr -> Beq),
            OpCode::Bne=> parse_instr!(val; r1 r2 addr -> Bne),
            OpCode::Jmp=> parse_instr!(val; adr -> Jmp),
        }
    }

    pub fn size(&self) -> usize {
        self.op_code().size()
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Load(tr, val) => write!(f, "Load r{} {}", tr, val),
            Instr::Load0(tr) => write!(f, "Load0 r{}", tr),
            Instr::Load1(tr) => write!(f, "Load1 r{}", tr),
            Instr::Exit => write!(f, "Exit"),
            Instr::Store(tr) => write!(f, "Store r{}", tr),
            Instr::Add(tr, v1, v2) => write!(f, "Add r{} r{} r{}", tr, v1, v2),
            Instr::Mov(tr, v1) => write!(f, "Mov r{} r{}", tr, v1),
            Instr::Sub(tr, v1, v2) => write!(f, "Sub r{} r{} r{}", tr, v1, v2),
            Instr::Mul(tr, v1, v2) => write!(f, "Mul r{} r{} r{}", tr, v1, v2),
            Instr::Div(tr, v1, v2) => write!(f, "Div r{} r{} r{}", tr, v1, v2),
            Instr::Mod(tr, v1, v2) => write!(f, "Mod r{} r{} r{}", tr, v1, v2),
            Instr::And(tr, v1, v2) => write!(f, "And r{} r{} r{}", tr, v1, v2),
            Instr::Or(tr, v1, v2) => write!(f, "Or r{} r{} r{}", tr, v1, v2),
            Instr::Xor(tr, v1, v2) => write!(f, "Xor r{} r{} r{}", tr, v1, v2),
            Instr::Not(tr, v1) => write!(f, "Not r{} r{}", tr, v1),
            Instr::Beq(r1,r2,addr) => write!(f, "Beq r{} r{} {}", r1, r2, addr),
            Instr::Bne(r1,r2,addr) => write!(f, "Bne r{} r{} {}", r1, r2, addr),
            Instr::Jmp(val) => write!(f, "Jmp {}", val),
        }
    }
}

pub struct Instrs (pub Vec<(u8,[u8;4])>);

impl From<Instrs> for Vec<u8> {
    fn from(val: Instrs) -> Self {
        let mut bytes = Vec::new();
        for (count, instr) in val.0.iter() {
            for b in 0..(*count as usize) {
                bytes.push(instr[b]);
            }
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_instr{
        ($test:ident; $name:ident$(($($args:expr),+))?; $size: expr) => {
            #[test]
            fn $test() {
                
                let instrs = Instrs(vec![
                    Instr::$name$(($($args),+))?.byte_description()
                ]);
                let bytes: Vec<u8> = instrs.into();
                assert_eq!(Instr::from_bytes(&bytes, 0), Some(Instr::$name$(($($args),+))?));
                assert_eq!(Instr::$name$(($($args),+))?.size(), $size);
            }
        };
    }

    check_instr!(load; Load(1,2); 4);
    check_instr!(load0; Load0(1); 2);
    check_instr!(load1; Load1(1); 2);
    check_instr!(exit; Exit; 1);
    check_instr!(store; Store(1); 2);
    check_instr!(add; Add(1,2,3); 4);
    check_instr!(mov; Mov(1,2); 3);
    check_instr!(sub; Sub(1,2,3); 4);
    check_instr!(mul; Mul(1,2,3); 4);
    check_instr!(div; Div(1,2,3); 4);
    check_instr!(modulo; Mod(1,2,3); 4);
    check_instr!(and; And(1,2,3); 4);
    check_instr!(or; Or(1,2,3); 4);
    check_instr!(xor; Xor(1,2,3); 4);
    check_instr!(not; Not(1,2); 3);
    check_instr!(beq; Beq(1,2,3); 4);
    check_instr!(bne; Bne(1,2,3); 4);
    check_instr!(jmp; Jmp(3); 4);
    check_instr!(jmp_negative; Jmp(-3); 4);
}
