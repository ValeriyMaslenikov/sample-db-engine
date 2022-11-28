use crate::aliases::{ DatabaseKey, PageSpace };

use super::{
    internal::InternalNode,
    key_reference_internal::KeyReferenceInternal,
    key_reference::KeyReferenceGeneral,
    common::{ BTreeIterator, BTreeNodeCommon },
};

#[allow(dead_code)]
pub(in crate::btree) struct InternalNodeIteratorItem {
    pub(in crate::btree) key_reference: KeyReferenceInternal,
    pub(in crate::btree) index: PageSpace,
}

pub(in crate::btree) struct InternalNodeIterator<'iterator> {
    current_asc_inx: PageSpace,
    current_desc_index: PageSpace,
    node: &'iterator InternalNode,
}

impl<'iterator> InternalNodeIterator<'iterator> {
    pub(super) fn new(node: &'iterator InternalNode) -> InternalNodeIterator<'iterator> {
        return InternalNodeIterator { current_asc_inx: 0, current_desc_index: node.len(), node };
    }
}

fn element_by_index(node: &InternalNode, index: PageSpace) -> Option<InternalNodeIteratorItem> {
    if !node.is_index_present(index) {
        return None;
    }

    let key_reference = KeyReferenceInternal::new_by_index(&node.data_space, index);
    Some(InternalNodeIteratorItem {
        index,
        key_reference,
    })
}

impl<'iterator> DoubleEndedIterator for InternalNodeIterator<'iterator> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.current_desc_index -= 1;

        element_by_index(self.node, self.current_desc_index)
    }
}

impl<'iterator> Iterator for InternalNodeIterator<'iterator> {
    type Item = InternalNodeIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        let index_to_return = self.current_asc_inx;

        self.current_asc_inx += 1;

        element_by_index(self.node, index_to_return)
    }
}

impl<'iterator> BTreeIterator<'iterator, InternalNodeIterator<'iterator>> for InternalNode {
    fn last(&'iterator self) -> Option<InternalNodeIteratorItem> {
        self.iterator().next_back()
    }

    fn first(&'iterator self) -> Option<InternalNodeIteratorItem> {
        self.iterator().next()
    }

    fn first_key(&self) -> DatabaseKey {
        self.first().unwrap().key_reference.key
    }

    fn last_key(&self) -> DatabaseKey {
        self.last().unwrap().key_reference.key
    }

    fn iterator(&'iterator self) -> InternalNodeIterator<'iterator> {
        return InternalNodeIterator::new(self);
    }
}