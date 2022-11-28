use std::{ fs::File };

use log::{ info, debug };

use crate::{ pager::{ db_header::DatabaseHeader } };

use super::{ PagerResult, Pager };

impl Pager {
    pub(super) fn load_header_and_init_if_needed(
        main_db_file: &mut File
    ) -> PagerResult<DatabaseHeader> {
        debug!("Checking the file metadata to create the database header if it is absent");
        let database_header;
        if need_initialization(&main_db_file)? {
            debug!("Database file is empty – initializing the metapage with header");
            database_header = DatabaseHeader::default();
        } else {
            debug!("Database file is not empty – reading the metapage");
            database_header = DatabaseHeader::load(&main_db_file)?;
            info!("Header is loaded from the metapage: {:?}", database_header);
        }

        Ok(database_header)
    }
}

fn need_initialization(main_db_file: &File) -> PagerResult<bool> {
    let file_metadata = main_db_file.metadata()?;

    let file_length = file_metadata.len();

    Ok(file_length == 0)
}