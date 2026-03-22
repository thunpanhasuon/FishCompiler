use crate::lexer::Experssion; 
use std::alloc::alloc;
use std::fs::OpenOptions;
use std::io::Write;


pub struct ArmRegisterAllocator {
    registers: Vec<usize>,
    arm_instruction: Vec<String>,
}

impl ArmRegisterAllocator {
    pub fn new() -> Self {
        ArmRegisterAllocator { 
            registers: vec![7, 6, 5, 6, 4, 3, 2, 1, 0], 
            arm_instruction: Vec::new(),
        } 
    }
    
    pub fn arm64_instruction(&self) -> &[String] {
        &self.arm_instruction
    }
    
    pub fn allocate(&mut self) -> usize {
        self.registers.pop().expect("run out general perpose registers")
    }
    
    pub fn free(&mut self, reg: usize) {
        self.registers.push(reg);
    }
    
    pub fn arm64(&mut self, exp: &Experssion) -> usize {
        match exp {
            Experssion::Atomic(c) => {
                let arm_res = self.allocate();
                let digit = c.to_digit(10).expect("version 0.0.1 only support digit");
                self.arm_instruction.push(format!("\tmov x{}, #{}", arm_res, digit));
                arm_res 
            }
            Experssion::Assign(name, value) => {
                let res = self.arm64(value);
                self.arm_instruction.push(format!("\t; {} = x{}", name, res));
                res
            }
            Experssion::Number(n) => {
                let arm_res = self.allocate(); 
                self.arm_instruction.push(format!("\tmov x{}, #{}", arm_res, n));
                arm_res

            }
            Experssion::Operation(op, operands) => {
                let left = self.arm64(&operands[0]);
                let right = self.arm64(&operands[1]);
                
                let result_res = self.allocate();
                
                let instr = match op {
                    '+' => format!("\tadd x{}, x{}, x{}", result_res, left, right),
                    '-' => format!("\tsub x{}, x{}, x{}", result_res, left, right),
                    '*' => format!("\tmul x{}, x{}, x{}", result_res, left, right),
                    '/' => format!("\tdiv x{}, x{}, x{}", result_res, left, right),
                    _ => panic!("Unknown operater for unsupported arm {}", op),
                };
                
                self.arm_instruction.push(instr);
                self.free(left);
                self.free(right);
                
                result_res
            }

        }
    }
    
}
pub fn cgen_arm64(instruction: &[String]) -> std::io::Result<()>{
    let mut file = OpenOptions::new().write(true).append(true).write(true).create(true).open("AppleSiliconFish.s")?;
    let version = "0.0.1";
    write!(file,   "; Version: {}\n", version).unwrap();
    writeln!(file, "; Apple Silicon ARM64 Assembly Program\n").unwrap();

    let program_main_load = r#".section __TEXT,__text,regular,pure_instructions 
.p2align	2
.global _main 
_main:

"#;

    file.write_all(program_main_load.as_bytes())?;

    for instr in instruction {
        writeln!(file, "{}", instr)?;
    }
    
    let program_load_exit = r#"
    mov x16, #1
    svc #0x80
    "#;

    file.write_all(program_load_exit.as_bytes())?;
    
    Ok(())
}
