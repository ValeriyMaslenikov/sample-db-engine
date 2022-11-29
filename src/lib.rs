use aliases::PageSpace;

pub mod connection;
mod pager;
mod helpers;
mod btree;
mod aliases;

pub const PAGE_SIZE: PageSpace = pager::constants::PAGE_SIZE_BYTES;