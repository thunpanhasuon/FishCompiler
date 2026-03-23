use core::panic;
use std::io;
use std::path::Path;
use std::mem::take;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    ComputerCompute,

    Atomic(char),
    Number(i64),
    Operator(char),
    Assign,
    /* trolling >:) */
    Semicolon,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Experssion {
    Atomic(char),
    Number(i64),
    Operation(char, Vec<Experssion>),
    Assign(char, Box<Experssion>),
}

#[derive(Debug, Clone, PartialEq)]
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
                    ' ' | '\t' | '\n' | '\r' => {
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

                    'A'..='Z' => {

                        let mut str =  String::new();
                        while let Some(&nc) = iter.peek()  {
                            if nc.is_alphabetic() {
                            str.push(iter.next().unwrap());
                            } else {
                                break;
                            }
                        } 
                         

                        match str.as_ref() {
                            "FISHCOMPUTE" => { 
                                self.tokens.push(Token::ComputerCompute); 
                                iter.next(); 
                            }
                            _ => println!("Unknown keyword"),
                        };

                    }
                    'a'..='z' => {
                        self.tokens.push(Token::Atomic(iter.next().unwrap()));
                    }
                    '='  => {
                        self.tokens.push(Token::Assign);
                        iter.next();
                    }
                    '!' => { 
                        self.tokens.push(Token::Semicolon);
                        iter.next();
                    }
                    '+' | '-' | '*'| '/'  => {
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

/* parsing varibale: 
 * 
 * lexer will look a char a head a (=) a + b
 * lexer will eat the char a head (a=) a + b
 * Enter recurssion
 * 
 * Example complex x = 10 + 4 * 5 / 2!
 *
 * expr(0.0)    
 *         left = 10 
 *         peek '+' 1.0 < 0.0 (false keep going)
 *                    expr(2.0)
 *                              left = 4 
 *                              peek '*'    3.0 < 2.0 (flase keep going)
 *                                          expr(4.0)
 *                                                  left = 5 
 *                                                  peek '/' 3.0 < 4.0 (true done go back up)
 * 
 *                                                  right = 5
 *                                                  [*, (4, 5)]
 *                              peek '/'
 *                                          3.0 < 2.0
 *                                          expr(4.0) 3.0 < 4  (true done go back up)  
 *                                                  left = 2 
 *                             [/, 2 (*, [4, 5])]  
 *               
 *     [+, 10, (/, (*, [4, 5]), 2)] 
 */
pub fn parse_experssion(lexer: &mut Lexer, min_bp: f32) -> Experssion {
    let mut left = match lexer.next() {
        Token::Number(value) => Experssion::Number(value),
        Token::ComputerCompute => {
            if let Token::Atomic(c) = lexer.peek() {
                lexer.next();
                if lexer.peek() == Token::Assign {
                    lexer.next();
                    let value = parse_experssion(lexer, 0.0); 
                    println!("where is c {}", c);
                    return Experssion::Assign(c, Box::new(value));
                } else {
                    Experssion::Atomic(c)
                }
            } else {
                panic!("Wopp");                
            }
        }
        t => panic!("Expected operator, found {:?}", t),
    };

    loop {
        let operator = match lexer.peek() {
            Token::Eof => break, 
            //Token::Number(_) | Token::Atomic(_) => break,
            Token::Semicolon => break,
            Token::Operator(c) => c, 
            t => panic!("Expected operator, found {:?}", t),
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
pub fn parse_all(lexer: &mut Lexer) -> Vec<Experssion> {
    let mut result = Vec::new();

    while lexer.peek() != Token::Eof {
        let expr = parse_experssion(lexer, 0.0);
        result.push(expr);
        if lexer.peek() == Token::Semicolon {
            lexer.next();
        } 
    }
    result
}

pub fn read(path: impl AsRef<Path>) -> Result<(String, usize), io::Error> {
    let content = std::fs::read_to_string(path)?;
    let line =  content.lines().count();
    Ok((content, line))
}
/* future impermentation later  */


/* fish compiler eval 
* 
* Example [+, 10, [/, [*, 4, 5], 2]]
*          eval + [10 [/, [*, 4, 5], 2]
*                left = 10 
*                right = eval /, [*, 4, 5], 2
*                        left = * eval [4, 5], 2
*                                left = 4
*                                right = 5
*                        right = 2
*                        
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
        
        Experssion::Assign(_name, value) => eval(value),

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
