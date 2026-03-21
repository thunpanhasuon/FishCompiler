use std::fs::OpenOptions;
use std::io::Write;

pub fn cgen_arm64() -> std::io::Result<()>{
    let mut file = OpenOptions::new().write(true).append(true).write(true).create(true).open("arm5.s")?;
    let version = "0.0.1";
    write!(file,   "; Version: {}\n", version).unwrap();
    writeln!(file, "; Apple Silicon ARM64 Assembly Program\n").unwrap();

    let program_main_load = r#".section __TEXT,__text,regular,pure_instructions 
.p2align	2
.global _main 
_main:
    "#;

    file.write_all(program_main_load.as_bytes())?;

    
    let program_load_exit = r#"
        mov x0, #0
        mov x16, #1
        svc #0x80
    "#;

    file.write_all(program_load_exit.as_bytes())?;
    
    Ok(())
}
