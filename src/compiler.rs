use wasm_bindgen::prelude::*;

pub enum CompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    Defect,
}

pub struct RuntimeCompiler;

pub trait Compiler {
    // TODO: figure out if we what the type of a WASM instance JS object would be.
    fn compile(source: impl Iterator<Item = u8>) -> Result<(), CompilerError>;
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    // TODO: define a streaming compiler interface
    // TODO: return a Ok(Future) that can be polled by the program.
    #[wasm_bindgen(catch)]
    fn extern_compile() -> Result<(), JsValue>;
}

#[cfg(target_arch = "wasm32")]
impl Compiler for RuntimeCompiler {
    // source is not a full WASM binary, it is simply the concatenation of emit_wasm results from instructions.
    // compilation could failed, let's handle failures by matching the error ID.
    // in the case of compilation failure we can simply do nothing and let the interpreter run to completion.
    fn compile(source: impl Iterator<Item = u8>) -> Result<(), CompilerError> {
        // TODO: add module preamble
        // TODO: add program bytes
        match extern_compile() {
            Ok(result) => Ok(result),
            Err(js_value) => {
                let compiler_error = if let Some(error) = js_value.as_f64() {
                    match error {
                        0.0 => CompilerError::TypeError,
                        1.0 => CompilerError::CompileError,
                        2.0 => CompilerError::LinkError,
                        3.0 => CompilerError::RuntimeError,
                        _ => CompilerError::Defect,
                    }
                } else {
                    CompilerError::Defect
                };

                Err(compiler_error)
            }
        }
    }
}
