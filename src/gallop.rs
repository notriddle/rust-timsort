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

macro_rules! test_both {
    ($v:ident, $($x:expr);*) => {{
        let $v = Mode::Forward;
        $($x;)*;
        let $v = Mode::Reverse;
        $($x;)*;
    }}
}

#[test]
fn test_gallop_empty() {
    let list: &[usize] = &[];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn test_gallop_single_greater() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn test_gallop_single_equal() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn test_gallop_single_less() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn test_gallop_start_less() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn test_gallop_start_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn test_gallop_middle_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 2)
    }
}

#[test]
fn test_gallop_end_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&3, list, mode), 2);
        assert_eq!(gallop_right(&3, list, mode), 3)
    }
}

#[test]
fn test_gallop_end_greater() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&4, list, mode), 3);
        assert_eq!(gallop_right(&4, list, mode), 3)
    }
}

#[test]
fn test_gallop_end_middle_before() {
    let list: &[usize] = &[1, 3, 5];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn test_gallop_end_middle_after() {
    let list: &[usize] = &[1, 3, 5];
    test_both!{mode,
        assert_eq!(gallop_left(&4, list, mode), 2);
        assert_eq!(gallop_right(&4, list, mode), 2)
    }
}

#[test]
fn test_gallop_large_start_before() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn test_gallop_large_start_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn test_gallop_large_start_after() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn test_gallop_large_center_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&21, list, mode), 5);
        assert_eq!(gallop_right(&21, list, mode), 6)
    }
}

#[test]
fn test_gallop_large_center_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&20, list, mode), 5);
        assert_eq!(gallop_right(&20, list, mode), 5)
    }
}

#[test]
fn test_gallop_large_end_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&100, list, mode), 13);
        assert_eq!(gallop_right(&100, list, mode), 13)
    }
}

#[test]
fn test_gallop_large_end_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&101, list, mode), 13);
        assert_eq!(gallop_right(&101, list, mode), 14)
    }
}

#[test]
fn test_gallop_large_end_greater() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&102, list, mode), 14);
        assert_eq!(gallop_right(&102, list, mode), 14)
    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    Forward,
    Reverse
}

pub fn gallop_left<T: Ord>(key: &T, list: &[T], mode: Mode) -> usize {
    gallop_left_by(key, list, mode, |a, b| a.cmp(b) )
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the beginning of any cluster of equal items.
pub fn gallop_left_by<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], mode: Mode, c: C) -> usize {
    let (mut base, mut lim) = gallop(key, list, mode, &c);
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

pub fn gallop_right<T: Ord>(key: &T, list: &[T], mode: Mode) -> usize {
    gallop_right_by(key, list, mode, |a, b| a.cmp(b) )
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the end of any cluster of equal items.
pub fn gallop_right_by<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], mode: Mode, c: C) -> usize {
    let list_len = list.len();
    let (mut base, mut lim) = gallop(key, list, mode, &c);
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


fn gallop<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], mode: Mode, c: C) -> (usize, usize) {
    let list_len = list.len();
    if list_len == 0 {
        return (0, 0);
    }
    match mode {
        Mode::Forward => {
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
            return (prev_val, next_val - prev_val);
        },
        Mode::Reverse => {
            let mut prev_val = list_len;
            let mut next_val = ((prev_val + 1) / 2) - 1;
            loop {
                match c(&list[next_val], key) {
                    Ordering::Greater => {
                        prev_val = next_val + 1;
                        next_val = (next_val + 1) / 2;
                        if next_val != 0 {
                            next_val -= 1;
                        } else {
                            break;
                        }
                    },
                    Ordering::Less | Ordering::Equal => {
                        break;
                    },
                }
            }
            return (next_val, prev_val - next_val);
        }
    }
}
