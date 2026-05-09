use crate::instruction::{Instruction, InstructionSet};

pub struct Program {
    pub counter: u64,
    pub memory: Vec<u8>,
    pub pointer: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            counter: 0,
            memory: vec![0],
            pointer: 0,
        }
    }

    pub fn run(&mut self, instructions: impl Iterator<Item = InstructionSet>) {
        for instruction in instructions {
            instruction.execute(self);
        }
    }
}
