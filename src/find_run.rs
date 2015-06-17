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

//! The run finder algorithm. Takes an unsorted slice, and returns the number
//! of sequential elements in a row.

use std::cmp::Ordering;

#[test]
fn test_empty() {
    let list: Vec<usize> = vec![];
    let (ord, len) = find_run(&list);
    assert_eq!(ord, false);
    assert_eq!(len, 0);
}

#[test]
fn test_single() {
    let (ord, len) = find_run(&vec![1]);
    assert_eq!(ord, false);
    assert_eq!(len, 1);
}

#[test]
fn test_greater() {
    let (ord, len) = find_run(&vec![1, 2, 2, 3, 4, 5]);
    assert_eq!(ord, false);
    assert_eq!(len, 6);
}

// Note: I used to have a version that would allow sub-runs of equal elements in a
// less ordering. Unfortunately, reversing those sub-runs creates an unstable sort.
#[test]
fn test_less_stable() {
    let (ord, len) = find_run(&vec![5, 4, 4, 3, 4, 5]);
    assert_eq!(ord, true);
    assert_eq!(len, 2);
}

#[test]
fn test_less() {
    let (ord, len) = find_run(&vec![5, 4, 3, 2, 1, 0]);
    assert_eq!(ord, true);
    assert_eq!(len, 6);
}

#[test]
fn test_equal() {
    let (ord, len) = find_run(&vec![2, 2, 2, 2, 2, 2]);
    assert_eq!(ord, false);
    assert_eq!(len, 6);
}

#[test]
fn test_get_run_reverse() {
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
fn test_get_run_noreverse() {
    let mut list = vec![3, 4, 5, 6, 7, 3];
    let len = get_run(&mut list);
    assert_eq!(len, 5);
    assert_eq!(list[0], 3);
    assert_eq!(list[1], 4);
    assert_eq!(list[2], 5);
    assert_eq!(list[3], 6);
    assert_eq!(list[4], 7);
}


/// Finds the length and type of the run. Starts at index 0 (if you need to find
/// one elsewhere, use split_at()).
pub fn find_run<T: Ord>(list: &[T]) -> (bool, usize) {
    find_run_by(list, |a, b| a.cmp(b))
}


/// Find a run, reversing if necessary.
pub fn get_run<T: Ord>(list: &mut [T]) -> usize {
    get_run_by(list, |a, b| a.cmp(b))
}


/// Find a run, reversing if necessary.
pub fn get_run_by<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) -> usize {
    let (ord, len) = find_run_by(list, c);
    if ord {
        list.split_at_mut(len).0.reverse();
    }
    return len;
}


/// Equivalent to `find_run` with a custom comparator. Returns true if it needs reversed,
/// and false otherwise.
pub fn find_run_by<T, C: Fn(&T, &T) -> Ordering>(list: &[T], c: C) -> (bool, usize) {
    let list_len = list.len();
    if list_len == 0 {
        return (false, 0);
    }
    if list_len == 1 {
        return (false, 1);
    }
    let mut pos = 1;
    match c(&list[1], &list[0]) {
        Ordering::Less => {
            while pos < list_len - 1 && c(&list[pos + 1], &list[pos]) == Ordering::Less {
                pos += 1;
            }
            return (true, pos + 1);
        },
        _ => {
            while pos < list_len - 1 && c(&list[pos + 1], &list[pos]) != Ordering::Less {
                pos += 1;
            }
            return (false, pos + 1);
        }
    }
}

