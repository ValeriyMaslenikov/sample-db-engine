use crate::aliases::PageId;
use crate::btree::constants::NODE_HEADER_BYTES;
use crate::btree::node::common::BTreeNode;
use crate::btree::node::header::NodeHeader;
use crate::btree::node::leaf::LeafNode;
use crate::btree::node::internal::InternalNode;
use crate::btree::structures::paged_node::PagedNode;
use crate::helpers::encoder::decode_unwrapped;
use crate::pager::constants::DATABASE_HEADER_BYTES;
use crate::pager::{ Pager, PagerResult };

pub(in crate::btree) fn buffer_to_node(node_buffer: &[u8]) -> BTreeNode {
    let header: NodeHeader = decode_unwrapped(&node_buffer[0..NODE_HEADER_BYTES as usize]);

    let data_space = node_buffer[NODE_HEADER_BYTES as usize..].to_vec();
    if header.is_internal() {
        return BTreeNode::Internal(InternalNode { header, data_space });
    } else if header.is_leaf() {
        return BTreeNode::Leaf(LeafNode { header, data_space });
    } else {
        unreachable!("We support only leafs and internal nodes");
    }
}

pub(in crate::btree) fn load_node(pager: &Pager, page_id: PageId) -> PagerResult<BTreeNode> {
    let read_result = pager.read_page(page_id);

    if read_result.is_err() {
        todo!();
    }

    let page_buffer = read_result.unwrap();

    let page_offset = (if pager.is_metapage(page_id) {
        DATABASE_HEADER_BYTES
    } else {
        0
    }) as usize;
    let node_buf_slice = &page_buffer[page_offset..];

    Ok(buffer_to_node(node_buf_slice))
}

pub(in crate::btree) fn load_root_node(pager: &Pager) -> PagedNode<BTreeNode> {
    PagedNode::<BTreeNode> {
        page_id: pager.root_page_id(),
        node: load_node(pager, pager.root_page_id()).unwrap(),
    }
}