use std::cmp;

/// This is the implementation of the binary search on the data which may be loaded by
/// index. To get the element by index and key by element we use callbacks.
type InsertionPoint = u32;

pub fn binary_search_over_fn<E>(
    target_key: u32,
    count: u32,
    element_by_index: &dyn Fn(u32) -> E,
    key_by_element: &dyn Fn(&E) -> u32
) -> (InsertionPoint, Option<E>) {
    if count == 0 {
        return (0, None);
    }

    let mut search_size = count;
    let mut left = 0_u32;
    let mut right = count;

    while left < right {
        let mid = left + search_size / 2;
        let current_key_ref = element_by_index(mid);
        let current_key = key_by_element(&current_key_ref);
        match target_key.cmp(&current_key) {
            cmp::Ordering::Equal => {
                return (mid, Some(current_key_ref));
            }
            cmp::Ordering::Greater => {
                left = mid + 1;
            }
            _ => {
                right = mid;
            }
        }
        // if target_key == current_key {
        //     return (mid, Some(current_key_ref));
        // } else if target_key > current_key {
        //     left = mid + 1;
        // } else {
        //     right = mid;
        // }

        search_size = right - left;
    }

    (left, None)
}