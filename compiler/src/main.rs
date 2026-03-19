mod lexer;
use crate::lexer::read;
use std::env;

fn main() {
    
    println!("reading from DIR: {:?}", env::current_dir().unwrap());

    let path = "program1.fsh";
    
    if std::path::Path::new(path).exists() {
        let lines = read(path);
        let mut lexer_parse = lexer::Lexer::new();
        let tokens = lexer_parse.tokenize(lines.expect("only char allow"));
        println!("Tokens: {:?}", tokens);
        // scan(lines.expect("only char allow"));
        
    } else {
        println!("Error: Rust cannot find the file at {:?}", path);
    }

}
