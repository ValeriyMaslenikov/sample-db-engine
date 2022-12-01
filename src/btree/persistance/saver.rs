use crate::{
    pager::{ Pager, PagerResult },
    btree::{
        structures::{ paged_node::{ BTreeNodeRepresentation, PagedNode } },
        constants::NODE_HEADER_BYTES,
        node::common::BTreeNodeEncodable,
    },
    aliases::PageId,
    helpers::encoder::encode,
};

pub(in crate::btree) fn save_paged_node<T>(
    pager: &mut Pager,
    paged_node: &PagedNode<T>
) -> PagerResult<PageId>
    where T: BTreeNodeEncodable + BTreeNodeRepresentation
{
    save_node(pager, &paged_node.node, Some(paged_node.page_id))
}

pub(in crate::btree) fn save_node<T>(
    pager: &mut Pager,
    node: &T,
    page_id_opt: Option<PageId>
) -> PagerResult<PageId>
    where T: BTreeNodeEncodable
{
    let mut page_buffer = Pager::new_page_payload_buffer(page_id_opt);
    node_to_buffer(node, &mut page_buffer);
    let page_id = pager.save_page(&page_buffer, page_id_opt)?;

    Ok(page_id)
}

pub(super) fn node_to_buffer<T: BTreeNodeEncodable>(node: &T, buffer: &mut [u8]) {
    encode(&node.header(), &mut buffer[0..NODE_HEADER_BYTES as usize]);

    assert_eq!(buffer.len(), node.data_space().len() + (NODE_HEADER_BYTES as usize));

    buffer[NODE_HEADER_BYTES as usize..].clone_from_slice(node.data_space());
}