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

use insort;

/// Test the insertion sort implementation with an empty list
#[test]
fn empty() {
    let mut list: Vec<u32> = vec![];
    sort(&mut list);
    assert!(list.len() == 0);
}

/// Test the insertion sort implementation with a single-element list
#[test]
fn single() {
    let mut list = vec![42];
    sort(&mut list);
    assert!(list[0] == 42);
}

/// Test the insertion sort implementation with a short unsorted list
#[test]
fn unsorted() {
    let mut list = vec![3, 1, 0, 4];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
}

/// Test the insertion sort implementation with a short backward list
#[test]
fn reverse() {
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
fn sorted() {
    let mut list = vec![0, 1, 2, 3];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 2);
    assert!(list[3] == 3);
}

/// Make sure the sort is stable.
#[test]
fn stable() {
    let len = 256;
    let mut key1: usize = 0;
    let mut key2: usize = 0;
    #[derive(Debug)]
    struct Item {
        key1: usize,
        key2: usize,
    };
    let mut list: Vec<Item> = (0..len).map(|_| {
        key1 += 1;
        key1 %= 5;
        key2 += 1;
        return Item {
            key1: key1,
            key2: key2,
        };
    }).collect();
    insort::sort(&mut list, |a, b| a.key1.cmp(&b.key1));
    println!("{:?}", list);
    for i in (0 .. (len - 1)) {
        assert!(list[i].key1 <= list[i + 1].key1);
        if list[i].key1 == list[i + 1].key1 {
            assert!(list[i].key2 <= list[i + 1].key2);
        }
    }
}

/// Insertion sort implementation convenience used for tests.
pub fn sort<T: Ord>(list: &mut[T]) {
    insort::sort(list, |a, b| a.cmp(b) );
}

