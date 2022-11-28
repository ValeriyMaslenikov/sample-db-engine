use std::{ os::unix::prelude::FileExt, fs::File, io::ErrorKind, borrow::Cow };

use crate::{
    aliases::{ PageId },
    pager::constants::DATABASE_HEADER_BYTES,
};

use super::{
    constants::{ PAGE_SIZE_BYTES, METAPAGE_ID },
    Pager,
    PagerResult,
    errors::{ PageNotFound, ReadPageResult },
};

pub(super) fn do_save(file: &mut File, page_id: PageId, buffer: &[u8]) -> PagerResult<()> {
    // In the case of page zero the offset is indtended to be 0
    // In the case of page one the offset is equal to page size bytes, meaning
    // that page 0 occupies ranges between 0 and PAGE_SIZE_BYTES - 1,
    // the page 1 between  PAGE_SIZE_BYTES and (PAGE_SIZE_BYTES * 2) - 1
    let file_offset = page_id * PAGE_SIZE_BYTES;

    file.write_all_at(buffer, file_offset as u64)
}

pub(super) fn do_read_page(db_file: &File, page_id: PageId) -> ReadPageResult {
    let mut page_buffer = Pager::new_page_payload_buffer(None);
    let file_offset = (page_id * PAGE_SIZE_BYTES) as u64;

    let result = db_file.read_exact_at(&mut page_buffer, file_offset);

    if let Err(error) = result {
        let error_kind = error.kind();
        return match error_kind {
            ErrorKind::UnexpectedEof => Err(PageNotFound),
            _ => todo!("We dont support handling any IO errors"),
        };
    }

    Ok(page_buffer)
}

impl Pager {
    pub fn save_page(
        &mut self,
        page_payload: &[u8],
        page_id_opt: Option<PageId>
    ) -> PagerResult<PageId> {
        let page_id;

        let buffer_to_save;

        match page_id_opt {
            // New page
            None => {
                page_id = self.database_header.pages_count;
                self.database_header.pages_count += 1;

                self.save_db_header()?;
                buffer_to_save = Cow::from(page_payload);
            }
            Some(METAPAGE_ID) => {
                page_id = METAPAGE_ID;

                let mut page_buffer = Pager::new_page_buffer();

                self.database_header.encode_db_header(
                    &mut page_buffer[0..DATABASE_HEADER_BYTES as usize]
                );

                let page_buffer_payload_slice =
                    &mut page_buffer[DATABASE_HEADER_BYTES as usize..];
                assert!(page_buffer_payload_slice.len() == page_payload.len());
                page_buffer_payload_slice.clone_from_slice(page_payload);

                buffer_to_save = Cow::from(page_buffer);
            }
            Some(x) => {
                page_id = x;
                buffer_to_save = Cow::from(page_payload);
            }
        }

        assert!(buffer_to_save.len() == (PAGE_SIZE_BYTES as usize));

        do_save(&mut self.main_db_file, page_id, &buffer_to_save)?;

        Ok(page_id)
    }

    pub(super) fn save_db_header(&mut self) -> PagerResult<()> {
        self.database_header.save(&mut self.main_db_file)
    }

    pub(crate) fn read_page(&self, page_id: PageId) -> ReadPageResult {
        do_read_page(&self.main_db_file, page_id)
    }
}