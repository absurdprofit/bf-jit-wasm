use std::io;

use wasm_bindgen::prelude::*;

use crate::io::IO;

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    fn extern_read() -> u8;
    fn extern_write(buf: u8) -> ();
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
}
