//! The bottom sorting algorithm (we could just have 1-element runs and do all
//! the sorting with the merge algorithm, but that would be much slower).

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::ptr;

/// Sorts the list using insertion sort.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
// This version was almost completely copied from libcollections/slice.rs
pub fn sort<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) {
    unsafe {
        let list_ptr = list.as_mut_ptr();
        let len = list.len();
        for i in 0..len {
            let mut j = i;
            let list_i = list_ptr.offset(i as isize);
            while j > 0 && c(&*list_i, &*list_ptr.offset(j as isize - 1)) == Ordering::Less {
                j -= 1;
            }
            if i != j {
                let list_j = list_ptr.offset(j as isize);
                let tmp = ptr::read(list_i);
                ptr::copy(list_j, list_j.offset(1), i - j);
                ptr::write(list_j, tmp);
            }
        }
    }
}
