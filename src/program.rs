use std::{pin::pin, task::Poll};

use crate::{
    compiler::{self, Compiler, Runnable},
    instruction::{self, Instruction},
    io::{self, IO},
    tokeniser::{self},
};

#[repr(C)]
pub struct Program {
    pub pointer: usize,
    pub counter: usize,
    pub memory: Vec<u8>,
    pub io: io::RuntimeIO,
    compiler: compiler::RuntimeCompiler,
    instructions: Vec<instruction::InstructionSet>,
}

impl Program {
    pub fn new(tokens: impl Iterator<Item = tokeniser::Token>) -> Self {
        Self {
            counter: 0,
            memory: vec![0; 1024 * 1024],
            pointer: 0,
            io: io::RuntimeIO::new(),
            compiler: compiler::RuntimeCompiler::new(),
            instructions: Vec::from_iter(tokens),
        }
    }

    pub async fn run(&mut self) {
        let compile_target = self.compiler.compile(self.instructions.iter(), self);

        let mut compile_target = if let Ok(future) = compile_target {
            Some(pin!(future))
        } else {
            None
        };
        while self.counter < self.instructions.len() {
            let instruction = &self.instructions[self.counter].clone();
            instruction.execute(self);
            if let Some(ref mut pinned) = compile_target {
                if let Poll::Ready(result) = futures::poll!(pinned) {
                    match result {
                        Ok(runnable) => {
                            runnable.run();
                            break;
                        }
                        Err(error) => {
                            self.io
                                .write_error(&format!(
                                    "Compilation failed with error: {:?}\n",
                                    error
                                ))
                                .ok();
                            compile_target = None;
                        }
                    }
                } else {
                    self.compiler.yield_now().await;
                }
            }
        }
    }
}
