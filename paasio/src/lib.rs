// sophie-cyborg's solution is elegant. It removes some duplication
//  and also ties both structs together. i've reproduced her solution below

use std::io::{Read, Result, Write};

pub type ReadStats<T> = IOStats<T>;
pub type WriteStats<T> = IOStats<T>;
pub struct IOStats<T> {
    bytes_through: usize,
    reads: usize,
    writes: usize,
    wrapped: T,
}

impl<T> IOStats<T> {
    pub fn new(wrapped: T) -> IOStats<T> {
        Self {
            bytes_through: 0,
            reads: 0,
            writes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<R: Read> Read for IOStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.wrapped.read(buf)?;
        self.reads += 1;
        self.bytes_through += bytes_read;
        Ok(bytes_read)
    }
}

impl<W: Write> Write for IOStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.wrapped.write(buf)?;
        self.writes += 1;
        self.bytes_through += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
