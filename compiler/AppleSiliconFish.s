; Version: 0.0.1
; Apple Silicon ARM64 Assembly Program

.section __TEXT,__text,regular,pure_instructions 
.p2align	2
.global _main 
_main:

	mov x0, #2
	mov x1, #3
	mul x2, x0, x1
	mov x1, #1
	add x0, x2, x1

    mov x0, #0
    mov x16, #1
    svc #0x80
    