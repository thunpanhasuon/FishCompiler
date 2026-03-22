use std::io;
use std::path::Path;
use std::mem::take;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Atomic(char),
    Number(i64),
    Operator(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Experssion {
    Atomic(char),
    Number(i64),
    Operation(char, Vec<Experssion>),
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
    pub fn load(&mut self, tokener: Vec<Token>) {
        self.tokens = tokener; 
        self.tokens.reverse();
    } 


    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
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
                        let mut value = String::new();
                        while let Some(&nc) = iter.peek() {
                            if nc.is_alphanumeric() {
                                value.push(iter.next().unwrap());
                            } else {
                                break; 
                            }
                        }

                        let number: i64 = value.parse().unwrap(); 
                        self.tokens.push(Token::Number(number));

                    }

                    'a'..='z' | 'A'..='Z' => {
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
        take(&mut self.tokens)
    }
}

pub fn pipeline(path: impl AsRef<Path>) -> Result<Lexer, io::Error> {
    let (source_code, line_count) = read(path)?;
    println!("Read {} lines", line_count);

    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(vec![source_code]);

    lexer.load(tokens); 

    /* return the tokenize token lexer */
    Ok(lexer)

}
pub fn binding_pow(operation: char) -> (f32, f32){
    match operation {
        '+' | '-' => (1.0, 2.0),
        '*' | '/' => (3.0, 4.0),
        _ => panic!("Unknown operator {:?}", operation),
    }
}
pub fn parse_experssion(lexer: &mut Lexer, min_bp: f32) -> Experssion {
    let mut left = match lexer.next() {
        Token::Number(value) => Experssion::Number(value),
        Token::Atomic(c) => Experssion::Atomic(c), 
        t => panic!("Bad token {:?}", t),
    };
    loop {
        let operator = match lexer.peek() {
            Token::Eof => break, 
            Token::Operator(c) => c, 
            Token::Number(_) => break,
            Token::Atomic(_) => break,
        };

        let (lbp, rbp)  = binding_pow(operator);
        if lbp < min_bp {
            break;
        }

        lexer.next();

        let right = parse_experssion(lexer, rbp);
        left = Experssion::Operation(operator, vec![left, right]);
    } 
    left
}

pub fn read(path: impl AsRef<Path>) -> Result<(String, usize), io::Error> {
    let content = std::fs::read_to_string(path)?;
    let line =  content.lines().count();
    Ok((content, line))
}
/* future impermentation later  */

/* pub fn scan(strings: Vec<String>) {
     for s in strings {
         let mut iter = s.chars().peekable();

         while let Some(&c) = iter.peek() {
             match c {
                 _ if c.is_whitespace() => {
                     iter.next();
                 }

                 /* '0'..='9' => {

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
                */
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
*/
pub fn eval(expr: &Experssion) -> i64 {
    match expr {
       Experssion::Atomic(c)  => {
            if c.is_digit(10) {
                c.to_digit(10).unwrap() as i64
            } else {
                panic!("Unknown atomic value {:?}", c);
            }
       }

        Experssion::Number(value) => { *value }

        Experssion::Operation(op, operand) => {
            let left_val = eval(&operand[0]);
            let right_val = eval(&operand[1]); 
            match op {
                '+' => left_val + right_val, 
                '-' => left_val - right_val, 
                '*' => left_val * right_val,
                '/' => {
                    if right_val == 0 {
                        panic!("Eval division by zero");
                    } 
                    left_val / right_val
                }
                _ => panic!("Unknown operator {:?}", op), 
            }
        }

    }
}
