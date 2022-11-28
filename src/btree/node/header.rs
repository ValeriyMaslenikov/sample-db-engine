use bincode::{ Encode, Decode };

use crate::{ aliases::PageSpace, btree::{constants::NODE_HEADER_BYTES} };


#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone)]
pub(in crate::btree) enum NodeType {
    TableInternal,
    TableLeaf,
}

#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone)]
pub(in crate::btree) struct NodeHeader {
    pub(super) node_type: NodeType,
    pub(super) free_space_start_offset: PageSpace,
    pub(super) free_space_end_offset: PageSpace,
    pub(super) elements_count: PageSpace,
}

impl NodeHeader {
    pub(super) fn new_internal(page_size_bytes: PageSpace) -> NodeHeader {
        Self::new(page_size_bytes, NodeType::TableInternal)
    }

    pub(super) fn new_leaf(page_size_bytes: PageSpace) -> NodeHeader {
        Self::new(page_size_bytes, NodeType::TableLeaf)
    }

    fn new(page_size_bytes: PageSpace, node_type: NodeType) -> NodeHeader {
        NodeHeader {
            node_type,
            free_space_start_offset: NODE_HEADER_BYTES as PageSpace,
            free_space_end_offset: page_size_bytes,
            elements_count: 0,
        }
    }

    pub(in crate::btree) fn is_leaf(&self) -> bool {
        self.node_type == NodeType::TableLeaf
    }

    pub(in crate::btree) fn is_internal(&self) -> bool {
        self.node_type == NodeType::TableInternal
    }
}