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

//! The bottom sorting algorithm (we could just have 1-element runs and do all
//! the sorting with the merge algorithm, but that would be much slower).

use std::cmp::Ordering;

/// Test the insertion sort implementation with an empty list
#[test]
fn test_empty() {
    let mut list: Vec<u32> = vec![];
    sort(&mut list);
    assert!(list.len() == 0);
}

/// Test the insertion sort implementation with a single-element list
#[test]
fn test_single() {
    let mut list = vec![42];
    sort(&mut list);
    assert!(list[0] == 42);
}

/// Test the insertion sort implementation with a short unsorted list
#[test]
fn test_unsorted() {
    let mut list = vec![3, 1, 0, 4];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
}

/// Test the insertion sort implementation with a short backward list
#[test]
fn test_reverse() {
    let mut list = vec![21, 18, 7, 1];
    sort(&mut list);
    println!("{:?}", list);
    assert!(list[0] == 1);
    assert!(list[1] == 7);
    assert!(list[2] == 18);
    assert!(list[3] == 21);
}

/// Test the insertion sort implementation with a short unsorted list
#[test]
fn test_sorted() {
    let mut list = vec![0, 1, 2, 3];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 2);
    assert!(list[3] == 3);
}

/// Insertion sort implementation convenience used for tests.
pub fn sort<T: Ord>(list: &mut[T]) {
    sort_by(list, |a, b| a.cmp(b) );
}

/// Sorts the list using insertion sort.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
pub fn sort_by<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) {
    let len = list.len();
    let mut pos = 1;
    while pos < len {
        let mut pos_sorted = pos;
        while pos_sorted > 0 && c(&list[pos_sorted - 1], &list[pos_sorted]) == Ordering::Greater {
            list.swap(pos_sorted - 1, pos_sorted);
            pos_sorted -= 1;
        }
        pos += 1;
    }
}
