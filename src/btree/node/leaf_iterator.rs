use crate::aliases::{ DatabaseKey, PageSpace };

use super::{
    leaf::LeafNode,
    key_reference_leaf::KeyReferenceLeaf,
    key_reference::KeyReferenceGeneral,
    common::{ BTreeIterator, BTreeNodeCommon },
};

pub(in crate::btree) struct LeafNodeIteratorItem<'a> {
    pub(in crate::btree) key_reference: KeyReferenceLeaf,
    pub(in crate::btree) value: &'a [u8],
    pub(in crate::btree) index: PageSpace,
}

pub(in crate::btree) struct LeafNodeIterator<'iterator> {
    current_asc_inx: PageSpace,
    current_desc_index: PageSpace,
    node: &'iterator LeafNode,
}

impl<'iterator> LeafNodeIterator<'iterator> {
    pub(super) fn new(node: &'iterator LeafNode) -> LeafNodeIterator<'iterator> {
        return LeafNodeIterator { current_asc_inx: 0, current_desc_index: node.len(), node };
    }
}

fn element_by_index(node: &LeafNode, index: PageSpace) -> Option<LeafNodeIteratorItem> {
    if !node.is_index_present(index) {
        return None;
    }

    let key_reference = KeyReferenceLeaf::new_by_index(&node.data_space, index);

    let data_space_range = key_reference.item_data_range();
    let value = &node.data_space[data_space_range.0..data_space_range.1];

    Some(LeafNodeIteratorItem {
        key_reference,
        value,
        index,
    })
}

impl<'iterator> DoubleEndedIterator for LeafNodeIterator<'iterator> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.current_desc_index -= 1;

        element_by_index(self.node, self.current_desc_index)
    }
}

impl<'iterator> Iterator for LeafNodeIterator<'iterator> {
    type Item = LeafNodeIteratorItem<'iterator>;

    fn next(&mut self) -> Option<Self::Item> {
        let index_to_return = self.current_asc_inx;

        self.current_asc_inx += 1;

        element_by_index(self.node, index_to_return)
    }
}

impl<'iterator> BTreeIterator<'iterator, LeafNodeIterator<'iterator>> for LeafNode {
    fn last(&'iterator self) -> Option<LeafNodeIteratorItem<'iterator>> {
        assert!(self.is_leaf());
        self.iterator().next_back()
    }

    fn first(&'iterator self) -> Option<LeafNodeIteratorItem<'iterator>> {
        assert!(self.is_leaf());
        self.iterator().next()
    }

    fn first_key(&self) -> DatabaseKey {
        self.first().unwrap().key_reference.key
    }

    fn last_key(&self) -> DatabaseKey {
        self.last().unwrap().key_reference.key
    }

    fn iterator(&'iterator self) -> LeafNodeIterator<'iterator> {
        assert!(self.is_leaf());

        return LeafNodeIterator::new(self);
    }
}