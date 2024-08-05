use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::io::SeekFrom::Current;
use crate::streams::readablestream::{ReadableByteStream, ReadableStream};
use crate::streams::stream::Stream;
use crate::streams::writablestream::WritableStream;
use crate::result::Result;

impl Stream for File {
    fn get_length(&self) -> Option<u64> {
        let meta = self.metadata();
        match meta {
            Ok(metadata) => Some(metadata.len()),
            Err(_) => None
        }
    }

    fn can_seek(&self) -> bool {
        true
    }

    fn get_position(&mut self) -> Option<u64> {
        let position = self.stream_position();
        match position {
            Ok(pos) => Some(pos),
            Err(_) => None
        }
    }

    fn set_position(&mut self, position: u64) -> bool {
        self.seek(SeekFrom::Start(position)).is_ok()
    }
}

impl ReadableStream<u8> for File {
    fn can_read(&mut self) -> bool {
        if let Some(available) = self.available() { available > 0 }
        else { true } // If unable to determine, always return true
    }

    fn available(&mut self) -> Option<u64> {
        let length = self.get_length();
        let position = self.get_position();

        if let Some(len) = length {
            if let Some(pos) = position {
                Some(len - pos)
            }
            else { None }
        } else { None }
    }

    fn read(&mut self, array: &mut [u8]) -> Result<usize> {
        Ok(std::io::Read::read(self, array)?)
    }
}

impl WritableStream<u8> for File {
    fn write(&mut self, value: &[u8]) {
        Write::write_all(self, value).ok();
    }

    fn can_write(&self) -> bool {
        let metadata = self.metadata();
        if let Ok(meta) = metadata { 
            !meta.permissions().readonly()
        } else { false }
    }
}