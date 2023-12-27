use bincode::{Decode, Encode};
use super::consts::{HEADER_SIGNATURE_STRING,PAGE_BYTES};

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Header {
    // The value should be always equal to MAGIC_HEADER_STRING constant
    header_string: [u8; HEADER_SIGNATURE_STRING.len()],
    // In the current version this value will always be equal to PAGE_SIZE_BYTES,
    page_bytes: u32,
}

impl Default for Header {
    fn default() -> Self {
        return Header {
            magic_header_string: HEADER_SIGNATURE_STRING,
            page_size_bytes: PAGE_BYTES,
        };
    }
}
