use std::{
    pin::Pin,
    task::{Context, Poll},
};

use enum_dispatch::enum_dispatch;
#[cfg(target_arch = "wasm32")]
use js_sys::{Function, JsOption, Promise, futures::JsFuture};
use wasm_bindgen::prelude::*;

pub enum RuntimeCompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    UnknownDefect,
}

pub struct RuntimeCompiler;

pub trait Compiler {
    type CompileFuture: Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>;

    fn compile(
        source: impl Iterator<Item = Vec<u8>>,
    ) -> Result<Self::CompileFuture, RuntimeCompilerError>;
}

#[enum_dispatch]
pub trait Runnable {
    fn run(&self) -> ();
}

pub struct WebAssembly(Function);

impl Runnable for WebAssembly {
    fn run(&self) -> () {
        let _ = self.0.call0(&JsValue::null());
    }
}

#[enum_dispatch(Runnable)]
pub enum RuntimeCompilerTarget {
    WebAssembly,
}

pub struct RuntimeCompilerTargetFuture {
    inner: JsFuture<Function>,
}

impl Future for RuntimeCompilerTargetFuture {
    type Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let pinned = Pin::new(&mut self.inner);
        pinned.poll(cx).map(|result| {
            result
                .map(|function| WebAssembly(function).into())
                .map_err(|js_value| js_value.as_f64().into())
        })
    }
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn extern_compile(
        get_chunk: &mut dyn FnMut() -> JsOption<JsValue>,
    ) -> Result<Promise<Function>, JsValue>;
}

#[cfg(target_arch = "wasm32")]
impl From<Option<f64>> for RuntimeCompilerError {
    fn from(value: Option<f64>) -> Self {
        if let Some(error) = value {
            match error {
                0.0 => RuntimeCompilerError::TypeError,
                1.0 => RuntimeCompilerError::CompileError,
                2.0 => RuntimeCompilerError::LinkError,
                3.0 => RuntimeCompilerError::RuntimeError,
                _ => RuntimeCompilerError::UnknownDefect,
            }
        } else {
            RuntimeCompilerError::UnknownDefect
        }
    }
}

#[cfg(target_arch = "wasm32")]
const PREAMBLE: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, 0x03,
    0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x72, 0x75, 0x6e, 0x00, 0x00, 0x0a, 0x06, 0x01, 0x04,
    0x00, 0x41, 0x2a, 0x0b, 0x00, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x02, 0x03, 0x01, 0x00, 0x00,
];

#[cfg(target_arch = "wasm32")]
impl Compiler for RuntimeCompiler {
    type CompileFuture = RuntimeCompilerTargetFuture;
    // source is not a full WASM binary, it is simply the concatenation of emit_wasm results from instructions.
    // compilation could fail, let's handle failures by matching the error ID.
    // in the case of compilation failure we can simply do nothing and let the interpreter run to completion.
    fn compile(
        mut source: impl Iterator<Item = Vec<u8>>,
    ) -> Result<Self::CompileFuture, RuntimeCompilerError> {
        // TODO: add module preamble
        // TODO: add program bytes
        let mut get_chunk =
            || JsOption::from_option(source.next().map(|value| JsValue::from(value)));

        match extern_compile(&mut get_chunk) {
            Ok(result) => {
                // Ok(result.into_future());
                Ok(RuntimeCompilerTargetFuture {
                    inner: result.into_future(),
                })
            }
            Err(js_value) => Err(js_value.as_f64().into()),
        }
    }
}
