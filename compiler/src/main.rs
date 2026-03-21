mod lexer;
mod genArm;
use crate::genArm::cgen_arm64;
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
        
        println!("Tokens: {:#?}", tokens);

        let mut parser = lexer::Lexer::new(); 
        parser.load(tokens);

        let expr = parse_experssion(&mut parser, 0.0);
        println!("Tree: {:#?}", expr);

        let eval = eval(&expr);

        println!("Result: {:#?}", eval);

        /* code gen */
        cgen_arm64();


        
    } else {
        println!("Error: Rust cannot find the file at {:?}", path);
    }

}
