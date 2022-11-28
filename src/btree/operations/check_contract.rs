/// This functionality is used during the debugging and during the 
/// integration testing. We want to make sure that the contract
/// on which the BTree is built is preserved 
/// I.e that leaf node contains only the data which belongs to the 
/// internal's node range.

#[cfg(feature = "integration")]
use crate::pager::Pager;

#[cfg(feature = "integration")]
pub(crate) fn do_check_tree_contract(pager: &Pager) {
    use crate::{ btree::{ iterator::BTreePreorderIterator, node::{ debugger::print_tree, common::BTreeIterator } } };

    let btree = BTreePreorderIterator::new(pager);

    for item in btree {
        if item.parent.node.is_leaf() {
            // We checked everything at the level of internals
            continue;
        }

        for child in &item.children {
            let node = &child.paged.node;
            let divider = child.divider;

            if node.is_leaf() {
                let leaf = node.leaf();
                for item in leaf.iterator() {
                    assert!(item.key_reference.key <= divider);
                }
            } else {
                let leaf = node.internal();
                for item in leaf.iterator() {
                    assert!(item.key_reference.key <= divider);
                }
            }
        }
    }

    print_tree(pager, None);
}