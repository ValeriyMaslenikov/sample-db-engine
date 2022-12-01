use std::fs::File;

use bincode::{ Decode, Encode };

use crate::{
    aliases::PageId,
    pager::{ constants::DATABASE_HEADER_BYTES, persistance::do_read_page },
    helpers::encoder::{ encode, decode_unwrapped },
};

use super::{
    constants::{ MAGIC_HEADER_STRING, PAGE_SIZE_BYTES, METAPAGE_ID },
    PagerResult,
    persistance::do_save,
};

#[derive(Encode, Decode, PartialEq, Eq, Debug)]
pub(crate) struct DatabaseHeader {
    // The value should be always equal to MAGIC_HEADER_STRING constant
    pub magic_header_string: [u8; MAGIC_HEADER_STRING.len()],
    // In the current version this value will always be equal to PAGE_SIZE_BYTES
    pub page_size_bytes: u32,
    // Current number of occupied pages
    pub pages_count: PageId,
    // Page where the root of the tree starts, it's 0 after the init, but the id is changed
    // after the first split
    pub root_page_id: PageId,
}

impl Default for DatabaseHeader {
    fn default() -> Self {
        DatabaseHeader {
            magic_header_string: MAGIC_HEADER_STRING,
            page_size_bytes: PAGE_SIZE_BYTES,
            pages_count: 1,
            root_page_id: 0,
        }
    }
}

impl DatabaseHeader {
    pub(super) fn encode_db_header(&self, buffer: &mut [u8]) {
        assert!(buffer.len() == (DATABASE_HEADER_BYTES as usize));
        let encoded_database_header_bytes = encode(
            self,
            &mut buffer[0..DATABASE_HEADER_BYTES as usize]
        );
        assert!(encoded_database_header_bytes <= (DATABASE_HEADER_BYTES as usize));
    }

    pub(super) fn save(&self, db_file: &mut File) -> PagerResult<()> {
        let mut db_header_buffer = vec![0_u8; DATABASE_HEADER_BYTES as usize];
        self.encode_db_header(&mut db_header_buffer);

        do_save(db_file, METAPAGE_ID, &db_header_buffer)?;
        Ok(())
    }

    pub(super) fn load(db_file: &File) -> PagerResult<DatabaseHeader> {
        let metapage_buffer = do_read_page(db_file, METAPAGE_ID).unwrap();

        let decoded_header: DatabaseHeader = decode_unwrapped(
            &metapage_buffer[0..DATABASE_HEADER_BYTES as usize]
        );

        assert_eq!(decoded_header.magic_header_string, MAGIC_HEADER_STRING);

        Ok(decoded_header)
    }
}