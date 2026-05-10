mod instruction;
mod program;
mod tokeniser;
use std::{fs::File, io::Read};

use crate::{program::Program, tokeniser::tokenise};

fn main() {
    let path = "test.bf";
    let mut file = File::open(path).expect("File not found.");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file.");

    let mut program = Program::new(tokenise(&input, &String::from(path)));
    program.run();
}
