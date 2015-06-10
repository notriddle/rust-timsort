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
use std::mem::{swap, uninitialized, replace};
use std::fmt::Debug;

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

/// Merge convenience used for tests.
#[allow(unused)]
pub fn merge<T: Ord + Debug>(list: &mut[T], first_len: usize) {
    merge_by(list, first_len, |a, b| a.cmp(b) );
}

/// Merge implementation switch.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
#[allow(unused)]
pub fn merge_by<T: Debug, C: Fn(&T, &T) -> Ordering>(list: &mut[T], first_len: usize, c: C) {
    let second_len = list.len() - first_len;
    //if first_len >= second_len {
    //    merge_hi(list, first_len, second_len, c);
    //} else {
        merge_lo(list, first_len, second_len, c);
    //}
}

/// Merge implementation used when the first run is smaller than the second.
#[allow(unused)]
pub fn merge_lo<T: Debug, C: Fn(&T, &T) -> Ordering>(list: &mut[T], first_len: usize, second_len: usize, c: C) {
    unsafe {
        // First, move the smallest run into temporary storage, leaving the
        // original contents uninitialized.
        let mut tmp: Vec<T> = Vec::with_capacity(first_len);
        for i in 0..first_len {
            tmp.push(replace(&mut list[i], uninitialized()));
        }
        // Do the actual merge.
        let mut first_pos  = 0;
        let mut second_pos = first_len;
        let mut dest_pos   = 0;
        while second_pos > dest_pos {
            println!("first_pos: {:?}, second_pos: {:?}, dest_pos: {:?}, list: {:?}, tmp: {:?}", first_pos, second_pos, dest_pos, list, tmp);
            if second_pos < list.len() && c(&tmp[first_pos], &list[second_pos]) == Ordering::Greater {
                list.swap(second_pos, dest_pos);
                second_pos += 1;
            } else {
                swap(&mut tmp[first_pos], &mut list[dest_pos]);
                first_pos += 1;
            }
            dest_pos += 1;
        }
        // The temporary storage is now full of nothing but uninitialized.
        // We want to deallocate the space, but not call the destructors.
        tmp.set_len(0);
    }
}

