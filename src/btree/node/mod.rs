//// Data layout of the node depends on the type of the node (leaf or non-leaf).
////
//// LEAF PAGE
////
//// Straight after the page header you will see the following structure,
//// which will represent the references to the "heap" of this page by exact keys of the elements.
////
////       ┌─ ┌──────────────────────────────────────────┐  │
////       │  │            1 element key (4b)            │  │
//// 12 bytes ├──────────────────────────────────────────┤  │
//// in total │       1 element address in page (4b)     │  │ From 0 byte
////       │  ├──────────────────────────────────────────┤  │ To data_space.len()
////       │  │          1 element length (4b)           │  │
////       └─ ├──────────────────────────────────────────┤  │
////          │            2 element key (4b)            │  │
////          ├──────────────────────────────────────────┤  │
////          │                   .....                  │  │
////          └──────────────────────────────────────────┘  ▼
////
////
//// And data itself, which grows from the data_space.len() to 0.
////
//// INTERNAL PAGE:
////
////
////        ┌─ ┌────────────────────────────────────────────────────┐  │
////  8 bytes  │                    1st divider                     │  │
////  in total ├────────────────────────────────────────────────────┤  │
////        │  │ Page ID for node b/w -∞ and 1st divider incl. (4b) │  │ From 0 byte
////        └─ ├────────────────────────────────────────────────────┤  │ To data_space.len()
////           │                    2nd divider                     │  │
////           ├────────────────────────────────────────────────────┤  │
////           │      Page ID for node b/w 1st key and 2nd key      │  │
////           ├────────────────────────────────────────────────────┤  │
////           │                        .....                       │  │
////           └────────────────────────────────────────────────────┘  ▼
////
pub(super) mod header;
pub(super) mod leaf;
pub(super) mod internal;
pub(super) mod internal_iterator;
pub(super) mod key_reference_internal;
pub(super) mod debugger;
pub(super) mod common;


mod key_reference;
mod key_reference_leaf;

mod leaf_iterator;

mod helpers;
