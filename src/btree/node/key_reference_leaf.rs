use bincode::{ Encode, Decode };

use crate::{ aliases::{ DatabaseKey, PageSpace }, helpers::encoder::{ decode_unwrapped, encode } };

use super::{ helpers::data_space_offset, key_reference::KeyReferenceGeneral };

/// Single entry of items key reference will occupy this amount of bytes
const LEAF_ITEMS_KEY_REFERENCE_BYTES: u8 = 12;

#[derive(Encode, Decode, PartialEq, Eq, Debug)]
pub(in crate::btree) struct KeyReferenceLeaf {
    pub(in crate::btree) key: DatabaseKey,
    pub(in crate::btree) length: PageSpace,
    /// Relative offset within this page to specify where the data starts
    pub(in crate::btree) reference: u32,
}

impl KeyReferenceLeaf {
    pub(super) fn new_item(key: DatabaseKey, reference: PageSpace, length: PageSpace) -> Self {
        Self {
            key,
            reference,
            length,
        }
    }

    pub(super) fn item_data_range(&self) -> (usize, usize) {
        let start = data_space_offset(self.reference) as usize;
        (start, start + (self.length as usize))
    }
}

impl KeyReferenceGeneral for KeyReferenceLeaf {
    fn offset_by_index(index: u32) -> usize {
        (index * (LEAF_ITEMS_KEY_REFERENCE_BYTES as u32)) as usize
    }

    fn new_by_index(buf: &[u8], index: u32) -> Self {
        let offset = Self::offset_by_index(index);
        return decode_unwrapped(&buf[offset..offset + (LEAF_ITEMS_KEY_REFERENCE_BYTES as usize)]);
    }

    fn save_by_index(&self, buf: &mut [u8], index: u32) {
        let offset = Self::offset_by_index(index);
        encode(&self, &mut buf[offset..offset + (LEAF_ITEMS_KEY_REFERENCE_BYTES as usize)]);
    }

    fn bytes_per_item() -> u8 {
        return LEAF_ITEMS_KEY_REFERENCE_BYTES;
    }
}