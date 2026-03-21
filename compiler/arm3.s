.global _start 
.align 4 

_start:

    ; How do we perform basic arithmetic operations in ARM assembly?
    ; Can you optimize the code for better performance?
    ; Limited to using only 4 registers for operation 

    ; Simple addition of two numbers 
    mov x0, #1 
    mov x1, #2 
    add x2, x0, x1 
    ; Simple subtraction of two numbers 
    mov x6, #5
    mov x7, #2
    sub x8, x6, x7 
    ; Simple multiplication of two numbers
    mov x3, #3 
    mov x4, #4 
    mul x5, x3, x4 
    ; Simple division of two numbers
    mov x9, #10
    mov x10, #2
    div x11, x9, x10

    ; call for exit 
    mov x0, #0  
    mov x16, #1 
    svc #0 



