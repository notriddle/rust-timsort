//! This crate is a stable sorting algorithm with O(n) worst-case storage
//! requirements, O(n log n) worst-case comparisons, and O(n) comparisons
//! on an already-sorted list, smoothly becoming O(n log n) as the sorted
//! sections (runs) get smaller and smaller.

mod insort;
mod merge;
mod gallop;
mod find_run;
mod sort;

pub use sort::sort as sort_by;

use std::cmp::Ordering;
pub fn sort<T: PartialOrd>(list: &mut [T]) {
    sort_by(list, |a, b| {
        a.partial_cmp(b).unwrap_or(Ordering::Equal)
    })
}
