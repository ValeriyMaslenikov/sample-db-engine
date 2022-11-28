use std::cmp::Ordering;

use crate::{aliases::PageId, btree::node::common::{BTreeNodeEncodable, BTreeNode}};

use super::super::{
    node::{ leaf::LeafNode, internal::InternalNode },
};

/// Trait is used to limit the possible generics value for PageIdAndNode
pub(in crate::btree) trait BTreeNodeRepresentation {}

impl BTreeNodeRepresentation for &dyn BTreeNodeEncodable {}
impl BTreeNodeRepresentation for BTreeNode {}
impl BTreeNodeRepresentation for LeafNode {}
impl BTreeNodeRepresentation for InternalNode {}

#[derive(Clone)]
pub(in crate::btree) struct PagedNode<T: BTreeNodeRepresentation> {
    pub(in crate::btree)page_id: PageId,
    pub(in crate::btree)node: T,
}

impl<T: BTreeNodeRepresentation> PartialEq for PagedNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.page_id == other.page_id
    }
}

impl<T: BTreeNodeRepresentation> Eq for PagedNode<T> {}

impl<T: BTreeNodeRepresentation> PartialOrd for PagedNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: BTreeNodeRepresentation> Ord for PagedNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.page_id.cmp(&other.page_id);
    }
}

impl PagedNode<BTreeNode> {
    pub(in crate::btree)fn to_leaf(&self) -> PagedNode<LeafNode> {
        if let BTreeNode::Leaf(leaf_node) = self.node.clone() {
            return PagedNode { page_id: self.page_id, node: leaf_node };
        } else {
            unreachable!("Current enum value stores Internal node, but Leaf is requested")
        }
    }

    pub(in crate::btree)fn to_internal(&self) -> PagedNode<InternalNode> {
        if let BTreeNode::Internal(internal_node) = self.node.clone() {
            return PagedNode { page_id: self.page_id, node: internal_node };
        } else {
            unreachable!("Current enum value stores Leaf node, but Internal is requested")
        }
    }
}

impl Into<PagedNode<BTreeNode>> for PagedNode<LeafNode> {
    fn into(self) -> PagedNode<BTreeNode> {
        PagedNode { page_id: self.page_id, node: BTreeNode::Leaf(self.node) }
    }
}

impl Into<PagedNode<BTreeNode>> for PagedNode<InternalNode> {
    fn into(self) -> PagedNode<BTreeNode> {
        PagedNode { page_id: self.page_id, node: BTreeNode::Internal(self.node) }
    }
}
