use enum_dispatch::enum_dispatch;
#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
use js_sys::{Function, JsOption};
use wasm_bindgen::prelude::*;

pub enum CompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    UnknownDefect,
}

pub struct RuntimeCompiler;

pub trait Compiler {
    // TODO: Replace Function in return type with an (abstract) opaque type wrapped in a future.
    // Let's call it a Runnable, it should have a method like run(&self) -> ().
    fn compile(source: impl Iterator<Item = Vec<u8>>) -> Result<Promise<Function>, CompilerError>;
}

#[enum_dispatch]
pub trait Runnable {
    fn run(&self) -> ();
}

struct WebAssembly(Function);

impl Runnable for WebAssembly {
    fn run(&self) -> () {
        let _ = self.0.call0(&JsValue::null());
    }
}

#[enum_dispatch(Runnable)]
enum RuntimeCompilerTarget {
    WebAssembly,
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    // TODO: define a streaming compiler interface
    // TODO: return a Ok(Future) that can be polled by the program.
    #[wasm_bindgen(catch)]
    fn extern_compile(
        get_chunk: &mut dyn FnMut() -> JsOption<JsValue>,
    ) -> Result<Promise<Function>, JsValue>;
}

#[cfg(target_arch = "wasm32")]
const PREAMBLE: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, 0x03,
    0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72, 0x75, 0x6e, 0x00, 0x00, 0x0a, 0x06, 0x01, 0x04,
    0x00, 0x41, 0x2a, 0x0b, 0x00, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x02, 0x03, 0x01, 0x00, 0x00,
];

#[cfg(target_arch = "wasm32")]
impl Compiler for RuntimeCompiler {
    // source is not a full WASM binary, it is simply the concatenation of emit_wasm results from instructions.
    // compilation could failed, let's handle failures by matching the error ID.
    // in the case of compilation failure we can simply do nothing and let the interpreter run to completion.
    fn compile(
        mut source: impl Iterator<Item = Vec<u8>>,
    ) -> Result<Promise<Function>, CompilerError> {
        // TODO: add module preamble
        // TODO: add program bytes
        let mut get_chunk =
            || JsOption::from_option(source.next().map(|value| JsValue::from(value)));

        match extern_compile(&mut get_chunk) {
            Ok(result) => Ok(result),
            Err(js_value) => {
                let compiler_error = if let Some(error) = js_value.as_f64() {
                    match error {
                        0.0 => CompilerError::TypeError,
                        1.0 => CompilerError::CompileError,
                        2.0 => CompilerError::LinkError,
                        3.0 => CompilerError::RuntimeError,
                        _ => CompilerError::UnknownDefect,
                    }
                } else {
                    CompilerError::UnknownDefect
                };

                Err(compiler_error)
            }
        }
    }
}
