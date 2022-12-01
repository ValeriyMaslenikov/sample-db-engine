use crate::{
    aliases::{ DatabaseKey },
    pager::Pager,
    btree::node::common::{ BTreeNode, BTreeIterator },
};

use super::{
    structures::paged_node::{ PagedNode },
    persistance::loader::{ load_node, load_root_node },
};

#[allow(dead_code)]
pub(super) struct DividerAndPagedNode {
    pub(super) divider: DatabaseKey,
    pub(super) paged: PagedNode<BTreeNode>,
}
pub(super) struct IteratorItem {
    pub(super) parent: PagedNode<BTreeNode>,
    pub(super) children: Vec<DividerAndPagedNode>,
}

fn iterator_item(pager: &Pager, paged: PagedNode<BTreeNode>) -> IteratorItem {
    if paged.node.is_leaf() {
        return IteratorItem {
            parent: paged,
            children: vec![],
        };
    }
    assert!(paged.node.is_internal());

    let internal = paged.to_internal();

    let children = internal.node
        .iterator()
        .map(|item| {
            let page_id = item.key_reference.page_id;
            DividerAndPagedNode {
                divider: item.key_reference.key,
                paged: PagedNode { page_id, node: load_node(pager, page_id).unwrap() },
            }
        })
        .collect();

    IteratorItem {
        parent: paged,
        children,
    }
}
pub(super) struct BTreePreorderIterator<'a> {
    stack: Vec<PagedNode<BTreeNode>>,
    pager: &'a Pager,
}

impl<'a> BTreePreorderIterator<'a> {
    pub(super) fn new(pager: &'a Pager) -> Self {
        BTreePreorderIterator {
            stack: vec![load_root_node(pager)],
            pager,
        }
    }
}
impl<'a> Iterator for BTreePreorderIterator<'a> {
    type Item = IteratorItem;

    fn next(&mut self) -> Option<IteratorItem> {
        if let Some(paged_node) = self.stack.pop() {
            let iterator_item = iterator_item(self.pager, paged_node);

            for child in &iterator_item.children {
                self.stack.push(child.paged.clone());
            }

            return Some(iterator_item);
        }

        None
    }
}