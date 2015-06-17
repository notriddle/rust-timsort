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

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

/// Sorts the list using insertion sort.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
pub fn sort<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) {
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
