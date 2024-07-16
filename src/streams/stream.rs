/// Stream trait, includes methods for seeking and getting the length.
pub trait Stream {
    /// Get the size of this stream, for a file stream, this would be the size of the file.
    fn get_length(&self) -> Option<u64>;

    /// Indicates whether the stream's current position can be moved or not.
    fn can_seek(&self) -> bool;

    /// Get the current position for reading/writing in this stream.
    fn get_position(&mut self) -> Option<u64>;

    /// Set the current position for reading/writing in this stream.
    fn set_position(&mut self, position: u64) -> bool;
}