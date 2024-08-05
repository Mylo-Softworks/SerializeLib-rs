use crate::streams::readablestream::ReadableStream;
use crate::streams::stream::Stream;
use crate::streams::writablestream::WritableStream;

pub struct MemoryStream<'a> {
    pub vec: &'a mut Vec<u8>,
    pub pos: u32
}

impl <'a> MemoryStream<'a> {
    pub fn from_vec(vec: &'a mut Vec<u8>) -> MemoryStream<'a> {
        MemoryStream {
            vec,
            pos: 0
        }
    }
}

impl <'a> Stream for MemoryStream<'a> {
    fn get_length(&self) -> Option<u64> {
        Some(self.vec.len() as u64)
    }

    fn can_seek(&self) -> bool {
        true
    }

    fn get_position(&mut self) -> Option<u64> {
        Some(self.pos as u64)
    }

    fn set_position(&mut self, position: u64) -> bool {
        if position >= self.vec.len() as u64 {
            return false // Can't set position to this point
        }
        self.pos = position as u32;
        true
    }
}

impl <'a> ReadableStream<u8> for MemoryStream<'a> {
    fn can_read(&mut self) -> bool {
        self.pos < self.vec.len() as u32
    }

    fn available(&mut self) -> Option<u64> {
        let len = self.vec.len() as u32;
        if self.pos >= len {
            None
        }
        else {
            Some((len - self.pos) as u64)
        }
    }

    fn read(&mut self, array: &mut [u8]) -> crate::result::Result<usize> {
        let requested = array.len();
        
        let start = self.pos as usize;
        let end = start + requested;
        
        let fetched = self.vec.get(start .. end).unwrap_or(&[]);
        let amount_fetched = fetched.len();
        self.pos += amount_fetched as u32;
        if amount_fetched == requested {
            array.copy_from_slice(fetched);
            Ok(requested)
        }
        else {
            array[0..amount_fetched].clone_from_slice(fetched);
            Ok(amount_fetched)
        }
    }
}

impl <'a> WritableStream<u8> for MemoryStream<'a> {
    fn write(&mut self, value: &[u8]) {
        self.vec.append(&mut Vec::from(value));
    }

    fn can_write(&self) -> bool {
        !self.vec.len() >= isize::MAX as usize
    }
}