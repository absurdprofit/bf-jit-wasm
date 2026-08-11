use enum_dispatch::enum_dispatch;

use crate::{
    instruction::{
        core::{Decrement, Increment, Input, Left, LeftJump, Output, Right, RightJump},
        optimisation::{LeftCarry, LeftScan, RightCarry, RightScan, Zero},
    },
    program::Program,
    tokeniser,
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

impl FromIterator<tokeniser::Token> for Vec<InstructionSet> {
    fn from_iter<T: IntoIterator<Item = tokeniser::Token>>(tokens: T) -> Self {
        let mut instructions: Vec<InstructionSet> = vec![];
        let mut stack = vec![];
        let mut tokens = tokens.into_iter().peekable();
        while let Some(token) = tokens.next() {
            match token {
                tokeniser::Token::RightJump(source_mapping) => {
                    stack.push((instructions.len(), source_mapping));
                    instructions.push(RightJump::new(0).into());
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
                    instructions[start] = RightJump::new(end).into();
                    instructions.push(LeftJump::new(start).into());
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
                            Right::new(token_instance_count, source_mapping).into()
                        }
                        tokeniser::Token::Left(source_mapping) => {
                            Left::new(token_instance_count, source_mapping).into()
                        }
                        tokeniser::Token::Increment => {
                            Increment::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Decrement => {
                            Decrement::new(token_instance_count as u8).into()
                        }
                        tokeniser::Token::Input => Input::new(token_instance_count).into(),
                        tokeniser::Token::Output => Output::new(token_instance_count).into(),
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

pub trait Optimisation {
    fn try_fold(&self) -> Option<InstructionSet>;
}
