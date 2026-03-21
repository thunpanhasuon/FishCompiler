mod lexer;
mod genArm;
use crate::lexer::read;
use crate::genArm::code_gen;
use crate::lexer::parse_experssion;
use crate::lexer::eval;
use std::env;

fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());

    let path = "program1.fsh";
    
    if std::path::Path::new(path).exists() {
        let lines = read(path);
        let mut lexer_parse = lexer::Lexer::new();
        let tokens = lexer_parse.tokenize(lines.expect("only char allow"));
        
        println!("Tokens: {:#?}", tokens);

        let mut parser = lexer::Lexer::new(); 
        parser.load(tokens);

        let expr = parse_experssion(&mut parser, 0.0);
        println!("Tree: {:#?}", expr);

        let eval = eval(&expr);

        println!("Result: {:#?}", eval);

        /* can't figure out code generation  */ 

        code_gen(&expr);
        
    } else {
        println!("Error: Rust cannot find the file at {:?}", path);
    }

}
