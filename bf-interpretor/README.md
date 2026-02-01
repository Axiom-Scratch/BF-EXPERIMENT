# BF Runtime (Rust Brainfuck Engine)

## Short description
- Brainfuck runtime written entirely in Rust with clean module boundaries.
- Designed as the foundation for compiler, IR, and JIT experimentation.
- Deterministic execution engine that never echoes program source.

## Current Features (v1 – Rust Core)
- Classic Brainfuck execution over a 30k cell tape with `u8` wrap.
- Bracket jump precomputation before running the VM loop.
- Deterministic model with strict pointer bounds and no undefined behavior.
- Buffered I/O for stdin/stdout during execution.
- Cargo build pipeline driven by the Rust toolchain.

## Execution Architecture Diagram
BF Source → Filter Ops → Bracket Map → VM Loop

## Build & Run
- **Requirements**: Rust toolchain with Cargo installed.
- **Build**: `cargo build`
- **Run**: `cargo run --bin bf -- programs/file.bf`

## Project Structure (Rust)
```
src/main.rs      → CLI entry
src/lib.rs       → Public API
src/vm.rs        → Execution engine
src/brackets.rs  → Jump table
src/parse.rs     → Opcode filtering
src/bin/bfpp.rs  → Preprocessor (future)
```

## Design Philosophy
- Runtime-first architecture that keeps the CLI (main.rs) separate from the core runtime (lib.rs).
- Layered evolution path: Interpreter → IR → Bytecode VM → JIT backend.
- Focus on determinism and correctness, with no source echo and explicit tape bounds.

## Future Roadmap
- Opcode merging and filtering optimizations.
- IR layer for analysis and transformation.
- Bytecode VM as an explicit execution layer.
- JIT backend exploration for native emission.
- Debug tools for stepping, traces, and tape inspection.
- Advanced memory models for segmented or large tapes.

## Project Goal
- Research sandbox for interpreter and compiler design centered on a deterministic Brainfuck runtime.

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
