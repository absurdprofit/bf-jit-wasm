use std::io;

use wasm_bindgen::prelude::*;

use crate::io::IO;

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    fn extern_read() -> u8;
    fn extern_write(buf: u8) -> ();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn extern_write_error(s: &str);
}

pub struct WebRuntimeIO;

impl IO for WebRuntimeIO {
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        for byte in buf {
            *byte = extern_read();
        }

        Ok(())
    }

    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        for byte in buf {
            extern_write(*byte);
        }

        Ok(())
    }

    fn write_error(&self, err: &str) -> io::Result<()> {
        // In a web environment, we can only write errors to the console.
        // We can use the `web_sys` crate to log errors to the console.
        extern_write_error(err);
        Ok(())
    }
}
