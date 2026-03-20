use std::fs::File;
use std::io;
use std::path::Path;
use std::mem::take;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Number(i64),
    Atomic(char),
    Operator(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Experssion {
    Atomic(char),
    Operation(char, Vec<Experssion>)
}

pub struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    pub fn new() -> Self {
        Lexer {
            tokens: Vec::new(),
        }
    }
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwarp_or(Token::Eof);
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof);
    }

    pub fn tokenize(&mut self, input: Vec<String>) -> Vec<Token> {
         for s in input {
            let mut iter = s.chars().peekable();
            
            while let Some(&c) = iter.peek() {
                match c {
                    _ if c.is_whitespace() => {
                        iter.next();
                    }

                    '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                        /* let mut number = String::new();
                        while let Some(&nc) = iter.peek() {
                           if nc.is_alphanumeric() {
                               number.push(iter.next().unwrap());
                           } else {
                            break;
                           }
                        }
                        let num_value: i64 = number.parse().unwrap();
                        self.tokens.push(Token::Number(num_value));
                        */
                        self.tokens.push(Token::Atomic(iter.next().unwrap()));

                    }
                    
                    '+' | '-' | '*'| '/' => {
                        self.tokens.push(Token::Operator(iter.next().unwrap()));
                    }

                    _ => {
                        println!("Error: Unknown character {}", iter.next().unwrap());
                        break;                  
                    }
                }
            }
         }
        self.tokens.reverse();
        take(&mut self.tokens)
    }
}

fn binding_pow(operation: char) -> (f32, f32){
    match op {
        '+' | '-' => (1.0, 2.0),
        '*' | '/' => (3.0, 4.0),
        _ => println!("Unknown operator {:?}", op);
    }
}
fn parse_experssion(lexer: &mut Lexer, min_bp: f32) -> Experssion {
    let mut left = match lexer.next() {
        Token::Atomic(c) => Experssion::Atomic(c), 
        _ => println!("Bad token {:?}", c),
    };  
    loop {
        let operator = match.peek() {
            Token::Eof => return left, 
            Token::Operation(c) => c, 
            _ => println!("Bad token {:?}", c),
        }

        let (lbp, rbp)  = binding_pow(c);
        if (lbp < min_bp) {
            break;
        }

        lexer.next();

        let right = parse_expression(lexer, rbp);
        left = Experssion(operator, vec![left, right]);
    } 
    left
}

pub fn read(path: impl AsRef<Path>) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
   
    reader.lines().collect()
}

pub fn scan(strings: Vec<String>) {
     for s in strings {
         let mut iter = s.chars().peekable();

         while let Some(&c) = iter.peek() {
             match c {
                 _ if c.is_whitespace() => {
                     iter.next();
                 }

                 '0'..='9' => {

                     let mut number  = String::new();
                     while let Some(&nc) = iter.peek() {
                         if nc.is_numeric() { 
                            number.push(iter.next().unwrap()); 
                         } else {
                             break;
                         }
                     }
                     let num_value: i64 = number.parse().unwrap();
                     let tok = Token::Number(num_value);
                     println!("Token: Number ({:?})", tok);
                }
                '+' | '-' | '*' | '/' => {
                    let tok = Token::Operator(iter.next().unwrap());
                    println!("Token: Operator{:?}", tok);
                }
                _ => {
                    println!("Error: Unknown character {}", iter.next().unwrap());
                    break;
                }
            }
      } 
    }
}
