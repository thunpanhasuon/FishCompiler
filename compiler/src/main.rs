mod lexer;
mod gen_arm;
use crate::gen_arm::ArmRegisterAllocator;
use crate::gen_arm::cgen_arm64;
use crate::lexer::read;
use crate::lexer::parse_experssion;
use crate::lexer::eval;
use std::env;

fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());

    let path = "program1.fh";
    
    if std::path::Path::new(path).exists() {
        let lines = read(path);
        let mut lexer_parse = lexer::Lexer::new();
        let tokens = lexer_parse.tokenize(lines.expect("only char allow"));

        let mut parser = lexer::Lexer::new(); 
        parser.load(tokens);

        let expr = parse_experssion(&mut parser, 0.0);

        let _eval = eval(&expr);

        let mut alloc = ArmRegisterAllocator::new();
        let result_reg = alloc.arm64(&expr);
        alloc.free(result_reg);
        
        let _ = cgen_arm64(&alloc.arm64_instruction());

    } else {
        println!("Error: Rust cannot find the file at {:?}", path);
    }

}
