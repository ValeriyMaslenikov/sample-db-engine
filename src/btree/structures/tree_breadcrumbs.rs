use std::{ cell::{ RefCell }, rc::Rc };
use crate::btree::node::{ leaf::LeafNode, internal::InternalNode, common::BTreeNode };
use super::paged_node::{ PagedNode };

type LeafBreadcrumb = Rc<RefCell<PagedNode<LeafNode>>>;

type InternalBreadcrumb = Rc<RefCell<PagedNode<InternalNode>>>;

/// It represents the breadcrumbs used to find the value in the tree:
/// Internal Node A ->
///   Internal Node B ->
///      Internal Node C
///         Leaf Node D
///
/// Parents are indexed from the end, in the following example the vector will store (A, B, C),
/// But when asking for the first parent – it's C, not A.
pub(in crate::btree) struct TreeBreadcrumbs {
    leaf: Option<LeafBreadcrumb>,
    parents: Vec<InternalBreadcrumb>,
}

impl TreeBreadcrumbs {
    pub(in crate::btree) fn contains_leaf(&self) -> bool {
        return self.leaf.is_some();
    }

    pub(in crate::btree) fn leaf(&self) -> Option<LeafBreadcrumb> {
        assert!(self.contains_leaf());

        return self.leaf.clone();
    }

    pub(in crate::btree) fn get_parent(&self, inx: usize) -> Option<InternalBreadcrumb> {
        // inx = 0, len = 10, reversed_index = 9
        // inx = 1, len = 10, reversed_index = 8
        // inx = 9, len = 10, reversed_index = 0
        let len = self.parents.len();
        let reversed_index = len.checked_sub(inx + 1);

        reversed_index.map(|i| self.parents[i].clone())
    }

    pub(in crate::btree) fn push(&mut self, paged_node: PagedNode<BTreeNode>) {
        if paged_node.node.is_leaf() {
            self.leaf = Some(Rc::new(RefCell::new(paged_node.to_leaf())));
        } else {
            self.parents.push(Rc::new(RefCell::new(paged_node.to_internal())));
        }
    }
    pub(in crate::btree) fn new() -> TreeBreadcrumbs {
        return TreeBreadcrumbs {
            leaf: None,
            parents: vec![],
        };
    }

    pub(in crate::btree) fn last_parent(&self) -> Option<InternalBreadcrumb> {
        self.get_parent(0)
    }

    pub(crate) fn has_parents(&self) -> bool {
        self.last_parent().is_some()
    }
}