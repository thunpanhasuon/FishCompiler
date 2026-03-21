.global _main 
.align 4  

_main: 
	mov x0, #1
	adr x1, message 
	mov x2, 22 
	mov x16, #4
	svc #0 
	
	mov x0, #0
	mov x16, #1 
	svc #0  

message:
	.ascii "Hello, Apple Silicon!\n"
