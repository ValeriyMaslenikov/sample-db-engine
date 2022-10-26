use bincode::{ Decode, Encode };

use super::constants::{ MAGIC_HEADER_STRING, PAGE_SIZE_BYTES };

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct DatabaseHeader {
    // The value should be always equal to MAGIC_HEADER_STRING constant
    magic_header_string: [u8; MAGIC_HEADER_STRING.len()],
    // In the current version this value will always be equal to PAGE_SIZE_BYTES,
    page_size_bytes: u32,
}

impl Default for DatabaseHeader {
    fn default() -> Self {
        return DatabaseHeader {
            magic_header_string: MAGIC_HEADER_STRING,
            page_size_bytes: PAGE_SIZE_BYTES,
        };
    }
}