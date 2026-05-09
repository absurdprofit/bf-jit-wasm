use crate::{
    instruction::{self, Instruction, InstructionSet, LeftJump, RightJump},
    tokeniser::{self, Token},
};

pub struct Program {
    pub counter: usize,
    pub memory: Vec<u8>,
    pub pointer: usize,
    instructions: Vec<InstructionSet>,
}

impl Program {
    pub fn new(tokens: impl Iterator<Item = Token>) -> Self {
        Self {
            counter: 0,
            memory: vec![0],
            pointer: 0,
            instructions: Self::collect_tokens(tokens),
        }
    }

    // pub fn run(&mut self) {
    //     while self.counter < self.instructions.len() {
    //         let instruction = &self.instructions[self.counter];
    //         instruction.execute(self);
    //     }
    // }

    fn collect_tokens(tokens: impl Iterator<Item = Token>) -> Vec<InstructionSet> {
        let mut instructions: Vec<InstructionSet> = vec![];
        let mut stack = vec![];
        let mut token_instance: Option<Token> = None;
        let mut token_instance_count = 0;
        for token in tokens {
            match token {
                tokeniser::Token::RightJump => {
                    stack.push(instructions.len());
                    instructions.push(instruction::RightJump::new(0).into());
                }
                tokeniser::Token::LeftJump => {
                    let start = stack.pop();
                    assert!(start.is_some(), "SYNTAX ERROR: Unbalanced jump.");
                    let start = start.unwrap();
                    instructions[start] = instruction::RightJump::new(instructions.len()).into();
                    instructions.push(instruction::LeftJump::new(start).into());
                }
                token => {
                    if let Some(instance) = token_instance {
                        if instance == token {
                            token_instance_count += 1;
                        } else {
                            token_instance_count = 1;
                            token_instance = Some(token);
                        }
                    }
                }
            }
        }
        instructions
    }
}
