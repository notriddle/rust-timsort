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

//! The top sorting algorithm; that is, the modified merge sort we keep
//! talking about.

use std::cmp::Ordering;
use std::cmp::min;
use find_run::get_run_by;
use insort;
use merge;

/// Test the sort implementation with an empty list
#[test]
fn test_empty() {
    let mut list: Vec<u32> = vec![];
    sort(&mut list);
    assert!(list.len() == 0);
}

/// Test the sort implementation with a single-element list
#[test]
fn test_single() {
    let mut list = vec![42];
    sort(&mut list);
    assert!(list[0] == 42);
}

/// Test the sort implementation with a short unsorted list
#[test]
fn test_unsorted() {
    let mut list = vec![3, 1, 0, 4];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 3);
    assert!(list[3] == 4);
}

/// Test the sort implementation with a short backward list
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

/// Test the sort implementation with a short unsorted list
#[test]
fn test_sorted() {
    let mut list = vec![0, 1, 2, 3];
    sort(&mut list);
    assert!(list[0] == 0);
    assert!(list[1] == 1);
    assert!(list[2] == 2);
    assert!(list[3] == 3);
}

/// Sort implementation convenience used for tests.
pub fn sort<T: Ord>(list: &mut[T]) {
    sort_by(list, |a, b| a.cmp(b) );
}

/// Minimum run length to merge; anything shorter will be lengthend and
/// sorted using `insort::sort`.
const MIN_MERGE: usize = 64;

/// Compute the actual minimum merge size for a particular list.
fn calc_min_merge(mut len: usize) -> usize {
    if len < MIN_MERGE {
        len
    } else {
        let mut r: usize = 0;
        while len >= MIN_MERGE {
            r |= len & 1;
            len >>= 1;
        }
        len + r
    }
}

/// Represents a known-sorted sublist.
#[derive(Copy, Clone, Debug)]
struct Run {
    pos: usize,
    len: usize
}

/// All the ongoing state of the sort.
struct SortState<'a, T: 'a, C: Fn(&T, &T) -> Ordering> {
    /// The list that is being sorted.
    list: &'a mut [T],
    /// The comparator function. Should return `Ordering::Greater` if the first
    /// argument is goes after the second.
    c: C,
    /// The list of known-sorted sections of the list that can be merged.
    /// To keep the size of this list down, this invariant is preserved:
    ///  - `runs.len < 3 || runs[i-2].len > runs[i-1].len + runs[i].len`
    ///  - `runs.len < 2 || runs[i-1].len > runs[i].len`
    runs: Vec<Run>,
    /// The current position in the list. When `pos == list.len()`, we can now
    /// merge the last of the runs, and we're done.
    pos: usize,
    /// Minimum run size to use merge sort on. Any sorted sections of the list
    /// that are shorter than this are lengthened using `insort::sort`.
    min_run: usize,
}

impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> SortState<'a, T, C> {

    fn new(list: &'a mut [T], c: C) -> SortState<'a, T, C> {
        SortState {
            min_run: calc_min_merge(list.len()),
            list: list,
            c: c,
            runs: Vec::new(),
            pos: 0,
        }
    }

    /// The outer loop. Find runs, and move forward.
    fn sort(&mut self) {
        let list_len = self.list.len();
        let min_run = self.min_run;
        while self.pos < list_len {
            let pos = self.pos;
            let mut run_len = get_run_by(self.list.split_at_mut(pos).1, &self.c);
            let run_min_len = min(min_run, list_len - pos);
            if run_len < run_min_len {
                run_len = run_min_len;
                let l = self.list.split_at_mut(pos).1.split_at_mut(run_len).0;
                insort::sort_by(l, &self.c);
            }
            self.runs.push(Run{
                pos: pos,
                len: run_len,
            });
            self.pos += run_len;
            self.merge_collapse();
        }
        self.merge_force_collapse();
    }

    /// Merge the runs if they're too big.
    /// Copied almost verbatim from
    /// http://envisage-project.eu/proving-android-java-and-python-sorting-algorithm-is-broken-and-how-to-fix-it/#sec3.2
    fn merge_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;
            if    (n >= 1 && runs[n - 1].len <= runs[n].len + runs[n + 1].len)
               || (n >= 2 && runs[n - 2].len <= runs[n].len + runs[n - 1].len) {
                let (pos1, pos2) = if runs[n - 1].len < runs[n + 1].len {
                    (n - 1, n)
                } else {
                    (n, n + 1)
                };
                let (run1, run2) = (runs[pos1], runs[pos2]);
                runs.remove(pos2);
                runs[pos1] = Run{
                    pos: run1.pos,
                    len: run1.len + run2.len,
                };
                let l = self.list.split_at_mut(run1.pos).1;
                let l = l.split_at_mut(run1.len + run2.len).0;
                merge::merge_by(l, run1.len, &self.c);
            } else {
                break; // Invariant established.
            }
        }
    }

    /// Merge any outstanding runs, at the end.
    fn merge_force_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;
            let (pos1, pos2) = if runs[n - 1].len < runs[n + 1].len {
                (n - 1, n)
            } else {
                (n, n + 1)
            };
            let (run1, run2) = (runs[pos1], runs[pos2]);
            runs.remove(pos2);
            runs[pos1] = Run{
                pos: run1.pos,
                len: run1.len + run2.len,
            };
            let l = self.list.split_at_mut(run1.pos).1;
            let l = l.split_at_mut(run1.len + run2.len).0;
            merge::merge_by(l, run1.len, &self.c);
        }
    }
}

/// Sorts the list using merge sort.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
pub fn sort_by<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) {
    let mut sort_state = SortState::new(list, c);
    sort_state.sort();
}
