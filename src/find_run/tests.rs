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

use find_run;

#[test]
fn empty() {
    let list: Vec<usize> = vec![];
    let (ord, len) = find_run(&list);
    assert_eq!(ord, false);
    assert_eq!(len, 0);
}

#[test]
fn single() {
    let (ord, len) = find_run(&vec![1]);
    assert_eq!(ord, false);
    assert_eq!(len, 1);
}

#[test]
fn greater() {
    let (ord, len) = find_run(&vec![1, 2, 2, 3, 4, 5]);
    assert_eq!(ord, false);
    assert_eq!(len, 6);
}

// Note: I used to have a version that would allow sub-runs of equal elements in a
// less ordering. Unfortunately, reversing those sub-runs creates an unstable sort.
#[test]
fn less_stable() {
    let (ord, len) = find_run(&vec![5, 4, 4, 3, 4, 5]);
    assert_eq!(ord, true);
    assert_eq!(len, 2);
}

#[test]
fn less() {
    let (ord, len) = find_run(&vec![5, 4, 3, 2, 1, 0]);
    assert_eq!(ord, true);
    assert_eq!(len, 6);
}

#[test]
fn equal() {
    let (ord, len) = find_run(&vec![2, 2, 2, 2, 2, 2]);
    assert_eq!(ord, false);
    assert_eq!(len, 6);
}

#[test]
fn get_run_reverse() {
    let mut list = vec![7, 6, 5, 4, 3, 3];
    let len = get_run(&mut list);
    assert_eq!(len, 5);
    assert_eq!(list[0], 3);
    assert_eq!(list[1], 4);
    assert_eq!(list[2], 5);
    assert_eq!(list[3], 6);
    assert_eq!(list[4], 7);
}

#[test]
fn get_run_noreverse() {
    let mut list = vec![3, 4, 5, 6, 7, 3];
    let len = get_run(&mut list);
    assert_eq!(len, 5);
    assert_eq!(list[0], 3);
    assert_eq!(list[1], 4);
    assert_eq!(list[2], 5);
    assert_eq!(list[3], 6);
    assert_eq!(list[4], 7);
}


/// With comparator.
pub fn find_run<T: Ord>(list: &[T]) -> (bool, usize) {
    find_run::find_run(list, |a, b| a.cmp(b))
}


/// With comparator.
pub fn get_run<T: Ord>(list: &mut [T]) -> usize {
    find_run::get_run(list, |a, b| a.cmp(b))
}

