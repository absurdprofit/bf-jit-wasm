use enum_dispatch::enum_dispatch;

use crate::{
    instruction::{
        core::{Decrement, Increment, Input, Left, LeftJump, Output, Right, RightJump},
        optimisation::{LeftCarry, LeftScan, RightCarry, RightScan, Zero},
    },
    program::Program,
};

pub mod core;
pub mod optimisation;

#[enum_dispatch]
pub trait Instruction {
    fn execute(&self, program: &mut Program) -> ();
    fn emit(&self, program: &Program) -> Vec<u8>;
}

#[enum_dispatch(Instruction)]
#[derive(Clone, Debug, PartialEq)]
pub enum InstructionSet {
    RightScan,
    LeftScan,
    LeftCarry,
    RightCarry,
    Zero,
    Right,
    Left,
    Increment,
    Decrement,
    Input,
    Output,
    LeftJump,
    RightJump,
}

pub trait Optimisation {
    fn try_fold(&self) -> Option<InstructionSet>;
}
