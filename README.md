# 🧠 BF-EXPERIMENT Vault

This repository is a **central vault** for all my Brainfuck-related work.

It contains:
- Brainfuck programs
- Runtime engines
- Experimental projects
- Future compiler and JIT research

This is not a single project — it's a growing **ecosystem** around Brainfuck as a low-level execution model.

---

## 📦 Current Status

### ✅ Interpreter Added

The first major project inside the vault is a **Rust Brainfuck Interpreter**.

Location:
projects/bf-interpretor/

This interpreter provides:
- Precomputed bracket jumps for loop execution
- Deterministic execution model
- 30,000-cell `u8` tape with wrapping arithmetic
- Strict pointer bounds
- Buffered stdin/stdout
- No source echo

This serves as the **foundation runtime** for everything that comes next.

---

## 🗂 Vault Structure

```text
BF-EXPERIMENT/
├── Brain-Fuck-web-IR-LAB/
│   └── web/public/
│       ├── assets/
│       ├── index.html
│       └── README.md
│
├── programs/              # All Brainfuck programs
│   ├── tests/
│   ├── demos/
│   └── experiments/
│
├── projects/
│   ├── bf-interpretor/    # Rust interpreter (current)
│   ├── bf-compiler/       # Future compiler
│   └── bf-jit/            # Future JIT engine
│
├── tools/                 # Helper scripts
└── README.md
```

### ✅ Minimal Web IR Lab Added

A small web workspace is added to the vault for browser-side experiments.

Location:
```text
Brain-Fuck-web-IR-LAB/web/public/
```

What it contains:
- `index.html` — minimal page to load/run future BF tools
- `assets/` — JS/CSS and future runtime/visualizer files
- `README.md` — notes for this web lab

Purpose:
- Start a lightweight **BF IR / visualizer lab** on the web
- Later connect it with the interpreter/compiler pipeline

## 🎯 Purpose of This Vault

This vault is a long-term experiment in:

- Language runtime design  
- Interpreter → Compiler → JIT evolution  
- Memory models and execution engines  
- Performance optimization research  
- Using Brainfuck as a minimal instruction set for complex systems  

The goal is to explore how far a tiny instruction set can be pushed with a powerful runtime.

---

## 🚀 Future Directions

Planned additions to this vault:

- IR (Intermediate Representation) layer
- Bytecode VM
- Peephole optimizations
- Loop pattern optimizations
- Native JIT backend
- WebAssembly backend
- Debugging tools
- Graphics experiments (framebuffer rendering in BF)

---

## 🧩 Philosophy

This repository treats Brainfuck not as a joke language, but as a:

> **Minimal CPU instruction set for runtime and compiler research**

By stripping away syntax and features, the focus shifts to:
- Execution models
- Memory control
- Optimization strategies

---

## 📜 License

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
