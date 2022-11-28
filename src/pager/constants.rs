use crate::aliases::{PageSpace, PageId};

/// Defines the magic header string "Simple Data Engine", which is used to verify that the loaded
/// file is the database file of this engine
pub const MAGIC_HEADER_STRING: [u8; 18] = [
    83, 105, 109, 112, 108, 101, 32, 68, 97, 116, 97, 32, 69, 110, 103, 105, 110, 101,
];

/// It's reserved amount of bytes for Database header. The original structure occupies
/// less amount of bytes, but to avoid any restructuring for the sake of compatibliity
/// we will reserve additional bytes for that purpose.
pub const DATABASE_HEADER_BYTES: u8 = 100;

/// This value defines the number of bytes the single page occupies on the disk.
/// It completely depends on the Virtual Memory page size of the OS.
///
/// TODO: Make this page size configurable, as far as performance depends on the
/// actual page size within the OS/Hardware
pub const PAGE_SIZE_BYTES: PageSpace = 4096;

/// The first page looks like this:
/// +------------------------------------------+
/// |         Database header 100 bytes        |
/// +------------------------------------------+
/// |          Page contents 3996 bytes        |
/// |             ( 4096 - 100 )               |
/// |                                          |
/// |                                          |
/// |                                          |
/// |                                          |
/// |                                          |
/// +------------------------------------------+
pub const METAPAGE_SIZE_BYTES: PageSpace = PAGE_SIZE_BYTES - (DATABASE_HEADER_BYTES as PageSpace);

pub const METAPAGE_ID: PageId = 0;