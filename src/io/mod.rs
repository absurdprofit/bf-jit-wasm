use std::io;

pub mod native;
pub mod web;

use enum_dispatch::enum_dispatch;

use crate::io::{native::NativeRuntimeIO, web::WebRuntimeIO};

#[enum_dispatch]
pub trait IO {
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()>;
    fn write_all(&self, buf: &[u8]) -> io::Result<()>;
}

#[enum_dispatch(IO)]
pub enum RuntimeIO {
    NativeRuntimeIO,
    WebRuntimeIO,
}

const PLATFORM_RUNTIME_IO: RuntimeIO = if cfg!(target_arch = "wasm32") {
    RuntimeIO::WebRuntimeIO(WebRuntimeIO)
} else {
    RuntimeIO::NativeRuntimeIO(NativeRuntimeIO)
};

impl RuntimeIO {
    pub fn read_exact(buf: &mut [u8]) -> io::Result<()> {
        PLATFORM_RUNTIME_IO.read_exact(buf)
    }

    pub fn write_all(buf: &[u8]) -> io::Result<()> {
        PLATFORM_RUNTIME_IO.write_all(buf)
    }
}
