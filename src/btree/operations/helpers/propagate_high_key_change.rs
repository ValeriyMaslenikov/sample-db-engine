use crate::{
    btree::{
        node::{
            internal_iterator::InternalNodeIteratorItem,
            key_reference_internal::KeyReferenceInternal, common::BTreeIterator,
        },
        structures::{ tree_breadcrumbs::TreeBreadcrumbs },
        persistance::saver::save_paged_node,
    },
    aliases::{ PageId, DatabaseKey },
    pager::{ Pager, PagerResult },
};

// This method is called when we found that to some of the Leaf nodes were pushed
// new value with the highest key. This may require changing from 1 internal node up to whole tree
// up to the root node (in case if it's the rightmost leaf node)
pub(in super::super) fn propagate_high_key_change(
    pager: &mut Pager,
    leaf_page_id: PageId,
    new_high_key: DatabaseKey,
    breadcrumbs: &TreeBreadcrumbs
) -> PagerResult<()> {
    assert!(breadcrumbs.has_parents());

    let mut child_page_id = leaf_page_id;

    let mut parents_pointer = 0;
    loop {
        // We assume this method is called only for rightmost pages
        let paged_node_opt = breadcrumbs.get_parent(parents_pointer);

        if paged_node_opt.is_none() {
            return Ok(());
        }

        let paged_node_cell = paged_node_opt.unwrap();
        let mut paged_node = paged_node_cell.borrow_mut();

        let InternalNodeIteratorItem {
            key_reference: KeyReferenceInternal {
                key: rightmost_divider,
                page_id: rightmost_page_id,
            },
            ..
        } = paged_node.node.last().unwrap();

        paged_node.node.replace_divider(rightmost_divider, new_high_key, rightmost_page_id);

        let is_rightmost = paged_node.node.is_rightmost_child(child_page_id);
        child_page_id = paged_node.page_id;

        save_paged_node(pager, &paged_node)?;

        if !is_rightmost {
            return Ok(());
        }
        parents_pointer += 1;
    }
}