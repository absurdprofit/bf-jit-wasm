mod compiler;
mod instruction;
mod io;
mod program;
mod tokeniser;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{program::Program, tokeniser::tokenise};

#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run(source: &str, path: &str) {
    #[cfg(target_arch = "wasm32")]
    init_panic_hook();

    Program::new(tokenise(source, path)).run();
}
