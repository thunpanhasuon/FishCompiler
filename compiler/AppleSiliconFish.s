; Version: 0.0.1
; Apple Silicon ARM64 Assembly Program

.section __TEXT,__text,regular,pure_instructions 
.p2align	2
.global _main 
_main:

	mov x0, #1
	mov x1, #2
	mov x2, #3
	mul x3, x1, x2
	add x2, x0, x3
	; y = x2

    mov x16, #1
    svc #0x80
    