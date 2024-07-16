use std::mem::size_of;
use crate::endian::USE_BIG_ENDIAN;
use crate::serializable::Serializable;
use crate::streams::readablestream::ReadableByteStream;
use crate::streams::writablestream::WritableStream;

macro_rules! impl_serializable_auto {
    ($t:ty) => {
        impl Serializable for $t {
            fn serialize(&self, writer: &mut impl WritableStream<u8>) {
                writer.write(&mut if unsafe { USE_BIG_ENDIAN } {
                    self.to_be_bytes()
                }
                else {
                    self.to_le_bytes()
                });
            }

            fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
                let mut buffer = [0;size_of::<$t>()];
                reader.read_simple(&mut buffer).unwrap();
                if unsafe { USE_BIG_ENDIAN } { 
                    <$t>::from_be_bytes(buffer)
                }
                else {
                    <$t>::from_le_bytes(buffer)
                }
            }
        }
    };
}

macro_rules! impl_serializable_auto_multiple {
    ($($t:ty)*) => {
        $(
            impl_serializable_auto!($t);
        )*
    };
}

impl_serializable_auto_multiple!(u8 i8 u16 i16 u32 i32 f32 u64 i64 f64 u128 i128 usize isize);

impl Serializable for char {
    fn serialize(&self, writer: &mut impl WritableStream<u8>) {
        (*self as u32).serialize(writer)
    }

    fn deserialize(reader: &mut impl ReadableByteStream) -> Self {
        char::from_u32(u32::deserialize(reader)).unwrap()
    }
}