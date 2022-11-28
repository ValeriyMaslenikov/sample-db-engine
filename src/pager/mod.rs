use log::{ debug, info };
use std::fs::File;
use std::io::{ Error as IOError };
use std::{ result::Result };

use crate::aliases::{ PageId, PageBuffer };

use self::constants::{ PAGE_SIZE_BYTES, METAPAGE_SIZE_BYTES };
use self::db_header::DatabaseHeader;

pub mod constants;
pub mod errors;
mod db_header;
mod initializer;
mod persistance;

pub type PagerResult<T> = Result<T, IOError>;

#[derive(Debug)]
pub struct Config {
    pub main_db_path: String,
}

#[derive(Debug)]
pub struct Pager {
    pub main_db_file: File,
    database_header: DatabaseHeader,
}

impl Pager {
    pub fn new(config: Config) -> PagerResult<Self> {
        debug!("Trying to open file {}, or create if it doesn't exist", &config.main_db_path);

        let mut main_db_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&config.main_db_path)?;

        info!("Database file by path {} is opened successfully", &config.main_db_path);

        let database_header = Self::load_header_and_init_if_needed(&mut main_db_file)?;

        Ok(Pager {
            database_header,
            main_db_file,
        })
    }

    pub fn root_page_id(&self) -> PageId {
        self.database_header.root_page_id
    }

    pub fn new_root(&mut self, new_root_page_id: PageId) -> PagerResult<()> {
        self.database_header.root_page_id = new_root_page_id;
        self.save_db_header()
    }

    pub(crate) fn is_metapage(&self, page_id: u32) -> bool {
        page_id == 0
    }

    #[inline(always)]
    pub(crate) fn new_page_payload_buffer(page_id: Option<PageId>) -> PageBuffer {
        let page_size = (match page_id {
            Some(0) => METAPAGE_SIZE_BYTES,
            _ => PAGE_SIZE_BYTES,
        }) as usize;
        vec![0_u8; page_size]
    }

    pub(super) fn new_page_buffer() -> PageBuffer {
        vec![0_u8; PAGE_SIZE_BYTES as usize]
    }
}