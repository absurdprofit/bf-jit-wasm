use crate::instruction::{
    Decrement, Increment, Input, InstructionSet, Left, LeftJump, Output, Right, RightJump,
};

pub fn tokenise(source: &String) -> impl Iterator<Item = InstructionSet> {
    source.as_bytes().iter().filter_map(|c| match c {
        b'>' => Some(Right.into()),
        b'<' => Some(Left.into()),
        b'+' => Some(Increment.into()),
        b'-' => Some(Decrement.into()),
        b'.' => Some(Output.into()),
        b',' => Some(Input.into()),
        b'[' => Some(RightJump.into()),
        b']' => Some(LeftJump.into()),
        _ => None,
    })
}
