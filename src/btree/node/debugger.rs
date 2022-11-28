/// This is the draft of the code used for debugging purposes.
/// You can display the tree, or the node separately in the human-readable format to verify the 
/// expected results before/after/in the middle of some operation
/// 
use crate::{ aliases::PageId, pager::Pager };

use super::{ internal::InternalNode, leaf::LeafNode, common::BTreeNode };

#[allow(dead_code)]
pub(in crate::btree) fn print_internal_node(node: &InternalNode, page_id: Option<PageId>) {
    return print_node(&BTreeNode::Internal(node.clone()), page_id);
}

#[allow(dead_code)]
pub(in crate::btree) fn print_leaf_node(node: &LeafNode, page_id: Option<PageId>) {
    return print_node(&BTreeNode::Leaf(node.clone()), page_id);
}

#[cfg(not(debug_assertions))]
pub(in crate::btree) fn print_node(node: &BTreeNode, page_id: Option<PageId>) {
    // This should fail the tests, this is used only for debugging
    unreachable!();
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub(in crate::btree) fn print_node(node: &BTreeNode, page_id: Option<PageId>) {
    use crate::btree::node::{
        key_reference_leaf::KeyReferenceLeaf,
        key_reference_internal::KeyReferenceInternal,
        key_reference::KeyReferenceGeneral, common::BTreeNodeEncodable,
    };

    let len = node.common().len();

    println!("Node on page {:?} (len: {})", page_id, len);
    match node {
        BTreeNode::Internal(x) => {
            print!("Internal:");
            for n in 0..len {
                let key_ref = KeyReferenceInternal::new_by_index(&x.data_space(), n);
                println!("\t{:?}", key_ref);
            }
        }
        BTreeNode::Leaf(x) => {
            print!("Leaf:");
            for n in 0..len {
                let key_ref = KeyReferenceLeaf::new_by_index(&x.data_space(), n);
                println!("{:?}", key_ref);
            }
        }
    }
}

#[cfg(not(debug_assertions))]
fn print_tree_internal(pager: &Pager, page_id: PageId, indent: usize) {
    // This should fail the tests, this is used only for debugging
    unreachable!();
}

#[cfg(debug_assertions)]
fn print_tree_internal(pager: &Pager, page_id: PageId, indent: usize) {
    use crate::btree::{
        persistance::loader::load_node, node::common::{BTreeNodeEncodable, BTreeIterator},
    };

    let node = load_node(pager, page_id).unwrap();
    let indent_tabs = String::from_utf8(vec![b'\t'; indent]).unwrap();

    if node.is_internal() {
        let internal_node = node.internal();

        println!();
        println!(
            "{}INTERNAL NODE. PAGE_ID: {}, {:?}",
            indent_tabs,
            page_id,
            internal_node.header()
        );
        println!("{}------------------------", indent_tabs);

        for child in internal_node.iterator() {
            println!();
            println!("{}\tDIVIDER: {}", indent_tabs, child.key_reference.key);
            print_tree_internal(pager, child.key_reference.page_id, indent + 1);
        }
    } else {
        let leaf_node = node.leaf();
        println!("{}LEAF NODE. PAGE_ID: {}, {:?}", indent_tabs, page_id, leaf_node.header());
        println!("{}------------------------", indent_tabs);
        for value in leaf_node.iterator() {
            println!(
                "{} Key: {}, Value: {}",
                indent_tabs,
                value.key_reference.key,
                value.value.len()
            );
        }
    }
}

pub(in crate::btree) fn print_tree(pager: &Pager, start_from_page_id: Option<PageId>) {
    let page_id = if start_from_page_id.is_none() {
        pager.root_page_id()
    } else {
        start_from_page_id.unwrap()
    };
    println!("\nPRINTING THE TREE FROM PAGE_ID: {}", page_id);

    print_tree_internal(pager, page_id, 0);
}