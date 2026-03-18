mod lexer;

use crate::lexer::read;
use crate::lexer::scan;
use std::env;

fn main() {
    
    /* debug this out */
    println!("reading from DIR: {:?}", env::current_dir().unwrap());

    let pathing = "program.fs";

    if std::path::Path::new(pathing).exists() {
        let lines = read(pathing);
        scan(lines.expect("only char allow"));
        
    } else {
        println!("Error: Rust cannot find the file at {:?}", pathing);
    }

}
