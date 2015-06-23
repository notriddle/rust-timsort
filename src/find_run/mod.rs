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

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

/// Find a run, reversing if necessary.
pub fn get_run<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) -> usize {
    let (ord, len) = find_run(list, c);
    if ord {
        list.split_at_mut(len).0.reverse();
    }
    len
}


/// Find a run. Returns true if it needs reversed, and false otherwise.
pub fn find_run<T, C: Fn(&T, &T) -> Ordering>(list: &[T], c: C) -> (bool, usize) {
    let list_len = list.len();
    if list_len < 2 {
        return (false, list_len);
    }
    let mut pos = 1;
    match c(&list[1], &list[0]) {
        Ordering::Less => {
            while pos < list_len - 1 && c(&list[pos + 1], &list[pos]) == Ordering::Less {
                pos += 1;
            }
            (true, pos + 1)
        },
        _ => {
            while pos < list_len - 1 && c(&list[pos + 1], &list[pos]) != Ordering::Less {
                pos += 1;
            }
            (false, pos + 1)
        }
    }
}

