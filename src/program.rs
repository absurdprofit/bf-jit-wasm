use std::{pin::pin, task::Poll};

use crate::{
    compiler::{Compiler, Runnable, RuntimeCompiler},
    instruction::{self, Instruction, InstructionSet, Optimisation},
    io::{IO, RuntimeIO},
    tokeniser::{self},
};

#[repr(C)]
pub struct Program {
    pub pointer: usize,
    pub counter: usize,
    pub memory: Vec<u8>,
    pub io: RuntimeIO,
    compiler: RuntimeCompiler,
    instructions: Vec<InstructionSet>,
}

impl Program {
    pub fn new(tokens: impl Iterator<Item = tokeniser::Token>) -> Self {
        Self {
            counter: 0,
            memory: vec![0; 1024 * 1024],
            pointer: 0,
            io: RuntimeIO::new(),
            compiler: RuntimeCompiler::new(),
            instructions: Self::collect_tokens(tokens),
        }
    }

    pub async fn run(&mut self) {
        let compile_target = self.compiler.compile(self.instructions.iter(), &self);

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

    fn collect_tokens(tokens: impl Iterator<Item = tokeniser::Token>) -> Vec<InstructionSet> {
        let mut instructions: Vec<InstructionSet> = vec![];
        let mut stack = vec![];
        let mut tokens = tokens.peekable();
        while let Some(token) = tokens.next() {
            match token {
                tokeniser::Token::RightJump(source_mapping) => {
                    stack.push((instructions.len(), source_mapping));
                    instructions.push(instruction::core::RightJump::new(0).into());
                }
                tokeniser::Token::LeftJump(source_mapping) => {
                    let start = stack.pop();
                    assert!(
                        start.is_some(),
                        "SyntaxError: Unbalanced jump at {}.",
                        source_mapping
                    );
                    let (start, _) = start.unwrap();
                    let end = instructions.len();
                    if let Some(instruction) = instructions[start..end].try_fold() {
                        instructions.truncate(start);
                        instructions.push(instruction);
                        continue;
                    }
                    instructions[start] = instruction::core::RightJump::new(end).into();
                    instructions.push(instruction::core::LeftJump::new(start).into());
                }
                token => {
                    let mut token_instance_count: usize = 1;
                    while let Some(next) = tokens.peek() {
                        if token == *next {
                            tokens.next();
                            token_instance_count += 1;
                        } else {
                            break;
                        }
                    }

                    let instruction: InstructionSet = match token {
                        tokeniser::Token::Right(source_mapping) => {
                            instruction::core::Right::new(token_instance_count, source_mapping)
                                .into()
                        }
                        tokeniser::Token::Left(source_mapping) => {
                            instruction::core::Left::new(token_instance_count, source_mapping)
                                .into()
                        }
                        tokeniser::Token::Increment => {
                            instruction::core::Increment::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Decrement => {
                            instruction::core::Decrement::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Input => {
                            instruction::core::Input::new(token_instance_count).into()
                        }
                        tokeniser::Token::Output => {
                            instruction::core::Output::new(token_instance_count).into()
                        }
                        _ => unreachable!(
                            "Jumps have been specially handled earlier in the routine."
                        ),
                    };
                    instructions.push(instruction);
                }
            }
        }

        assert!(
            stack.len() == 0,
            "SyntaxError: Unbalanced jump at {}",
            stack.pop().unwrap().1
        );
        instructions
    }
}
