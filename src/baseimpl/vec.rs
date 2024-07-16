use crate::serializable::Serializable;
use crate::streams::readablestream::ReadableByteStream;
use crate::streams::writablestream::WritableStream;

impl <T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, writer: &mut impl WritableStream<u8>) {
        (self.len() as u32).serialize(writer);
        
        for item in self.iter() {
            item.serialize(writer);
        }
    }

    fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
        let count = u32::deserialize(reader);
        let mut vec = Vec::new();

        for _ in 0..count {
            vec.push(T::deserialize(reader));
        }
        
        vec
    }
}