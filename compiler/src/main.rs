mod lexer;
mod gen_arm;

use crate::gen_arm::ArmRegisterAllocator;
use crate::gen_arm::cgen_arm64;
use crate::lexer::pipeline;
use crate::lexer::parse_experssion;
use crate::lexer::eval;

use std::path::Path;
use std::env;

/* todo Panha can you handle 2 line or multiple line read and parse all the expression without a oneliner */
fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());
    let path = "program1.fh";
    if Path::new(path).exists() {
        /* parser pipeline */
        let mut parser = pipeline(path).expect("Failed to read file");

        /* build ast */
        let expr = parse_experssion(&mut parser,0.0);

        /* eval */
        let _eval = eval(&expr);

        /* magic :) */
        let mut alloc = ArmRegisterAllocator::new(); 
        let result_res = alloc.arm64(&expr);
        alloc.free(result_res);

        let _ = cgen_arm64(&alloc.arm64_instruction());

    } else {
        println!("Error: cannot find file at {:?}", path);
    }

}
