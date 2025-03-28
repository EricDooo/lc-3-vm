# LC-3 Virtual Machine in Rust

This project implements a virtual machine for the LC-3 (Little Computer 3) architecture, a simplified computer architecture designed for educational purposes. The LC-3 is commonly used in introductory computer organization and architecture courses to teach fundamental concepts like assembly language programming, memory addressing, and CPU operation.

## Overview

The LC-3 VM emulates the complete LC-3 architecture including:

- 16-bit address space with 65,536 memory locations
- 8 general-purpose registers (R0-R7)
- Program Counter (PC) and Condition Flag register
- All 16 LC-3 instructions (ADD, AND, NOT, BR, JMP, JSR, etc.)
- Trap routines for I/O operations

## Features

- Complete implementation of all LC-3 instructions
- Cross-platform support (Windows and Unix-like systems)
- Memory-mapped I/O for keyboard input
- File I/O for loading LC-3 object files
- Terminal-based input/output with raw mode support
- Clean, modular Rust implementation

## Requirements

- Rust (1.58.0 or later recommended)
- Cargo (comes with Rust)

## Installation

Clone this repository:

```bash
git clone https://github.com/yourusername/lc3-vm.git
cd lc3-vm
```

Build the project:

```bash
cargo build --release
```

## Usage

Run the VM with an LC-3 object file:

```bash
cargo run --release -- path/to/program.obj
```

Or use the built binary directly:

```bash
./target/release/lc3-vm path/to/program.obj
```

## LC-3 Architecture Details

### Registers

- `R0` to `R7`: General-purpose registers
- `PC`: Program Counter
- `COND`: Condition Register (holds flags for negative, zero, or positive results)

### Instructions

The LC-3 has 16 opcodes:

- `BR`: Branch
- `ADD`: Addition
- `LD`: Load
- `ST`: Store
- `JSR`/`JSRR`: Jump to Subroutine
- `AND`: Bitwise AND
- `LDR`: Load Register
- `STR`: Store Register 
- `RTI`: Return from Interrupt (unused in this implementation)
- `NOT`: Bitwise NOT
- `LDI`: Load Indirect
- `STI`: Store Indirect
- `JMP`/`RET`: Jump / Return from Subroutine
- `RES`: Reserved (unused)
- `LEA`: Load Effective Address
- `TRAP`: System Call

### Trap Routines

LC-3 provides system calls through trap routines:

- `GETC` (0x20): Read a character from the keyboard
- `OUT` (0x21): Output a character
- `PUTS` (0x22): Output a null-terminated string
- `IN` (0x23): Read a character and echo it
- `PUTSP` (0x24): Output a null-terminated byte string
- `HALT` (0x25): Halt the program

## Implementation Details

This implementation is written in Rust and focuses on clarity and correctness. Key aspects:

- Memory is represented as a 65,536-element array of 16-bit words
- Registers are stored in a fixed-size array
- Instructions are executed in a fetch-decode-execute cycle
- Memory-mapped registers are included for device I/O
- Trap routines are implemented using Rust's standard I/O