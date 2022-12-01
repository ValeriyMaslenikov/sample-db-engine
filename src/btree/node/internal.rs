use bincode::{ Encode, Decode };

use crate::aliases::{ PageSpace, DatabaseKey, PageId };

use crate::btree::node::key_reference::KeyReferenceGeneral;

use crate::helpers::binary_search::binary_search_over_fn;

use super::common::{ BTreeNodeEncodable, BTreeNodeCommon, BTreeIterator };
use super::leaf::LeafNode;
use super::{
    header::NodeHeader,
    helpers::data_space_offset,
    key_reference_internal::KeyReferenceInternal,
};

#[derive(Encode, Decode, PartialEq, Eq, Clone)]
pub(in crate::btree) struct InternalNode {
    pub(in crate::btree) header: NodeHeader,
    pub(in crate::btree) data_space: Vec<u8>,
}

impl BTreeNodeEncodable for InternalNode {
    fn header(&self) -> &NodeHeader {
        &self.header
    }

    fn data_space(&self) -> &Vec<u8> {
        &self.data_space
    }
}
impl BTreeNodeCommon for InternalNode {}

#[derive(PartialEq)]
enum FindFor {
    Insert,
    Child,
}

impl InternalNode {
    pub(in crate::btree) fn new(page_size_bytes: PageSpace) -> Self {
        let header = NodeHeader::new_internal(page_size_bytes);
        InternalNode {
            data_space: vec![0_u8; data_space_offset(header.free_space_end_offset) as usize],
            header,
        }
    }

    pub(in crate::btree) fn find_position_for_child(
        &self,
        target_key: DatabaseKey
    ) -> (PageSpace, KeyReferenceInternal) {
        if let (inx, Some(key_ref)) = self.find_position_for(target_key, FindFor::Child) {
            (inx, key_ref)
        } else {
            unreachable!();
        }
    }

    pub(in crate::btree) fn find_position_for_insert(
        &self,
        target_key: DatabaseKey
    ) -> (PageSpace, Option<KeyReferenceInternal>) {
        self.find_position_for(target_key, FindFor::Insert)
    }

    /// Knowing that Key Ref data for Non-leaf node will look like this:
    ///   ┌───────────────┬──────────────┐
    ///   │ 10            │ Page 2       │ Page 2 contains node for range: -inf to 10
    ///   ├───────────────┼──────────────┤
    ///   │ 25            │ Page 4       │ Page 4 contains node for range: 11 to 25
    ///   ├───────────────┼──────────────┤
    ///   │ ...           │ ...          │
    ///   ├───────────────┼──────────────┤
    ///   │ 182           │ Page 53      │
    ///   ├───────────────┼──────────────┤
    ///   │ 200           │ Page 40      │ Page 40 contains node for range: 183 to 200
    ///   └───────────────┴──────────────┘
    ///
    /// We apply binary search for searching the child pages of this Node.
    ///
    /// Method responsible for finding the right position of the element and
    /// is used for several places:
    /// - To determine the position where the new dividers should be put
    /// - Find the child page having the key (during the search, insertion, deletion, etc.)
    /// - Replacing and determining new dividers during the split of the node to multiple nodes
    ///
    /// Depending on intention we will receive different positions:
    /// - insert new element, position may be bigger than the existing elements
    /// - determine the child – position will always be within the range between (0..node.len()-1)
    fn find_position_for(
        &self,
        target_key: DatabaseKey,
        intention: FindFor
    ) -> (PageSpace, Option<KeyReferenceInternal>) {
        assert!(
            intention != FindFor::Child || self.len() != 0,
            "Caller asked for child when the node is empty"
        );

        let key_ref_by_inx = |inx| KeyReferenceInternal::new_by_index(&self.data_space, inx);
        let key_by_key_ref = |key_ref: &KeyReferenceInternal| key_ref.key;

        let (inx_insertion_point, _) = binary_search_over_fn(
            target_key,
            self.len(),
            &key_ref_by_inx,
            &key_by_key_ref
        );

        // It's out of scope of existing dividers, we should use
        if !self.is_index_present(inx_insertion_point) {
            // Caller looks for child – the index should be present in this node
            match intention {
                FindFor::Child => {
                    assert_ne!(inx_insertion_point, 0);
                    // As far as we didn't find the position for that element - that may only mean that
                    // potential position for that item is rightmost element (which is high key)
                    let last_inx = self.len() - 1;
                    return (last_inx, Some(key_ref_by_inx(last_inx)));
                }
                FindFor::Insert => {
                    // As far as someone want to insert new pair (divider,page_id),
                    // we will return the new index of this element
                    return (inx_insertion_point, None);
                }
            }
        }

        let key_ref = key_ref_by_inx(inx_insertion_point);

        (inx_insertion_point, Some(key_ref))
    }

    fn sorted_insert(&mut self, index: PageSpace, divider: DatabaseKey, page_id: PageId) {
        let key_ref = KeyReferenceInternal::new_item(divider, page_id);

        if index < self.len() {
            // Shift elements between to one position to fit the new element
            let shift_range = (
                KeyReferenceInternal::offset_by_index(index),
                KeyReferenceInternal::offset_by_index(self.len()),
            );
            let shift_to = KeyReferenceInternal::offset_by_index(index + 1);

            self.data_space.copy_within(shift_range.0..shift_range.1, shift_to);
        }

        key_ref.save_by_index(&mut self.data_space, index);

        self.header.elements_count += 1;
        self.header.free_space_start_offset += KeyReferenceInternal::bytes_per_item() as PageSpace;
    }

    pub(in crate::btree) fn put(&mut self, divider: DatabaseKey, page_id: PageId) {
        assert!(self.can_fit_more());
        let (inx, _) = self.find_position_for_insert(divider);
        self.sorted_insert(inx, divider, page_id)
    }

    pub(in crate::btree) fn can_fit_more(&self) -> bool {
        self.get_free_space() >= (KeyReferenceInternal::bytes_per_item() as u32)
    }

    pub(in crate::btree) fn get_child_page_id_by_key(&self, key: u32) -> PageId {
        self.find_position_for_child(key).1.page_id
    }

    pub(in crate::btree) fn key_reference_for(&self, node: &LeafNode) -> KeyReferenceInternal {
        self.find_position_for_child(node.first_key()).1
    }

    ///
    /// some_key_from_node is used to use binary search for fast search of the existing reference
    ///
    pub(in crate::btree) fn replace_divider(
        &mut self,
        for_key: DatabaseKey,
        new_divider: DatabaseKey,
        page_id: PageId
    ) {
        let (inx, mut key_ref) = self.find_position_for_child(for_key);

        assert_eq!(key_ref.page_id, page_id);

        key_ref.key = new_divider;
        key_ref.save_by_index(&mut self.data_space, inx);
    }

    pub(in crate::btree) fn is_rightmost_child(&self, page_id: PageId) -> bool {
        self.last().unwrap().key_reference.page_id == page_id
    }
}

#[cfg(test)]
mod find_position_for_test {
    use super::*;

    mod for_insert {
        use super::*;

        #[test]
        fn two_dividers_minus_infinity_expected() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(0);

            assert_eq!(inx, 0);
            assert!(key_ref_opt.is_some());
            let key_ref = key_ref_opt.unwrap();
            assert_eq!(key_ref.key, 6);
            assert_eq!(key_ref.page_id, 0);
        }

        #[test]
        fn two_dividers_before_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(10);

            assert_eq!(inx, 1);
            assert!(key_ref_opt.is_some());
            let key_ref = key_ref_opt.unwrap();
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }

        #[test]
        fn two_dividers_equal_to_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(18);

            assert_eq!(inx, 1);
            assert!(key_ref_opt.is_some());
            let key_ref = key_ref_opt.unwrap();
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }

        #[test]
        fn two_dividers_more_than_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(10000);

            assert_eq!(inx, 2);
            assert!(key_ref_opt.is_none());
        }

        #[test]
        fn separator_equal_to_target_key() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(6);

            assert_eq!(inx, 0);
            assert!(key_ref_opt.is_some());
            let key_ref = key_ref_opt.unwrap();
            assert_eq!(key_ref.key, 6);
            assert_eq!(key_ref.page_id, 0);
        }

        #[test]
        fn separator_equal_to_high_key() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref_opt) = node.find_position_for_insert(18);

            assert_eq!(inx, 1);
            assert!(key_ref_opt.is_some());
            let key_ref = key_ref_opt.unwrap();
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }
    }
    mod for_child {
        use super::*;

        #[test]
        fn two_dividers_minus_infinity_expected() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(0);

            assert_eq!(inx, 0);
            assert_eq!(key_ref.key, 6);
            assert_eq!(key_ref.page_id, 0);
        }

        #[test]
        fn two_dividers_before_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(10);

            assert_eq!(inx, 1);
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }

        #[test]
        fn two_dividers_equal_to_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(18);

            assert_eq!(inx, 1);
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }

        #[test]
        fn two_dividers_more_than_high_key_return_last_key_ref() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(10000);

            assert_eq!(inx, 1);
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }

        #[test]
        fn separator_equal_to_target_key() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(6);

            assert_eq!(inx, 0);
            assert_eq!(key_ref.key, 6);
            assert_eq!(key_ref.page_id, 0);
        }

        #[test]
        fn separator_equal_to_high_key() {
            let mut node = InternalNode::new(1024);
            node.put(6, 0);
            node.put(18, 1);

            let (inx, key_ref) = node.find_position_for_child(18);

            assert_eq!(inx, 1);
            assert_eq!(key_ref.key, 18);
            assert_eq!(key_ref.page_id, 1);
        }
    }
}