use crate::serializable::Serializable;
use crate::streams::readablestream::ReadableByteStream;
use crate::streams::writablestream::WritableStream;

impl Serializable for String {
    fn serialize(&self, writer: &mut impl WritableStream<u8>) {
        let utf8bytes = self.as_bytes();
        (utf8bytes.len() as u32).serialize(writer);
        writer.write(utf8bytes);
    }

    fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
        let len = u32::deserialize(reader);
        let mut buffer = vec![0_u8; len as usize];
        reader.read(&mut buffer).unwrap_or_default();
        String::from_utf8(buffer).unwrap()
    }
}