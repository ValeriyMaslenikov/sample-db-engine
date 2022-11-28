use crate::{
    helpers::{ average::average },
    aliases::{ DatabaseKey },
    btree::{
        node::{
            internal::InternalNode,
            leaf::LeafNode,
            common::{ BTreeIterator, BTreeNodeCommon },
        },
        structures::paged_node::PagedNode,
    },
};

pub(in crate::btree::operations) struct Divider<'a> {
    pub(in crate::btree) divider: DatabaseKey,
    pub(in crate::btree) paged_node: &'a PagedNode<LeafNode>,
}

pub(in crate::btree::operations) struct Dividers<'a> {
    pub(crate) insert: Vec<Divider<'a>>,
    pub(crate) replace: Vec<Divider<'a>>,
}
pub(in crate::btree::operations) fn compute_dividers<'a>(
    parent_node: &InternalNode,
    existing: &'a PagedNode<LeafNode>,
    new: &'a PagedNode<LeafNode>
) -> Dividers<'a> {
    let first_key_of_new_node = new.node.first_key();
    let last_key_of_new_node = new.node.last_key();
    let last_key_of_exist_node = existing.node.last_key();

    let divider_btw_nodes = average(first_key_of_new_node, last_key_of_exist_node);

    // If empty, insert first and last element of new_node
    if parent_node.is_empty() {
        return Dividers {
            insert: vec![
                Divider { divider: divider_btw_nodes, paged_node: existing },
                Divider { divider: last_key_of_new_node, paged_node: new }
            ],
            replace: vec![],
        };
    }

    // If not empty and existing node is already part of parent - replace existing and insert last
    let last_child = parent_node.last();
    let existing_node_is_last_in_parent =
        last_child.is_some() && last_child.unwrap().key_reference.page_id == existing.page_id;

    if existing_node_is_last_in_parent {
        return Dividers {
            replace: vec![Divider { divider: divider_btw_nodes, paged_node: existing }],
            insert: vec![Divider { divider: last_key_of_new_node, paged_node: new }],
        };
    } else {
        let existing_divider = parent_node.key_reference_for(&existing.node);

        return Dividers {
            replace: vec![Divider { divider: divider_btw_nodes, paged_node: existing }],
            insert: vec![Divider { divider: existing_divider.key, paged_node: new }],
        };
    }
}

#[cfg(test)]
mod compute_dividers_test {
    use std::ptr;

    use crate::{ btree::node::leaf::LeafNode };

    use super::*;

    #[test]
    fn test_new_parent_second_node_one_element() {
        let parent_node = InternalNode::new(1024);

        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut second_child = LeafNode::new(1024);
        second_child.put(3, "Third".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &second_child_paged_node
        );

        assert!(result.insert[0].divider == 2);
        assert!(result.insert[1].divider == 3);
    }

    #[test]
    fn test_new_parent_second_node_several_elements() {
        let parent_node = InternalNode::new(1024);

        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut second_child = LeafNode::new(1024);
        second_child.put(3, "Third".as_bytes());
        second_child.put(4, "Four".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &second_child_paged_node
        );

        assert_eq!(result.insert.len(), 2);
        assert_eq!(result.insert[0].divider, 2);
        assert_eq!(result.insert[1].divider, 4);
        assert!(ptr::eq(result.insert[0].paged_node, &first_child_paged_node));
        assert!(ptr::eq(result.insert[1].paged_node, &second_child_paged_node));
    }

    #[test]
    fn test_new_parent_second_node_odd_mid_key() {
        let parent_node = InternalNode::new(1024);

        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut second_child = LeafNode::new(1024);
        second_child.put(9, "Third".as_bytes());
        second_child.put(10, "Four".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &second_child_paged_node
        );

        assert_eq!(result.insert.len(), 2);
        assert_eq!(result.insert[0].divider, 5);
        assert_eq!(result.insert[1].divider, 10);
        assert!(ptr::eq(result.insert[0].paged_node, &first_child_paged_node));
        assert!(ptr::eq(result.insert[1].paged_node, &second_child_paged_node));
    }

    #[test]
    fn test_new_parent_second_node_even_mid_key() {
        let parent_node = InternalNode::new(1024);

        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut second_child = LeafNode::new(1024);
        second_child.put(12, "Third".as_bytes());
        second_child.put(20, "Four".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &second_child_paged_node
        );

        assert_eq!(result.insert.len(), 2);
        assert_eq!(result.insert[0].divider, 7);
        assert_eq!(result.insert[1].divider, 20);
        assert!(ptr::eq(result.insert[0].paged_node, &first_child_paged_node));
        assert!(ptr::eq(result.insert[1].paged_node, &second_child_paged_node));
    }

    #[test]
    fn test_parent_one_child() {
        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut parent_node = InternalNode::new(1024);
        // Before it stored both of these nodes and after split
        parent_node.put(4, 0);

        let mut second_child = LeafNode::new(1024);
        second_child.put(3, "Third".as_bytes());
        second_child.put(4, "Four".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &second_child_paged_node
        );

        assert_eq!(result.replace.len(), 1);
        assert_eq!(result.replace[0].divider, 2);
        assert!(ptr::eq(result.replace[0].paged_node, &first_child_paged_node));

        assert_eq!(result.insert.len(), 1);
        assert_eq!(result.insert[0].divider, 4);
        assert!(ptr::eq(result.insert[0].paged_node, &second_child_paged_node));
    }

    #[test]
    fn test_parent_two_children_exist_is_first() {
        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());
        let first_child_paged_node = PagedNode { node: first_child, page_id: 0 };

        let mut second_child = LeafNode::new(1024);
        second_child.put(10, "Ten".as_bytes());
        second_child.put(12, "Twelve".as_bytes());

        let mut parent_node = InternalNode::new(1024);
        // Before it stored two nodes and first one had three elements: 1,2,3
        parent_node.put(3, 0);
        parent_node.put(12, 2);

        // After the division, this child appears to have only element: 3
        let mut divided_child = LeafNode::new(1024);
        divided_child.put(3, "Three".as_bytes());
        let divided_child_paged_node = PagedNode { node: divided_child, page_id: 1 };

        let result = compute_dividers(
            &parent_node,
            &first_child_paged_node,
            &divided_child_paged_node
        );

        assert_eq!(result.replace.len(), 1);
        assert_eq!(result.replace[0].divider, 2);
        assert!(ptr::eq(result.replace[0].paged_node, &first_child_paged_node));

        assert_eq!(result.insert.len(), 1);
        assert_eq!(result.insert[0].divider, 3);
        assert!(ptr::eq(result.insert[0].paged_node, &divided_child_paged_node));
    }

    #[test]
    fn test_parent_two_children_exist_is_last() {
        let mut first_child = LeafNode::new(1024);
        first_child.put(1, "First".as_bytes());
        first_child.put(2, "Second".as_bytes());

        let mut second_child = LeafNode::new(1024);
        second_child.put(10, "Ten".as_bytes());
        second_child.put(12, "Twelve".as_bytes());
        second_child.put(15, "Fifteen".as_bytes());
        let second_child_paged_node = PagedNode { node: second_child, page_id: 1 };

        let mut parent_node = InternalNode::new(1024);
        parent_node.put(6, 0);
        // Before it stored two nodes and first one had three elements: 10,12,15,16,17,18
        parent_node.put(18, 1);

        // After the division, this child appears to have three elements:
        let mut divided_child = LeafNode::new(1024);
        divided_child.put(16, "Sixteen".as_bytes());
        divided_child.put(17, "Seventeen".as_bytes());
        divided_child.put(18, "Eighteen".as_bytes());
        let divided_child_paged_node = PagedNode { node: divided_child, page_id: 2 };

        let result = compute_dividers(
            &parent_node,
            &second_child_paged_node,
            &divided_child_paged_node
        );

        assert_eq!(result.replace.len(), 1);
        assert_eq!(result.replace[0].divider, 15);
        assert!(ptr::eq(result.replace[0].paged_node, &second_child_paged_node));

        assert_eq!(result.insert.len(), 1);
        assert_eq!(result.insert[0].divider, 18);
        assert!(ptr::eq(result.insert[0].paged_node, &divided_child_paged_node));
    }
}