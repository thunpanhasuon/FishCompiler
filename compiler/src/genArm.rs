
use std::sync::WaitTimeoutResult;

use crate::lexer::Experssion;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Register {
    R_0, 
}

fn register_for_atomic() -> Register {
    Register::R_0
}

pub fn code_gen(expr: &Experssion) {

}
