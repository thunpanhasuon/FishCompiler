# 🐟 Fish Programming Language

> A hobby project by Panha and Jule — students at **Royal University of Phnom Penh (RUPP)**, Cambodia 🇰🇭

![Fish Logo](https://img.shields.io/badge/language-Fish-orange?style=for-the-badge)
![Built With](https://img.shields.io/badge/built%20with-Rust-blue?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)


**Fish** is a toys and compile programing language that does one thing — math. You write an expression, Fish compiles it, and spits out **Apple Silicon assembly (ARM64)**. That's it. Built for fun and learning at RUPP.

## What Fish Can Do

Fish only supports arithmetic expressions right now:

```fish
FISHCOMPUTE x = 10 + 10 * 20 / 2!
FISHCOMPUTE y = 2 + 2 * 3 / 4!
FISHCOMPUTE z = 200 - 321 / 3110!
FISHCOMPUTE v = 200 + 323!
```

## Output — Apple Assembly (ARM64)

Fish compiles your expression down to **Apple Silicon assembly**. For the example above:

```fish
FISHCOMPUTE x = 10 + 10 * 20 / 2!
```

Fish produces something like:

```asm
.global _main
.align 2

_main:
    mov  x0, #10       ; load 10
    mov  x1, #20       ; load 20
    mul  x1, x1, x0    ; 10 * 20 = 200
    mov  x2, #2
    sdiv x1, x1, x2    ; 200 / 2 = 100
    mov  x2, #10       ; load 10
    add  x0, x2, x1    ; 10 + 100 = 110  (result of x)
    ret
```

The result respects **operator precedence** — `*` and `/` happen before `+` and `-`.
