mod value;
mod ast_node;
mod ast;
mod block;
mod expr;
mod branch;

use std::slice::Iter;

use crate::loc::Location;
use crate::token::Token;
use crate::errors::PError;

pub use self::ast::build;
pub use value::Value;
pub use self::ast_node::{Node, Op};

pub fn parse(iter: &mut Iter<Token>) -> Result<Node, PError> {
    build(iter, 0, Location::new_point(0,0,0))
}
