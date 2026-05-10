use wasm_bindgen::prelude::*;

pub enum CompilerError {}

pub struct RuntimeCompiler;

pub trait Compiler {
    // TODO: figure out if we what the type of a WASM instance JS object would be.
    fn compile(/* source: impl Iterator<Item = u8> */) -> Result<(), CompilerError>;
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    // TODO: define a streaming compiler interface
    fn extern_compile() -> ();
}

#[cfg(target_arch = "wasm32")]
impl Compiler for RuntimeCompiler {
    // source is not a full WASM binary, it is simply the concatenation of emit_wasm results from instructions.
    fn compile(/*source: impl Iterator<Item = u8>*/) -> Result<(), CompilerError> {
        extern_compile();
        // TODO: add module preamble
        // TODO: add program bytes
        Ok(())
    }
}
