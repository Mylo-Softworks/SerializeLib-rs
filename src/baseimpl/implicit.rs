use crate::serializable::Serializable;
use crate::streams::readablestream::ReadableByteStream;
use crate::streams::writablestream::WritableStream;

impl <T: Serializable> Serializable for Option<T> {
    fn serialize(&self, writer: &mut impl WritableStream<u8>) {
        // Indicate null
        if let Some(value) = self { 
            1_u8.serialize(writer);
            value.serialize(writer);
        }
        else { 
            0_u8.serialize(writer); // Write null byte
        }
    }

    fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
        let is_null = reader.read_one().unwrap().unwrap() == 0_u8;
        if is_null { None }
        else { 
            Some(T::deserialize(reader))
        }
    }
}