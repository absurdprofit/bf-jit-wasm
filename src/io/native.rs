use std::io;

use crate::io::IO;

pub struct NativeRuntimeIO;

impl IO for NativeRuntimeIO {
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        use std::io::Read;

        io::stdin().read_exact(buf)
    }

    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        use std::io::Write;

        io::stdout().write_all(buf)
    }

    fn write_error(&self, err: &str) -> io::Result<()> {
        use std::io::Write;

        io::stderr().write_all(err.as_bytes())
    }
}
