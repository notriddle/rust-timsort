// This file is a part of Timsort-Rust.
// 
// Copyright (C) 2015 Michael Howell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The merge algorithm. This one can merge unequal slices, allocating an n/2
//! sized temporary slice of the same type. Naturally, it can only merge slices
//! that are themselves already sorted.

use std::cmp::Ordering;
use std::ptr;
use gallop;

/// Test mergeing two empty slices.
#[test]
fn test_empty() {
    let mut list: Vec<u32> = vec![];
    merge(&mut list, 0);
    assert!(list.len() == 0);
}

/// Test merging two equal-sized single-element vectors that are already sorted.
#[test]
fn test_single_sorted() {
    let mut list = vec![42, 90];
    merge(&mut list, 1);
    assert!(list[0] == 42);
    assert!(list[1] == 90);
}

/// Test merging two equal-sized single-element vectors that are already sorted.
#[test]
fn test_single_unsorted() {
    let mut list = vec![90, 42];
    merge(&mut list, 1);
    println!("{:?}", list);
    assert!(list[0] == 42);
    assert!(list[1] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn test_hi_unsorted() {
    let mut list = vec![90, 17, 42];
    merge(&mut list, 1);
    assert!(list[0] == 17);
    assert!(list[1] == 42);
    assert!(list[2] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn test_lo_unsorted() {
    let mut list = vec![17, 90, 42];
    merge(&mut list, 2);
    assert!(list[0] == 17);
    assert!(list[1] == 42);
    assert!(list[2] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn test_hi_unsorted_multiple() {
    let mut list = vec![21, 32, 91, 17, 20, 40, 80];
    merge(&mut list, 3);
    assert!(list[0] == 17);
    assert!(list[1] == 20);
    assert!(list[2] == 21);
    assert!(list[3] == 32);
    assert!(list[4] == 40);
    assert!(list[5] == 80);
    assert!(list[6] == 91);
}

/// Test merging two unequal-sized vectors.
#[test]
fn test_lo_unsorted_multiple() {
    let mut list = vec![17, 20, 40, 80, 21, 32, 91];
    merge(&mut list, 4);
    assert!(list[0] == 17);
    assert!(list[1] == 20);
    assert!(list[2] == 21);
    assert!(list[3] == 32);
    assert!(list[4] == 40);
    assert!(list[5] == 80);
    assert!(list[6] == 91);
}

/// Test panic safety when the first run is longest
#[test]
fn test_lo_panic() {
    use std::thread;
    let mut list = vec![1, 2, 3, 4, 5];
    unsafe {
        let list2p: *mut Vec<usize> = &mut list;
        let list2: &mut Vec<usize> = &mut *list2p;
        let _ = thread::spawn(move || {
            merge_by(list2, 3, |_, _| { panic!("Expected panic: this is normal") });
        }).join().err().unwrap();
    }
    assert!(list[0] == 1);
    assert!(list[1] == 2);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
    assert!(list[4] == 5);
}

/// Test panic safety when the second run is longest
#[test]
fn test_hi_panic() {
    use std::thread;
    let mut list = vec![1, 2, 3, 4, 5];
    unsafe {
        let list2p: *mut Vec<usize> = &mut list;
        let list2: &mut Vec<usize> = &mut *list2p;
        let _ = thread::spawn(move || {
            merge_by(list2, 2, |_, _| { panic!("Expected panic: this is normal") });
        }).join().err().unwrap();
    }
    assert!(list[0] == 1);
    assert!(list[1] == 2);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
    assert!(list[4] == 5);
}

/// Test that the drop() is never run while sorting.

#[test]
fn test_lo_nodrop() {
    struct ExplodeOnDrop(usize);
    impl Drop for ExplodeOnDrop {
        fn drop(&mut self) {
            panic!("We're not supposed to panic.");
        }
    }
    let mut list = vec![ExplodeOnDrop(3), ExplodeOnDrop(7), ExplodeOnDrop(2)];
    merge_by(&mut list, 2, |a, b| {a.0.cmp(&b.0) });
    assert!(list[0].0 == 2);
    assert!(list[1].0 == 3);
    assert!(list[2].0 == 7);
    unsafe { list.set_len(0); }
}

#[test]
fn test_hi_nodrop() {
    struct ExplodeOnDrop(usize);
    impl Drop for ExplodeOnDrop {
        fn drop(&mut self) {
            panic!("We're not supposed to panic.");
        }
    }
    let mut list = vec![ExplodeOnDrop(3), ExplodeOnDrop(2), ExplodeOnDrop(7)];
    merge_by(&mut list, 1, |a, b| {a.0.cmp(&b.0) });
    assert!(list[0].0 == 2);
    assert!(list[1].0 == 3);
    assert!(list[2].0 == 7);
    unsafe { list.set_len(0); }
}

/// Merge convenience used for tests.
pub fn merge<T: Ord>(list: &mut [T], first_len: usize) {
    merge_by(list, first_len, |a, b| a.cmp(b) );
}

/// Merge implementation switch.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
pub fn merge_by<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], first_len: usize, c: C) {
    let second_len = list.len() - first_len;
    if first_len >= second_len {
        merge_hi(list, first_len, second_len, c);
    } else {
        merge_lo(list, first_len, c);
    }
}

/// The number of times any one run can win before we try galloping.
const MIN_GALLOP: usize = 0;

/// Merge implementation used when the first run is smaller than the second.
pub fn merge_lo<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], first_len: usize, c: C) {
    unsafe {
        let mut state = MergeLo::new(list, first_len, c);
        state.merge();
    }
}

/// Implementation of `merge_lo`. We need to have an object in order to
/// implement panic safety.
struct MergeLo<'a, T: 'a, C: Fn(&T, &T) -> Ordering> {
    list_len: usize,
    first_pos: usize,
    first_len: usize,
    second_pos: usize,
    dest_pos: usize,
    list: &'a mut [T],
    tmp: Vec<T>,
    c: C,
}
impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> MergeLo<'a, T, C> {
    /// Constructor for a lower merge.
    unsafe fn new(list: &'a mut [T], first_len: usize, c: C) -> Self {
        let mut ret_val = MergeLo{
            list_len:   list.len(),
            first_pos:  0,
            first_len:  first_len,
            second_pos: first_len,
            dest_pos:   0,
            list:       list,
            tmp:        Vec::with_capacity(first_len),
            c:          c,
        };
        // First, move the smallest run into temporary storage, leaving the
        // original contents uninitialized.
        ret_val.tmp.set_len(first_len);
        for i in 0..first_len {
            ptr::copy_nonoverlapping(&ret_val.list[i], &mut ret_val.tmp[i], 1);
        }
        return ret_val;
    }
    /// Perform the one-by-one comparison and insertion.
    unsafe fn merge(&mut self) {
        let c = &self.c;
        let mut first_count  = 0;
        let mut second_count = 0;
        while self.second_pos > self.dest_pos && self.second_pos < self.list_len {
            // Make sure gallop doesn't bring our positions out of sync.
            debug_assert!(self.first_pos + (self.second_pos - self.first_len) == self.dest_pos);
            if (second_count | first_count) >= MIN_GALLOP {
                if c(&self.tmp[self.first_pos], &self.list[self.second_pos]) == Ordering::Greater {
                    ptr::copy_nonoverlapping(&self.list[self.second_pos], &mut self.list[self.dest_pos], 1);
                    self.second_pos += 1;
                    second_count += 1;
                    first_count = 0;
                } else {
                    ptr::copy_nonoverlapping(&self.tmp[self.first_pos], &mut self.list[self.dest_pos], 1);
                    self.first_pos += 1;
                    first_count += 1;
                    second_count = 0;
                }
                self.dest_pos += 1;
            } else {
                second_count = gallop::gallop_left_by(&self.tmp[self.first_pos], self.list.split_at(self.second_pos).1, c);
                ptr::copy(&self.list[self.second_pos], &mut self.list[self.dest_pos], second_count);
                self.dest_pos   += second_count;
                self.second_pos += second_count;
                // Make sure gallop doesn't bring our positions out of sync.
                debug_assert!(self.first_pos + (self.second_pos - self.first_len) == self.dest_pos);
                if self.second_pos > self.dest_pos && self.second_pos < self.list_len {
                    first_count = gallop::gallop_right_by(&self.list[self.second_pos], self.tmp.split_at(self.first_pos).1, c);
                    ptr::copy(&self.tmp[self.first_pos], &mut self.list[self.dest_pos], first_count);
                    self.dest_pos  += first_count;
                    self.first_pos += first_count;
                }
            }
        }
    }
}
impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> Drop for MergeLo<'a, T, C> {
    /// Copy all remaining items in the temporary storage into the list.
    /// If the comparator panics, the result will not be sorted, but will still
    /// contain no duplicates or uninitialized spots.
    fn drop(&mut self) {
        unsafe {
            // Make sure that the entire tmp storage is consumed. Since there are no uninitialized
            // spaces before dest_pos, and no uninitialized space after first_pos, this will ensure
            // that there are no uninitialized spaces inside the slice after we drop. Thus, the
            // function is safe.
            if self.first_pos < self.first_len {
                ptr::copy_nonoverlapping(&self.tmp[self.first_pos], &mut self.list[self.dest_pos], self.first_len - self.first_pos);
            }
            // The temporary storage is now full of nothing but uninitialized.
            // We want to deallocate the space, but not call the destructors.
            self.tmp.set_len(0);
        }
    }
}

/// Merge implementation used when the first run is larger than the second.
pub fn merge_hi<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], first_len: usize, second_len: usize, c: C) {
    unsafe {
        let mut state = MergeHi::new(list, first_len, second_len, c);
        state.merge();
    }
}

/// Implementation of `merge_hi`. We need to have an object in order to
/// implement panic safety.
struct MergeHi<'a, T: 'a, C: Fn(&T, &T) -> Ordering> {
    first_pos: isize,
    second_pos: isize,
    dest_pos: isize,
    list: &'a mut [T],
    tmp: Vec<T>,
    c: C
}

impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> MergeHi<'a, T, C> {
    /// Constructor for a higher merge.
    unsafe fn new(list: &'a mut [T], first_len: usize, second_len: usize, c: C) -> Self {
        let mut ret_val = MergeHi{
            first_pos:  first_len as isize - 1,
            second_pos: second_len as isize - 1,
            dest_pos:   list.len() as isize - 1,
            list:       list,
            tmp:        Vec::with_capacity(second_len),
            c:          c
        };
        // First, move the smallest run into temporary storage, leaving the
        // original contents uninitialized.
        ret_val.tmp.set_len(second_len);
        for i in 0..second_len {
            ptr::copy_nonoverlapping(&ret_val.list[i + first_len], &mut ret_val.tmp[i], 1);
        }
        return ret_val;
    }
    /// Perform the one-by-one comparison and insertion.
    unsafe fn merge(&mut self) {
        let c = &self.c;
        while self.first_pos < self.dest_pos && self.first_pos >= 0 {
            // Make sure gallop doesn't bring our positions out of sync.
            debug_assert!(self.first_pos + self.second_pos + 1 == self.dest_pos);
            if c(&self.tmp[self.second_pos as usize], &self.list[self.first_pos as usize]) == Ordering::Greater {
                ptr::copy_nonoverlapping(&self.tmp[self.second_pos as usize], &mut self.list[self.dest_pos as usize], 1);
                self.second_pos -= 1;
            } else {
                ptr::copy_nonoverlapping(&self.list[self.first_pos as usize], &mut self.list[self.dest_pos as usize], 1);
                self.first_pos -= 1;
            }
            self.dest_pos -= 1;
        }
    }
}

/// Perform a backwards `ptr::copy_nonoverlapping`. Behave identically when size = 1, but behave
/// differently all other times
unsafe fn copy_nonoverlapping_backwards<T>(src: *const T, dest: *mut T, size: usize) {
    ptr::copy_nonoverlapping(src.offset(-(size as isize - 1)), dest.offset(-(size as isize - 1)), size)
}

impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> Drop for MergeHi<'a, T, C> {
    /// Copy all remaining items in the temporary storage into the list.
    /// If the comparator panics, the result will not be sorted, but will still
    /// contain no duplicates or uninitialized spots.
    fn drop(&mut self) {
        unsafe {
            // Make sure that the entire tmp storage is consumed. Since there are no uninitialized
            // spaces before dest_pos, and no uninitialized space after first_pos, this will ensure
            // that there are no uninitialized spaces inside the slice after we drop. Thus, the
            // function is safe.
            if self.second_pos >= 0 {
                copy_nonoverlapping_backwards(&self.tmp[self.second_pos as usize], &mut self.list[self.dest_pos as usize], self.second_pos as usize + 1);
            }

            // The temporary storage is now full of nothing but uninitialized.
            // We want to deallocate the space, but not call the destructors.
            self.tmp.set_len(0);
        }
    }
}

