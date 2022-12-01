use crate::aliases::PageId;
use crate::btree::node::common::{ BTreeNode, BTreeIterator };
use crate::btree::node::internal::InternalNode;
use crate::btree::node::key_reference_internal::KeyReferenceInternal;
use crate::btree::operations::helpers::compute_dividers::compute_dividers;
use crate::btree::persistance::loader::{ load_node, load_root_node };
use crate::btree::persistance::saver::{ save_node, save_paged_node };
use crate::btree::structures::paged_node::{ PagedNode, BTreeNodeRepresentation };
use crate::btree::structures::tree_breadcrumbs::{ TreeBreadcrumbs };
use crate::pager::PagerResult;
use crate::pager::constants::PAGE_SIZE_BYTES;
use crate::{ aliases::DatabaseKey, btree::node::leaf::LeafNode };
use crate::pager::Pager;
use super::helpers::propagate_high_key_change::{ propagate_high_key_change };

struct SplitResult<T: BTreeNodeRepresentation> {
    pub(super) new_child: PagedNode<T>,
}

pub(crate) fn do_insert(pager: &mut Pager, key: DatabaseKey, value: &[u8]) -> PagerResult<()> {
    if !LeafNode::can_fit_into_empty_node(value) {
        todo!();
    }

    let mut tree_breadcrumbs = find_leaf_for(pager, key);

    let new_child_cell;
    let paged_leaf_cell = tree_breadcrumbs.leaf().unwrap();
    let mut paged_leaf = paged_leaf_cell.borrow_mut();

    let (_, existing_key_ref) = paged_leaf.node.find_position_for(key);

    if existing_key_ref.is_some() {
        todo!("Case when id is already exists");
    }

    let split_result;

    if !paged_leaf.node.can_fit(value) {
        let to_split_page_id = paged_leaf.page_id;
        split_result = split_and_persist_nodes(pager, &mut paged_leaf, &mut tree_breadcrumbs)?;

        let parent_paged_node_cell = tree_breadcrumbs.last_parent().unwrap();
        let parent_paged_node = parent_paged_node_cell.borrow();

        let (_, KeyReferenceInternal { page_id: insert_into_page_id, .. }) =
            parent_paged_node.node.find_position_for_child(key);

        if insert_into_page_id != to_split_page_id {
            assert_eq!(insert_into_page_id, split_result.new_child.page_id);
            tree_breadcrumbs.push(split_result.new_child.into());
            new_child_cell = tree_breadcrumbs.leaf().unwrap();
            paged_leaf = new_child_cell.borrow_mut();
        }
    }

    let actualize_high_key = paged_leaf.node.gt_high_key(key) && tree_breadcrumbs.has_parents();

    assert!(paged_leaf.node.can_fit(value));
    paged_leaf.node.put(key, value);

    save_paged_node(pager, &paged_leaf)?;

    if actualize_high_key {
        propagate_high_key_change(pager, paged_leaf.page_id, key, &tree_breadcrumbs)?;
    }

    Ok(())
}

fn split_and_persist_nodes(
    pager: &mut Pager,
    to_split: &mut PagedNode<LeafNode>,
    tree_breadcrumbs: &mut TreeBreadcrumbs
) -> PagerResult<SplitResult<LeafNode>> {
    let new_node = to_split.node.split();
    let new_node_page_id: PageId = save_node(pager, &new_node, None)?;

    save_paged_node(pager, to_split)?;

    let parent_paged_node = tree_breadcrumbs.last_parent();

    let mut parent_node_owned = None;
    let parent_paged_node_cell;
    let mut parent_paged_node_ref;

    let parent_node: &mut InternalNode;
    let parent_page_id;

    if let Some(cell) = parent_paged_node {
        parent_paged_node_cell = cell;
        parent_paged_node_ref = parent_paged_node_cell.borrow_mut();

        parent_page_id = Some(parent_paged_node_ref.page_id);
        parent_node = &mut parent_paged_node_ref.node;
    } else {
        parent_node_owned = Some(InternalNode::new(PAGE_SIZE_BYTES));
        parent_node = parent_node_owned.as_mut().unwrap();
        parent_page_id = None;
    }

    // we replace existing and insert the new one
    let paged_new = PagedNode { node: new_node, page_id: new_node_page_id };
    let dividers = compute_dividers(parent_node, to_split, &paged_new);

    for replace in dividers.replace {
        parent_node.replace_divider(
            replace.paged_node.node.first_key(),
            replace.divider,
            to_split.page_id
        );
    }

    for insert in dividers.insert {
        parent_node.put(insert.divider, insert.paged_node.page_id);
    }

    let saved_parent_page_id = save_node(pager, parent_node, parent_page_id)?;
    if parent_page_id.is_none() {
        // The only case when there's no parent – it's root page
        // So we make another page marked as root
        pager.new_root(saved_parent_page_id)?;

        tree_breadcrumbs.push(PagedNode {
            page_id: saved_parent_page_id,
            node: BTreeNode::Internal(parent_node_owned.unwrap()),
        });
    }

    Ok(SplitResult {
        new_child: paged_new,
    })
}

fn find_leaf_for(pager: &Pager, key: DatabaseKey) -> TreeBreadcrumbs {
    let mut result: TreeBreadcrumbs = TreeBreadcrumbs::new();
    result.push(load_root_node(pager));

    loop {
        if result.contains_leaf() {
            return result;
        }

        let paged_parent_cell = result.last_parent().unwrap();
        let paged_parent = paged_parent_cell.borrow();
        let child_page_id = paged_parent.node.get_child_page_id_by_key(key);

        result.push(PagedNode {
            page_id: child_page_id,
            node: load_node(pager, child_page_id).unwrap(),
        });
    }
}