mod instruction;
mod io;
mod program;
mod tokeniser;
use std::{env, fs::File, io::Read};

use crate::{program::Program, tokeniser::tokenise};

fn main() {
    let args = env::args();
    let path = args.skip(1).next();
    if path.is_none() {
        panic!("Usage: <input.bf>");
    }
    let path = path.unwrap();
    let mut file = File::open(&path).expect(&format!("File not found at {}.", path)[..]);
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect(&format!("Unable to read file at {}.", path)[..]);

    let mut program = Program::new(tokenise(&source, &path));
    program.run();
}
