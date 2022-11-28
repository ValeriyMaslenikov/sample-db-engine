use bincode::{ Encode, Decode };

use crate::{
    aliases::{ PageSpace, DatabaseKey },
    btree::{node::{
        key_reference_leaf::KeyReferenceLeaf,
        key_reference::KeyReferenceGeneral, common::BTreeIterator,
    }, constants::NODE_HEADER_BYTES},
    pager::constants::PAGE_SIZE_BYTES,
    helpers::div_ceil::div_ceil,
};

use super::{ header::NodeHeader, helpers::data_space_offset, common::{BTreeNodeEncodable, BTreeNodeCommon} };

use crate::helpers::binary_search::binary_search_over_fn;

#[derive(Encode, Decode, PartialEq, Eq, Clone)]
pub(in crate::btree) struct LeafNode {
    pub(in crate::btree) header: NodeHeader,
    pub(in crate::btree) data_space: Vec<u8>,
}

impl BTreeNodeEncodable for LeafNode {
    fn header(&self) -> &NodeHeader {
        return &self.header;
    }

    fn data_space(&self) -> &Vec<u8> {
        return &self.data_space;
    }
}
impl BTreeNodeCommon for LeafNode {}

impl LeafNode {
    pub(in crate::btree)fn new(page_size_bytes: PageSpace) -> Self {
        let header = NodeHeader::new_leaf(page_size_bytes);
        LeafNode {
            data_space: vec![0_u8; data_space_offset(header.free_space_end_offset) as usize],
            header,
        }
    }

    pub(in crate::btree)fn split(&mut self) -> Self {
        assert!(self.len() > 1, "We may split the node only in case if it contains 2+ elements");

        /*
            2 elements = 1 / 1
            3 elements = 2 / 1
            4 elements = 2 / 2...
            */
        let leave_elements_in_self = div_ceil(self.len(), 2);

        let mut new_node = Self::new(PAGE_SIZE_BYTES);
        let new_node_element_count = self.len() - leave_elements_in_self;

        // Reversing the iterator as far as removing from the end is a cheap operation
        let iterator = self
            .iterator()
            .rev()
            .take(new_node_element_count as usize);

        let mut delete_from_self = Vec::with_capacity(new_node_element_count as usize);

        for item in iterator {
            new_node.put(item.key_reference.key, item.value);
            delete_from_self.push(item.index);
        }

        for item_index in delete_from_self {
            self.delete_by_index(item_index);
        }

        new_node
    }

    pub(crate) fn can_fit(&self, value: &[u8]) -> bool {
        assert!(self.is_leaf());

        let will_occupy = value.len() + (KeyReferenceLeaf::bytes_per_item() as usize);
        let free_space = self.get_free_space();
        will_occupy <= (free_space as usize)
    }

    pub(crate) fn put(&mut self, key: DatabaseKey, value: &[u8]) {
        assert!(self.is_leaf());
        let (index, existing_key_ref) = self.find_position_for(key);

        if existing_key_ref.is_none() {
            self.sorted_insert(index, key, value);
        } else {
            todo!();
        }
    }

    fn place_into_heap(&mut self, value: &[u8]) {
        self.header.free_space_end_offset -= value.len() as PageSpace;

        assert!(
            (self.header.free_space_end_offset as usize) + value.len() <=
                (NODE_HEADER_BYTES as usize) + self.data_space.len()
        );

        let data_space_offset = data_space_offset(self.header.free_space_end_offset) as usize;

        self.data_space[data_space_offset..data_space_offset + value.len()].clone_from_slice(
            value
        );
    }

    fn sorted_insert(&mut self, index: PageSpace, key: DatabaseKey, value: &[u8]) {
        self.place_into_heap(value);

        let reference = self.header.free_space_end_offset;
        let key_ref = KeyReferenceLeaf::new_item(key, reference, value.len() as PageSpace);

        if index < self.len() {
            let shift_range = (
                KeyReferenceLeaf::offset_by_index(index),
                KeyReferenceLeaf::offset_by_index(self.len()),
            );
            let shift_to = KeyReferenceLeaf::offset_by_index(index + 1);

            self.data_space.copy_within(shift_range.0..shift_range.1, shift_to);
        }

        key_ref.save_by_index(&mut self.data_space, index);

        self.header.elements_count += 1;
        self.header.free_space_start_offset += KeyReferenceLeaf::bytes_per_item() as PageSpace;
    }

    pub(in crate::btree)fn can_fit_into_empty_node(value: &[u8]) -> bool {
        let empty_node_can_fit =
            PAGE_SIZE_BYTES -
            (NODE_HEADER_BYTES as u32) -
            (KeyReferenceLeaf::bytes_per_item() as u32);
        // Allow 2/3
        let allow_empty_node_to_fit = (empty_node_can_fit / 3) * 2;
        return value.len() <= (allow_empty_node_to_fit as usize);
    }

    fn delete_by_index(&mut self, inx: PageSpace) {
        if inx == self.len() - 1 {
            self.header.free_space_start_offset -= KeyReferenceLeaf::bytes_per_item() as PageSpace;
            self.header.elements_count -= 1;
        } else {
            todo!("implemented only if the element in the last one");
        }
    }

    pub(in crate::btree)fn find_position_for(
        &self,
        target_key: DatabaseKey
    ) -> (PageSpace, Option<KeyReferenceLeaf>) {
        assert!(self.is_leaf());
        let elements_in_node: PageSpace = self.len();

        let key_ref_by_inx = |inx| KeyReferenceLeaf::new_by_index(&self.data_space, inx);
        let key_by_key_ref = |key_ref: &KeyReferenceLeaf| key_ref.key;

        return binary_search_over_fn(
            target_key,
            elements_in_node,
            &key_ref_by_inx,
            &key_by_key_ref
        );
    }

    pub(crate) fn gt_high_key(&self, key: u32) -> bool {
        return !self.is_empty() && key > self.last_key();
    }
}