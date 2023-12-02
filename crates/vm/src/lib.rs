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
pub use instr::{Instr, Instrs};
pub use op_code::OpCode;
pub use vm::VM;
