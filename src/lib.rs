mod instruction;
mod io;
mod program;
mod tokeniser;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{program::Program, tokeniser::tokenise};

#[wasm_bindgen]
pub fn run(source: &str, path: &str) {
    Program::new(tokenise(source, path)).run();
}
