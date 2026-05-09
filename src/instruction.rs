use enum_dispatch::enum_dispatch;

use crate::program::Program;

pub struct Right;
pub struct Left;
pub struct Increment;
pub struct Decrement;
pub struct Input;
pub struct Output;
pub struct RightJump;
pub struct LeftJump;

#[enum_dispatch]
pub trait Instruction {
    fn execute(&self, program: &mut Program) -> ();
}

impl Instruction for Increment {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] += 1;
        program.counter += 1;
    }
}

impl Instruction for Decrement {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] -= 1;
        program.counter += 1;
    }
}

impl Instruction for Left {
    fn execute(&self, program: &mut Program) -> () {
        assert!(program.pointer > 0, "RUNTIME ERROR: Memory underflow.");
        program.pointer -= 1;
        program.counter += 1;
    }
}

impl Instruction for Right {
    fn execute(&self, program: &mut Program) -> () {
        // need to assert no overflow
        program.pointer += 1;
        program.counter += 1;
    }
}

impl Instruction for Input {
    fn execute(&self, program: &mut Program) -> () {}
}

impl Instruction for Output {
    fn execute(&self, program: &mut Program) -> () {
        print!("{}", program.memory[program.pointer]);
        program.counter += 1;
    }
}

impl Instruction for RightJump {
    fn execute(&self, program: &mut Program) -> () {}
}

impl Instruction for LeftJump {
    fn execute(&self, program: &mut Program) -> () {}
}

#[enum_dispatch(Instruction)]
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
