mod lexer;
use crate::lexer::read;
use crate::lexer::parse_experssion;
use crate::lexer::Lexer;
use std::env;

fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());

    let path = "program2.fsh";
    
    if std::path::Path::new(path).exists() {
        let lines = read(path);
        let mut lexer_parse = lexer::Lexer::new();
        let mut tokens = lexer_parse.tokenize(lines.expect("only char allow"));
        let mut parser = lexer::Lexer::new(); 
        parser.load(tokens);

        let expr = parse_experssion(&mut parser, 0.0);
        println!("Tree: {:#?}", expr);
        // scan(lines.expect("only char allow"));
        
    } else {
        println!("Error: Rust cannot find the file at {:?}", path);
    }

}
