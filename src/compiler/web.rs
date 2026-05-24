use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::compiler::{Runnable, RuntimeCompilerError};

pub struct LEB128 {
    pub inner: Vec<u8>,
}

impl From<u32> for LEB128 {
    fn from(mut value: u32) -> Self {
        let mut inner = vec![];
        loop {
            let mut byte = value & 0x7f;
            value = value >> 7;
            if value != 0 {
                byte |= 0x80;
            }
            inner.push(byte as u8);
            if value == 0 {
                break;
            }
        }
        Self { inner }
    }
}

pub struct SLEB128 {
    pub inner: Vec<u8>,
}

impl From<i32> for SLEB128 {
    fn from(mut value: i32) -> Self {
        let mut inner = Vec::new();

        loop {
            let byte = (value & 0x7f) as u8;

            // arithmetic shift preserves sign
            value >>= 7;

            let sign_bit_set = (byte & 0x40) != 0;

            let done = (value == 0 && !sign_bit_set) || (value == -1 && sign_bit_set);

            if done {
                inner.push(byte);
                break;
            } else {
                inner.push(byte | 0x80);
            }
        }

        Self { inner }
    }
}

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

pub struct WebAssembly(Function);

impl WebAssembly {
    pub fn new(function: Function) -> Self {
        Self(function)
    }
}

impl Runnable for WebAssembly {
    fn run(&self) -> () {
        let _ = self.0.call0(&JsValue::null());
    }
}
