# Brainf*ck Jit Runtime — Rust Implementation
My own Rust implementation of the [*Brainf\*ck* programming language](https://en.wikipedia.org/wiki/Brainfuck).  
The project implements a hybrid runtime where the program starts off being interpreted while it is compiled to WebAssembly asynchronously. Once the WebAssembly module is ready the program hands execution over to the WebAssembly module. The WebAssembly module has access to the runtime's state and as such picks up execution where the interpreter left off using what I'd like to call a "hopscotch jump" method. You can can see it in action in [the online demo](https://absurdprofit.github.io/bf-jit-wasm/).

## Hopscotch Jump
Since the runtime hot-swaps from the interpreter to the JIT binary I needed a way to jump to the block that corresponds to the current instruction being executed. WebAssembly, however, doesn't support (for good reason) unstructured control flow (think C style goto statements). Due to this limitation I couldn't, for example, inject a label into the JIT binary that I could arbitrarily jump to at execution time. I only call it a hopscotch jump because I'm not sure if there's already a term for it. It is however a simple concept.
Each block of WebAssembly emitted by each instruction is wrapped in its own WebAssembly block.
```
(block
  <emit>
)
```
At the top of this block, the instruction's index is injected along with a conditional jump that breaks out of the block if the program counter is larger than the index. Previously executed instruction blocks are skipped until execution reaches the correct instruction index.
```
(block $label_0
  i32.const 1      ; instruction index 1
  local.get        ; <program_counter>
  i32.sub          ; 
  i32.const 0     ;
  i32.lt_s         ; remainder less than 0
  br_if $label_0   ; break out of the block
  <emit>
)
```

## Features
- Native interpreter execution
- WebAssembly concurrent just in time compilation
- Switch to JIT binary during program execution
- Support for source mapped error output
- Instruction compression
- An in-browser demo

## Requirements

- The Rustlang toolchain 'rustup'

## Build

To build the demo
```bash
cargo install just
just build-web
```
To run the interpreter natively
```bash
cargo install just
just run <path-to-file.bf>
```