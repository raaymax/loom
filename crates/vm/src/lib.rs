mod vm;
mod op_code;
mod instr;

extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate enum_display;

pub use instr::{Instr, Instrs};
pub use op_code::OpCode;
pub use vm::VM;
