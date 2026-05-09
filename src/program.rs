use crate::{
    instruction::{self, Instruction, InstructionSet},
    tokeniser::Token,
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

    pub fn run(&mut self) {
        while self.counter < self.instructions.len() {
            let instruction = &self.instructions[self.counter];
            instruction.execute(self);
        }
    }

    fn collect_tokens(tokens: impl Iterator<Item = Token>) -> Vec<InstructionSet> {
        vec![]
    }
}
