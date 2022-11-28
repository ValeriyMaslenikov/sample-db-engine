use crate::aliases::{ DatabaseKey, PageSpace };

use super::{ leaf::LeafNode, internal::InternalNode, header::NodeHeader };

// ! ||--------------------------------------------------------------------------------||
// ! ||                                   BTree Node                                   ||
// ! ||--------------------------------------------------------------------------------||

#[derive(Eq, PartialEq, Clone)]
pub(in crate::btree) enum BTreeNode {
    Leaf(LeafNode),
    Internal(InternalNode),
}

impl BTreeNode {
    pub(in crate::btree) fn is_leaf(&self) -> bool {
        return matches!(self, BTreeNode::Leaf(..));
    }

    pub(in crate::btree) fn is_internal(&self) -> bool {
        return matches!(self, BTreeNode::Internal(..));
    }

    pub(in crate::btree) fn common(&self) -> &dyn BTreeNodeCommon {
        return match self {
            BTreeNode::Leaf(x) => x,
            BTreeNode::Internal(x) => x,
        };
    }

    pub(in crate::btree) fn internal(&self) -> &InternalNode {
        if let BTreeNode::Internal(internal_node) = self {
            return internal_node;
        } else {
            unreachable!("Current enum value stores Internal node, but Leaf is requested")
        }
    }

    pub(in crate::btree) fn leaf(&self) -> &LeafNode {
        if let BTreeNode::Leaf(leaf_node) = self {
            return leaf_node;
        } else {
            unreachable!("Current enum value stores Internal node, but Leaf is requested")
        }
    }
}

impl BTreeNodeEncodable for BTreeNode {
    fn header(&self) -> &NodeHeader {
        return self.common().header();
    }

    fn data_space(&self) -> &Vec<u8> {
        return self.common().data_space();
    }
}

pub(in crate::btree) trait BTreeNodeEncodable {
    fn header(&self) -> &NodeHeader;
    fn data_space(&self) -> &Vec<u8>;
}
pub(in crate::btree) trait BTreeNodeCommon: BTreeNodeEncodable {
    fn is_leaf(&self) -> bool {
        self.header().is_leaf()
    }

    fn is_internal(&self) -> bool {
        self.header().is_internal()
    }

    fn get_free_space(&self) -> PageSpace {
        self.header().free_space_end_offset - self.header().free_space_start_offset
    }

    fn len(&self) -> PageSpace {
        self.header().elements_count
    }

    fn is_index_present(&self, index: PageSpace) -> bool {
        return index < self.len();
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ! ||--------------------------------------------------------------------------------||
// ! ||                                 BTree Iterator                                 ||
// ! ||--------------------------------------------------------------------------------||
pub(in crate::btree) trait BTreeIterator<'iterator, I: Iterator> {
    fn iterator(&'iterator self) -> I;

    fn last(&'iterator self) -> Option<I::Item>;
    fn first(&'iterator self) -> Option<I::Item>;

    fn first_key(&self) -> DatabaseKey;
    fn last_key(&self) -> DatabaseKey;
}