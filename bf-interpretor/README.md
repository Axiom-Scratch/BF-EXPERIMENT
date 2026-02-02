# BF Runtime (Rust Brainfuck Engine)

## Overview
Rust-based Brainfuck runtime and preprocessing toolchain designed as a deterministic execution engine and research platform for interpreter and compiler design.

This project now consists of:
- A high-performance IR-driven Brainfuck interpreter
- A modular BF preprocessor (BFPP) for scalable program construction

## Current Capabilities

### Interpreter
- IR-based execution (not raw opcode stepping)
- Opcode merging and optimization passes
- Bracket matching precomputation
- Auto-growing tape with strict left bound
- Deterministic behavior
- Buffered I/O
- No source echo

### BFPP (Preprocessor)
- `#include` support
- Comment stripping
- Repeat expansion for bulk opcode generation
- Modular structure ready for macro extensions
- Example program: programs/bfpp/hello_world.bfpp

## Execution Pipeline
Source → Filter → Bracket Map → IR Build → Optimization → VM Execution

## Build & Run

Build:
cargo build --release

Run interpreter:
./target/release/bf programs/file.bf

Run preprocessor:
./target/release/bfpp input.bfpp -o output.bf

Example BFPP program:
./target/release/bfpp programs/bfpp/hello_world.bfpp -o /tmp/hello_world.bf
./target/release/bf /tmp/hello_world.bf

## Directory Layout

src/
- main.rs        CLI
- lib.rs         runtime pipeline
- parse.rs       filtering
- brackets.rs    validation
- ir.rs          IR representation
- opt.rs         optimizations
- vm.rs          execution engine
- io.rs          buffered I/O

src/bin/
- bfpp.rs        preprocessor entry

programs/
- bfpp/
- tests/
- stress/
- lib/

## Philosophy
The project treats Brainfuck as a minimal instruction set for studying runtime architecture. Emphasis is placed on deterministic execution, clear module boundaries, and scalable program construction.

## License
MIT License

Copyright (c) 2026 Axiom-Scratch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
