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
    tokeniser::tokenise,
};

#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn run(source: &str, path: &str) {
    #[cfg(target_arch = "wasm32")]
    init_panic_hook();

    let program = Program::new(tokenise(source, path));
    let instructions: Vec<instruction::InstructionSet> = vec![
        instruction::Input::new(1).into(),
        instruction::Output::new(1).into(),
        // instruction::Increment::new(10).into(),
        // instruction::Increment::new(10).into(),
        // instruction::Decrement::new(5).into(),
    ];
    if let Ok(result) = RuntimeCompiler::compile(instructions.iter().map(|i| i.emit(&program))) {
        let runnable = result.await;
        if let Ok(runnable) = runnable {
            runnable.run();
        }
    }
    RuntimeIO::write_all(&[program.memory[program.pointer]]);
    RuntimeIO::write_all(&[program.counter as u8]);
}
