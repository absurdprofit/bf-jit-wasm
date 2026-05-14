use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::task::noop_waker;

use crate::{
    compiler::{Compiler, Runnable, RuntimeCompiler},
    instruction::{self, Instruction, InstructionSet},
    tokeniser::{self},
};

#[repr(C)]
pub struct Program {
    pub pointer: usize,
    pub counter: usize,
    pub memory: Vec<u8>,
    instructions: Vec<InstructionSet>,
}

impl Program {
    pub fn new(tokens: impl Iterator<Item = tokeniser::Token>) -> Self {
        Self {
            counter: 0,
            memory: vec![0; 1024 * 1024],
            pointer: 0,
            instructions: Self::collect_tokens(tokens),
        }
    }

    pub async fn run(&mut self) {
        let mut compile_target = RuntimeCompiler::compile(
            self.instructions
                .iter()
                .map(|instruction| instruction.emit(self)),
            self.instructions.len(),
            self,
        );
        let waker = noop_waker();

        let mut context = Context::from_waker(&waker);
        let mut pinned = if let Ok(future) = &mut compile_target {
            Some(Pin::new(future))
        } else {
            None
        };
        while self.counter < self.instructions.len() {
            let instruction = &self.instructions[self.counter].clone();
            instruction.execute(self);
            RuntimeCompiler::yield_now().await;
            if let Some(ref mut pinned) = pinned {
                match pinned.as_mut().poll(&mut context) {
                    Poll::Ready(result) => match result {
                        Ok(runnable) => {
                            runnable.run();
                            break;
                        }
                        Err(_) => continue,
                    },
                    Poll::Pending => continue,
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
                    instructions.push(instruction::RightJump::new(0).into());
                }
                tokeniser::Token::LeftJump(source_mapping) => {
                    let start = stack.pop();
                    assert!(
                        start.is_some(),
                        "SyntaxError: Unbalanced jump at {}.",
                        source_mapping
                    );
                    let (start, _) = start.unwrap();
                    instructions[start] = instruction::RightJump::new(instructions.len()).into();
                    instructions.push(instruction::LeftJump::new(start).into());
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
                            instruction::Right::new(token_instance_count, source_mapping).into()
                        }
                        tokeniser::Token::Left(source_mapping) => {
                            instruction::Left::new(token_instance_count, source_mapping).into()
                        }
                        tokeniser::Token::Increment => {
                            instruction::Increment::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Decrement => {
                            instruction::Decrement::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Input => {
                            instruction::Input::new(token_instance_count).into()
                        }
                        tokeniser::Token::Output => {
                            instruction::Output::new(token_instance_count).into()
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
