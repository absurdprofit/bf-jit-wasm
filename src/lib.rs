mod compiler;
mod instruction;
mod io;
mod program;
mod tokeniser;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    compiler::{Compiler, Runnable, RuntimeCompiler},
    instruction::Instruction,
    io::{IO, RuntimeIO},
    program::Program,
    tokeniser::{SourceMapping, tokenise},
};

#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn run(source: &str, path: &str) {
    #[cfg(target_arch = "wasm32")]
    init_panic_hook();

    let mut program = Program::new(tokenise(source, path));

    program.run().await;
}
