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

//! The galloping search algorithm. 

use std::cmp::Ordering;

#[test]
fn test_gallop_empty() {
    let list: &[usize] = &[];
    assert_eq!(gallop_left(&0, list), 0);
    assert_eq!(gallop_right(&0, list), 0);
}

#[test]
fn test_gallop_single_greater() {
    let list: &[usize] = &[1];
    assert_eq!(gallop_left(&0, list), 0);
    assert_eq!(gallop_right(&0, list), 0);
}

#[test]
fn test_gallop_single_equal() {
    let list: &[usize] = &[1];
    assert_eq!(gallop_left(&1, list), 0);
    assert_eq!(gallop_right(&1, list), 1);
}

#[test]
fn test_gallop_single_less() {
    let list: &[usize] = &[1];
    assert_eq!(gallop_left(&2, list), 1);
    assert_eq!(gallop_right(&2, list), 1);
}

#[test]
fn test_gallop_start_less() {
    let list: &[usize] = &[1, 2, 3];
    assert_eq!(gallop_left(&0, list), 0);
    assert_eq!(gallop_right(&0, list), 0);
}

#[test]
fn test_gallop_start_equal() {
    let list: &[usize] = &[1, 2, 3];
    assert_eq!(gallop_left(&1, list), 0);
    assert_eq!(gallop_right(&1, list), 1);
}

#[test]
fn test_gallop_middle_equal() {
    let list: &[usize] = &[1, 2, 3];
    assert_eq!(gallop_left(&2, list), 1);
    assert_eq!(gallop_right(&2, list), 2);
}

#[test]
fn test_gallop_end_equal() {
    let list: &[usize] = &[1, 2, 3];
    assert_eq!(gallop_left(&3, list), 2);
    assert_eq!(gallop_right(&3, list), 3);
}

#[test]
fn test_gallop_end_greater() {
    let list: &[usize] = &[1, 2, 3];
    assert_eq!(gallop_left(&4, list), 3);
    assert_eq!(gallop_right(&4, list), 3);
}

#[test]
fn test_gallop_end_middle_before() {
    let list: &[usize] = &[1, 3, 5];
    assert_eq!(gallop_left(&2, list), 1);
    assert_eq!(gallop_right(&2, list), 1);
}

#[test]
fn test_gallop_end_middle_after() {
    let list: &[usize] = &[1, 3, 5];
    assert_eq!(gallop_left(&4, list), 2);
    assert_eq!(gallop_right(&4, list), 2);
}

#[test]
fn test_gallop_large_start_before() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&0, list), 0);
    assert_eq!(gallop_right(&0, list), 0);
}

#[test]
fn test_gallop_large_start_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&1, list), 0);
    assert_eq!(gallop_right(&1, list), 1);
}

#[test]
fn test_gallop_large_start_after() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&2, list), 1);
    assert_eq!(gallop_right(&2, list), 1);
}

#[test]
fn test_gallop_large_center_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&21, list), 5);
    assert_eq!(gallop_right(&21, list), 6);
}

#[test]
fn test_gallop_large_center_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&20, list), 5);
    assert_eq!(gallop_right(&20, list), 5);
}

#[test]
fn test_gallop_large_end_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&100, list), 13);
    assert_eq!(gallop_right(&100, list), 13);
}

#[test]
fn test_gallop_large_end_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&101, list), 13);
    assert_eq!(gallop_right(&101, list), 14);
}

#[test]
fn test_gallop_large_end_greater() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    assert_eq!(gallop_left(&102, list), 14);
    assert_eq!(gallop_right(&102, list), 14);
}

pub fn gallop_left<T: Ord>(key: &T, list: &[T]) -> usize {
    gallop_left_by(key, list, |a, b| a.cmp(b) )
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the beginning of any cluster of equal items.
pub fn gallop_left_by<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], c: C) -> usize {
    let list_len = list.len();
    let mut prev_val = 0;
    let mut next_val = 1;
    while next_val < list_len {
        match c(&list[next_val], key) {
            Ordering::Less => {
                prev_val = next_val;
                next_val = ((next_val + 1) * 2) - 1;
            },
            Ordering::Greater => {
                break;
            },
            Ordering::Equal => {
                next_val += 1;
                break;
            },
        }
    }
    if next_val > list_len {
        next_val = list_len;
    }
    let mut base = prev_val;
    let mut lim  = next_val - prev_val;
    while lim != 0 {
        let ix = base + (lim >> 1);
        match c(&list[ix], key) {
            Ordering::Less => {
                base = ix + 1;
                lim -= 1;
            },
            Ordering::Greater => (),
            Ordering::Equal => {
                if ix == 0 || c(&list[ix - 1], key) == Ordering::Less {
                    return ix;
                }
            },
        };
        lim >>= 1;
    }
    return base;
}

pub fn gallop_right<T: Ord>(key: &T, list: &[T]) -> usize {
    gallop_right_by(key, list, |a, b| a.cmp(b) )
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the end of any cluster of equal items.
pub fn gallop_right_by<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], c: C) -> usize {
    let list_len = list.len();
    let mut prev_val = 0;
    let mut next_val = 1;
    while next_val < list_len {
        match c(&list[next_val], key) {
            Ordering::Less => {
                prev_val = next_val;
                next_val = ((next_val + 1) * 2) - 1;
            },
            Ordering::Greater => {
                break;
            },
            Ordering::Equal => {
                next_val += 1;
                break;
            },
        }
    }
    if next_val > list_len {
        next_val = list_len;
    }
    let mut base = prev_val;
    let mut lim  = next_val - prev_val;
    while lim != 0 {
        let ix = base + (lim >> 1);
        match c(&list[ix], key) {
            Ordering::Less => {
                base = ix + 1;
                lim -= 1;
            },
            Ordering::Greater => (),
            Ordering::Equal => {
                if ix == list_len - 1 || c(&list[ix + 1], key) == Ordering::Greater {
                    return ix + 1;
                } else {
                    base = ix + 1;
                    lim -= 1;
                }
            },
        };
        lim >>= 1;
    }
    return base;
}

