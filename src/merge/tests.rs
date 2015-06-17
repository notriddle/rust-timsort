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

use merge;

/// Test mergeing two empty slices.
#[test]
fn empty() {
    let mut list: Vec<u32> = vec![];
    merge(&mut list, 0);
    assert!(list.len() == 0);
}

/// Test merging two equal-sized single-element vectors that are already sorted.
#[test]
fn single_sorted() {
    let mut list = vec![42, 90];
    merge(&mut list, 1);
    assert!(list[0] == 42);
    assert!(list[1] == 90);
}

/// Test merging two equal-sized single-element vectors that are already sorted.
#[test]
fn single_unsorted() {
    let mut list = vec![90, 42];
    merge(&mut list, 1);
    assert!(list[0] == 42);
    assert!(list[1] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn hi_unsorted() {
    let mut list = vec![90, 17, 42];
    merge(&mut list, 1);
    assert!(list[0] == 17);
    assert!(list[1] == 42);
    assert!(list[2] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn lo_unsorted() {
    let mut list = vec![17, 90, 42];
    merge(&mut list, 2);
    assert!(list[0] == 17);
    assert!(list[1] == 42);
    assert!(list[2] == 90);
}

/// Test merging two unequal-sized vectors.
#[test]
fn hi_unsorted_multiple() {
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
fn lo_unsorted_multiple() {
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
fn lo_panic() {
    use std::thread;
    let mut list = vec![1, 2, 3, 4, 5];
    unsafe {
        let list2p: *mut Vec<usize> = &mut list;
        let list2: &mut Vec<usize> = &mut *list2p;
        let _ = thread::spawn(move || {
            merge::merge(list2, 3, |_, _| { panic!("Expected panic: this is normal") });
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
fn hi_panic() {
    use std::thread;
    let mut list = vec![1, 2, 3, 4, 5];
    unsafe {
        let list2p: *mut Vec<usize> = &mut list;
        let list2: &mut Vec<usize> = &mut *list2p;
        let _ = thread::spawn(move || {
            merge::merge(list2, 2, |_, _| { panic!("Expected panic: this is normal") });
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
fn lo_nodrop() {
    #[derive(Debug)]
    struct ExplodeOnDrop(usize);
    impl Drop for ExplodeOnDrop {
        fn drop(&mut self) {
           // panic!("We're not supposed to panic.");
        }
    }
    let mut list = vec![ExplodeOnDrop(3), ExplodeOnDrop(7), ExplodeOnDrop(2)];
    merge::merge(&mut list, 2, |a, b| {a.0.cmp(&b.0) });
    assert!(list[0].0 == 2);
    assert!(list[1].0 == 3);
    assert!(list[2].0 == 7);
    unsafe { list.set_len(0); }
}

#[test]
fn hi_nodrop() {
    #[derive(Debug)]
    struct ExplodeOnDrop(usize);
    impl Drop for ExplodeOnDrop {
        fn drop(&mut self) {
            panic!("We're not supposed to panic.");
        }
    }
    let mut list = vec![ExplodeOnDrop(3), ExplodeOnDrop(2), ExplodeOnDrop(7)];
    merge::merge(&mut list, 1, |a, b| {a.0.cmp(&b.0) });
    assert!(list[0].0 == 2);
    assert!(list[1].0 == 3);
    assert!(list[2].0 == 7);
    unsafe { list.set_len(0); }
}

/// Ensure that, when we enter galloping mode, we still work right.

#[test]
fn lo_gallop_stress() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    merge(&mut list, 21);
    println!("{:?}", list);
    assert!(list[0] == 1);
    assert!(list[1] == 2);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
    assert!(list[4] == 5);
    assert!(list[5] == 6);
    assert!(list[6] == 7);
    assert!(list[7] == 8);
    assert!(list[8] == 9);
    assert!(list[9] == 10);
    assert!(list[10] == 11);
    assert!(list[11] == 12);
    assert!(list[12] == 13);
    assert!(list[13] == 14);
    assert!(list[14] == 15);
    assert!(list[15] == 16);
    assert!(list[16] == 17);
    assert!(list[17] == 18);
    assert!(list[18] == 19);
    assert!(list[19] == 20);
    assert!(list[20] == 20);
    assert!(list[21] == 21);
    assert!(list[22] == 22);
    assert!(list[23] == 23);
    assert!(list[24] == 24);
    assert!(list[25] == 25);
    assert!(list[26] == 26);
    assert!(list[27] == 27);
    assert!(list[28] == 28);
    assert!(list[29] == 29);
    assert!(list[30] == 30);
}

/// Ensure that, when we enter galloping mode, we still work right.

#[test]
fn hi_gallop_stress() {
    let mut list = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
    merge(&mut list, 10);
    println!("{:?}", list);
    assert!(list[0] == 1);
    assert!(list[1] == 2);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
    assert!(list[4] == 5);
    assert!(list[5] == 6);
    assert!(list[6] == 7);
    assert!(list[7] == 8);
    assert!(list[8] == 9);
    assert!(list[9] == 10);
    assert!(list[10] == 11);
    assert!(list[11] == 12);
    assert!(list[12] == 13);
    assert!(list[13] == 14);
    assert!(list[14] == 15);
    assert!(list[15] == 16);
    assert!(list[16] == 17);
    assert!(list[17] == 18);
    assert!(list[18] == 19);
    assert!(list[19] == 20);
    assert!(list[20] == 20);
    assert!(list[21] == 21);
    assert!(list[22] == 22);
    assert!(list[23] == 23);
    assert!(list[24] == 24);
    assert!(list[25] == 25);
    assert!(list[26] == 26);
    assert!(list[27] == 27);
    assert!(list[28] == 28);
    assert!(list[29] == 29);
    assert!(list[30] == 30);
}

/// Merge convenience used for tests.
pub fn merge<T: Ord>(list: &mut [T], first_len: usize) {
    merge::merge(list, first_len, |a, b| a.cmp(b) );
}

