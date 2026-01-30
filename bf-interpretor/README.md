# 🧠 BF-Interpretor (Brainfuck Runtime Engine)

A Brainfuck interpreter written in **modern C++**, designed as the foundation for a future **compiler, IR system, and JIT engine**.

This project is not just a toy interpreter — it is built as a **language runtime experiment** that will evolve into a high-performance execution engine capable of running complex workloads (including graphics experiments).

---

## ✨ Current Features (v1)

✅ Classic Brainfuck execution  
✅ Full loop support using bracket jump table (`[` `]`)  
✅ Configurable tape size (default: 30,000 cells)  
✅ `u8` cell model with wraparound (0–255)  
✅ Strict pointer bounds (prevents memory bugs)  
✅ Standard input/output support (`.` and `,`)  
✅ Clean CMake-based build system  

Example tested program:

```bf
+++++[.-]
```

Output:
```
05 04 03 02 01
```

Interpreter architecture:

```
BF Source → Bracket Map → VM Execution Loop
```

---

## 🛠 Build Instructions

### Requirements
- C++17 compiler
- CMake ≥ 3.16

### Build
```bash
cmake -S . -B build
cmake --build build -j
```

### Run
```bash
./build/bf programs/loop.bf
```

---

## 📁 Project Structure

```
include/
 ├── bf_vm.h          # Brainfuck virtual machine
 └── bf_brackets.h    # Bracket jump table builder

src/
 ├── main.cpp         # CLI entry point
 ├── bf_vm.cpp        # Interpreter execution engine
 └── bf_brackets.cpp  # Loop matching logic
```

---

## 🧠 Design Philosophy

This project is structured as a **miniature language runtime**, similar in architecture to real-world language engines:

| Layer | Purpose |
|------|--------|
| Parser/Filter | Prepares valid BF code |
| Bracket Map | Precomputes loop jumps |
| VM | Executes instructions over memory |
| Future IR | Optimized intermediate layer |
| Future JIT | Native/WASM execution backend |

The goal is to evolve from:

```
Interpreter → IR Compiler → Bytecode VM → JIT Engine
```

---

## 🚀 Future Upgrades

Planned features:

### 🔹 Core Runtime
- BF source filtering stage
- Instruction merging (`+++++` → `ADD 5`)
- Bytecode / IR representation
- Performance profiling

### 🔹 Optimization
- Peephole optimizations
- Loop pattern recognition (`[-]`, `[->+<]`)
- Reduced branch execution
- Faster memory access strategies

### 🔹 Execution Backends
- Bytecode VM
- Native JIT (x86-64)
- WebAssembly backend

### 🔹 Debugging Tools
- Step debugger
- Memory viewer
- Instruction trace mode
- Execution statistics

### 🔹 Advanced Memory Models
- Large tape configurations
- Segmented memory
- Custom cell sizes (u16/u32 modes)

---

## 🎯 Project Goal

To create a **minimal yet powerful language execution engine** that demonstrates how real interpreters, compilers, and JIT runtimes are built — using Brainfuck as the core instruction set.

This project serves as:
- A systems programming exercise
- A compiler/runtime research sandbox
- A foundation for extreme experiments (like graphics rendering in BF)

---

## 📜 License

MIT (or choose your preferred license)