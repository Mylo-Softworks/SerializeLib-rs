use crate::streams::stream::Stream;

/// Writable stream trait.
pub trait WritableStream<T>: Stream {
    /// Write a single T to the stream.
    fn write_single(&mut self, value: T) {
        self.write(&[value])
    }
    
    /// Write an array of T to the stream.
    fn write(&mut self, value: &[T]);
    
    /// Returns true if the stream can be written to, false if it can't.
    fn can_write(&self) -> bool;
}