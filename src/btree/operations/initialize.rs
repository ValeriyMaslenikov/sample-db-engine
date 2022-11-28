use log::debug;

use crate::{
    pager::{ Pager, constants::{ METAPAGE_SIZE_BYTES, METAPAGE_ID, DATABASE_HEADER_BYTES } },
    btree::{
        node::{ leaf::LeafNode, header::NodeHeader },
        persistance::{ saver::save_node },
        constants::NODE_HEADER_BYTES,
    },
    helpers::encoder::decode,
};

fn create_root_node() -> LeafNode {
    // When we initializing the root page it starts to be the leaf page, cause we don't need the
    // complex structures to find the elements first.
    // Once it grows, the root page becomes the "breadcrumbs" to the path to real value (leaf)
    LeafNode::new(METAPAGE_SIZE_BYTES)
}

fn need_initialization(pager: &Pager) -> bool {
    // Try to load the metapage:
    let read_result = pager.read_page(METAPAGE_ID);

    if read_result.is_err() {
        return true;
    }

    let page_buffer = read_result.unwrap();
    let start_offset = DATABASE_HEADER_BYTES as usize;
    let end_offset = start_offset + (NODE_HEADER_BYTES as usize);
    let node_header_buffer = &page_buffer[start_offset..end_offset];

    let result = decode::<NodeHeader>(node_header_buffer);

    return result.is_err();
}

// Clients will read as "initialize root if  needed"
pub(crate) fn root_node_if_needed(pager: &mut Pager) {
    if need_initialization(&pager) {
        debug!("Initializing the root node within the metapage");
        let root_node = create_root_node();

        save_node(pager, &root_node, Some(METAPAGE_ID)).unwrap();
    }
}