use crate::streams::stream::Stream;
use crate::result::Result;

/// Readable stream trait.
pub trait ReadableStream<T>: Stream {
    /// Indicates whether the stream can be read currently.
    fn can_read(&mut self) -> bool;

    /// Get the amount of space available for reading.
    fn available(&mut self) -> Option<u64>;
    
    /// Read bytes into an array without offset or size.
    fn read_simple(&mut self, array: &mut [T]) -> Result<usize> {
        self.read(array, 0)
    }
    
    /// Read bytes into an array.
    fn read(&mut self, array: &mut [T], offset: i64) -> Result<usize>;
}

pub trait ReadableByteStream: ReadableStream<u8> {
    fn read_one(&mut self) -> Result<Option<u8>> {
        let mut buf = vec![0];
        let read = self.read_simple(&mut buf)?;
        if read == 0 { return Ok(None) }
        Ok(Some(buf[0]))
    }
}

impl <T: ReadableStream<u8>> ReadableByteStream for T {}