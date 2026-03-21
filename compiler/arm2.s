//
// hello world 
//

.section __TEXT,__text 
.global _main
.align 2 

msg_len = . - msg 


.align 2 
_print:
	sub sp, sp, #16
	mov x2, sp 
	add x2, x2, #15 
	mov x3, #10 
loop: 
	udiv x4, x0, x3     // x4 = x0 / 10 
	msub x5, x4, x3, x0 // x5 = x0 % 10 (remainder) 
	add x5, x5, #48	    // '0' = is ASCII 48 
	strb w5, [x2]
	sub x2, x2, #1 	    // move the pointer back 
	mov x0, x4	    // x0 = x0 / 10 
	cbnz x0, loop	    // loop if not zero 

	add, x1, x2, #1 
	mov x3, sp 
	add x3, x3, #16 
	sub x2, x3, x1 	     // end - start
	mov x0, #1 
	
	// x2 == lenght need computing 

	mov x16, #4 
	svc #0x80 
	add sp, sp, #16
	ret



_main: 

	// X0 - X7 all for paramater 
	// X16 a specail call 
	bl _print

	// macOS call for exit 
	mov x0, #0 
	mov x16, #1	
	svc #0x80 

.align 2
.section __TEXT,__cstring 
msg: 
	.ascii "Hello Apple arm64 :)\n" 




