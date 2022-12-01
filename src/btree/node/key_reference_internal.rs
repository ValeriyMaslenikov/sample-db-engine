use bincode::{ Encode, Decode };

use crate::{
    aliases::{ DatabaseKey, PageId, PageSpace },
    helpers::encoder::{ decode_unwrapped, encode },
};

use super::key_reference::KeyReferenceGeneral;

/// Single entry of items key reference will occupy this amount of bytes
const NON_LEAF_ITEMS_KEY_REFERENCE_BYTES: u8 = 8;

#[derive(Encode, Decode, PartialEq, Eq, Debug)]
pub(in crate::btree) struct KeyReferenceInternal {
    pub(in crate::btree) key: DatabaseKey,
    pub(in crate::btree) page_id: PageId,
}

impl KeyReferenceInternal {
    pub(super) fn new_item(key: DatabaseKey, page_id: PageSpace) -> Self {
        Self {
            key,
            page_id,
        }
    }
}

impl KeyReferenceGeneral for KeyReferenceInternal {
    fn offset_by_index(index: u32) -> usize {
        (index * (NON_LEAF_ITEMS_KEY_REFERENCE_BYTES as u32)) as usize
    }

    fn new_by_index(buf: &[u8], index: u32) -> Self {
        let offset = Self::offset_by_index(index);
        decode_unwrapped(&buf[offset..offset + (Self::bytes_per_item() as usize)])
    }

    fn save_by_index(&self, buf: &mut [u8], index: u32) {
        let offset = Self::offset_by_index(index);
        encode(&self, &mut buf[offset..offset + (Self::bytes_per_item() as usize)]);
    }

    fn bytes_per_item() -> u8 {
        NON_LEAF_ITEMS_KEY_REFERENCE_BYTES
    }
}

trait KeyReferenceLoader {}