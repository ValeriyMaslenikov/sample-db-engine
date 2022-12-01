use crate::{ aliases::{ PageSpace }, btree::constants::NODE_HEADER_BYTES };

pub(super) fn data_space_offset(absolute_offset: PageSpace) -> PageSpace {
    absolute_offset - (NODE_HEADER_BYTES as PageSpace)
}