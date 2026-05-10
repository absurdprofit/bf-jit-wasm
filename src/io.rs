use std::io;
use wasm_bindgen::prelude::*;

pub struct RuntimeIO;

pub trait IO {
    fn read_exact(buf: &mut [u8]) -> io::Result<()>;
}

#[cfg(not(target_arch = "wasm32"))]
impl IO for RuntimeIO {
    fn read_exact(buf: &mut [u8]) -> io::Result<()> {
        use std::io::Read;

        io::stdin().read_exact(buf)
    }
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    #[wasm_bindgen()]
    fn extern_read() -> u8;
}

#[cfg(target_arch = "wasm32")]
impl IO for RuntimeIO {
    fn read_exact(buf: &mut [u8]) -> io::Result<()> {
        for byte in buf {
            *byte = extern_read();
        }

        Ok(())
    }
}
