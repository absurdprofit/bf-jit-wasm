use std::io::{self, Read};

use enum_dispatch::enum_dispatch;

use crate::{
    program::Program,
    tokeniser::{self, SourceMapping},
};

#[derive(Clone, Debug)]
pub struct Right {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug)]
pub struct Left {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug)]
pub struct Increment {
    amount: u8,
}

#[derive(Clone, Debug)]
pub struct Decrement {
    amount: u8,
}

#[derive(Clone, Debug)]
pub struct Input {
    count: usize,
}

#[derive(Clone, Debug)]
pub struct Output {
    count: usize,
}

#[derive(Clone, Debug)]
pub struct RightJump {
    end: usize,
}

#[derive(Clone, Debug)]
pub struct LeftJump {
    start: usize,
}

#[enum_dispatch]
pub trait Instruction {
    fn execute(&self, program: &mut Program) -> ();
}

impl Increment {
    pub fn new(amount: u8) -> Self {
        Self { amount }
    }
}

impl Instruction for Increment {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] = program.memory[program.pointer].wrapping_add(self.amount);
        program.counter += 1;
    }
}

impl Decrement {
    pub fn new(amount: u8) -> Self {
        Self { amount }
    }
}

impl Instruction for Decrement {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] = program.memory[program.pointer].wrapping_sub(self.amount);
        program.counter += 1;
    }
}

impl Left {
    pub fn new(count: usize, source_mapping: SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for Left {
    fn execute(&self, program: &mut Program) -> () {
        assert!(
            program.pointer > 0,
            "RuntimeError: Memory underflow at {}",
            self.source_mapping
        );
        program.pointer -= self.count;
        program.counter += 1;
    }
}

impl Right {
    pub fn new(count: usize, source_mapping: SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for Right {
    fn execute(&self, program: &mut Program) -> () {
        program.pointer += self.count;
        if program.pointer >= program.memory.len() {
            match program.memory.try_reserve(1) {
                Ok(_) => program.memory.push(0),
                Err(_) => panic!("RuntimeError: Memory overflow at {}", self.source_mapping),
            };
        }
        program.counter += 1;
    }
}

impl Input {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Instruction for Input {
    fn execute(&self, program: &mut Program) -> () {
        for _ in 0..self.count {
            io::stdin()
                .read_exact(&mut program.memory[program.pointer..program.pointer + 1])
                .expect("Failed to read byte from standard input.");
        }
        program.counter += 1;
    }
}

impl Output {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Instruction for Output {
    fn execute(&self, program: &mut Program) -> () {
        for _ in 0..self.count {
            print!("{}", program.memory[program.pointer] as char);
        }
        program.counter += 1;
    }
}

impl RightJump {
    pub fn new(end: usize) -> Self {
        Self { end }
    }
}

impl Instruction for RightJump {
    fn execute(&self, program: &mut Program) -> () {
        program.counter = if program.memory[program.pointer] == 0 {
            self.end + 1
        } else {
            program.counter + 1
        }
    }
}

impl LeftJump {
    pub fn new(start: usize) -> Self {
        Self { start }
    }
}

impl Instruction for LeftJump {
    fn execute(&self, program: &mut Program) -> () {
        program.counter = if program.memory[program.pointer] != 0 {
            self.start + 1
        } else {
            program.counter + 1
        }
    }
}

#[enum_dispatch(Instruction)]
#[derive(Clone, Debug)]
pub enum InstructionSet {
    Right,
    Left,
    Increment,
    Decrement,
    Input,
    Output,
    LeftJump,
    RightJump,
}
