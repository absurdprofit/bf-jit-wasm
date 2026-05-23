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
