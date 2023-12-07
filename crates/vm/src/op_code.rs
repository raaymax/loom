#[derive(Debug, Clone, Copy, FromPrimitive, EnumDisplay)]
#[repr(u8)]
pub enum OpCode {
    Load  = 0b11000001,
    Add   = 0b11000010,
    Sub   = 0b11000011,
    Mul   = 0b11000100,
    Div   = 0b11000101,
    Mod   = 0b11000110,
    And   = 0b11000111,
    Or    = 0b11001000,
    Xor   = 0b11001001,
    Beq   = 0b11001010,
    Bne   = 0b11001011,
    Jmp   = 0b11001100,
    Movs  = 0b11001101,

    Mov   = 0b10000001,
    Not   = 0b10000010,

    Load0 = 0b01000001,
    Load1 = 0b01000010,
    Store = 0b01000011,
    Push  = 0b01000100,
    Pop   = 0b01000101,

    Exit  = 0b00000000,


}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        let element = num::FromPrimitive::from_u8(byte);
        match element {
            Some(op) => op,
            None => panic!("Unknown op code: {}", byte)
        }
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        val as u8
    }
}

impl OpCode {
    pub fn as_u8(&self) -> u8 {
        (*self).into()
    }

    pub fn size(&self) -> usize {
       let c: u8 = (*self).into();
       (c >> 6) as usize + 1
    }
}
