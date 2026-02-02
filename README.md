# 🧠 BF-EXPERIMENT Vault

This repository is the central vault for Brainfuck-related work.

It contains:
- Runtime engines
- Preprocessors and tooling
- Experimental projects and research notes

This is not a single project. It is a growing ecosystem around Brainfuck as a minimal execution model.

## 📦 Current Projects

### ✅ bf-interpretor (Rust runtime + BFPP)

Location:
bf-interpretor/

Highlights:
- IR-based interpreter with optimization pipeline
- Bracket map precomputation
- Deterministic execution with buffered I/O
- BFPP with `#include`, comment stripping, and repeat expansion

See `bf-interpretor/README.md` for build and usage.

### ✅ Brain-Fuck-web-IR-LAB (Web lab)

Location:
Brain-Fuck-web-IR-LAB/web/public/

Goal:
- Browser workspace for BF tooling, IR visualization, and future pipelines

## 🗂 Vault Structure

```text
BF-EXPERIMENT/
├── bf-interpretor/
├── Brain-Fuck-web-IR-LAB/
├── LICENSE
└── README.md
```

## 🎯 Purpose of This Vault

This vault is a long-term experiment in:

- Language runtime design
- Interpreter to compiler to JIT evolution
- Memory models and execution engines
- Performance optimization research
- Using Brainfuck as a minimal instruction set for complex systems

The goal is to explore how far a tiny instruction set can be pushed with a powerful runtime.

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
