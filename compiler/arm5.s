; Version: 0.0.1
; Apple Silicon ARM64 Assembly Program

.section __TEXT,__text,regular,pure_instructions 
.p2align	2
.global _main 
_main:
    
        mov x0, #0
        mov x16, #1
        svc #0x80
    