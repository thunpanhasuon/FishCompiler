use std::fs::File;
use std::io;
use std::path::Path;
use std::mem::take;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Number(i64),
    Operator(char),
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
    pub fn tokenize(&mut self, input: Vec<String>) -> Vec<Token> {
         for s in input {
            let mut iter = s.chars().peekable();
            
            while let Some(&c) = iter.peek() {
                match c {
                    _ if c.is_whitespace() => {
                        iter.next();
                    }

                    '0'..='9' => {
                        let mut number = String::new();
                        while let Some(&nc) = iter.peek() {
                           if nc.is_alphanumeric() {
                               number.push(iter.next().unwrap());
                           } else {
                            break;
                           }
                        }
                        let num_value: i64 = number.parse().unwrap();
                        self.tokens.push(Token::Number(num_value));
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
        take(&mut self.tokens)
    }
}
/* create tokens */
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
