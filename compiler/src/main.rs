mod lexer;
mod gen_arm;

use crate::gen_arm::ArmRegisterAllocator;
use crate::gen_arm::cgen_arm64;
use crate::lexer::pipeline;
use crate::lexer::parse_all;
use crate::lexer::eval;

use std::path::Path;
use std::env;

fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());
    let path = "program3.fh";

    if Path::new(path).exists() {
        /* parser pipeline */
        let mut parser = pipeline(path).expect("Failed to read file");

        /* build ast */
        //let expr = parse_experssion(&mut parser,0.0);


        /* build ast && eval */
        let exprs = parse_all(&mut parser);

        for eval_syntax in &exprs {
            let _eval = eval(&eval_syntax);
            println!("{}", _eval);
        } 

        /* magic :) */ 
        let mut alloc = ArmRegisterAllocator::new(); 
        for expr in &exprs {
            let result_res = alloc.arm64(&expr);
            alloc.free(result_res);
        }

        let _ = cgen_arm64(&alloc.arm64_instruction());

    } else {
        println!("Error: cannot find file at {:?}", path);
    }

}
