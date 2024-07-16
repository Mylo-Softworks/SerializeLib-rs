/// Indicates if this system is big endian
pub const IS_BIG_ENDIAN: bool = cfg!(target_endian = "big");

/// Indicates if serialization should be performed in big endian
pub static mut USE_BIG_ENDIAN: bool = IS_BIG_ENDIAN;

/// Sets the endianness to be of the target, assuming it is system
pub fn match_endian(bytes: &mut [u8]) {
    if IS_BIG_ENDIAN != unsafe { USE_BIG_ENDIAN } { 
        bytes.reverse();
    }
}

/// Sets the endianness to be of the target, assuming it is system
pub fn convert_endian(bytes: &mut [u8]) -> &mut [u8] {
    match_endian(bytes);
    bytes
}