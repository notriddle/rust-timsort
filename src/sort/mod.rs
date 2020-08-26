//! The top sorting algorithm; that is, the modified merge sort we keep
//! talking about.

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::cmp::min;
use find_run::get_run;
use insort;
use merge::merge;

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
}

impl<'a, T: 'a, C: Fn(&T, &T) -> Ordering> SortState<'a, T, C> {

    fn new(list: &'a mut [T], c: C) -> SortState<'a, T, C> {
        SortState {
            list: list,
            c: c,
            runs: Vec::new(),
            pos: 0,
        }
    }

    /// The outer loop. Find runs, and move forward.
    fn sort(&mut self) {
        let list_len = self.list.len();
        // Minimum run size to use merge sort on. Any sorted sections of the
        // list that are shorter than this are lengthened using `insort::sort`.
        let min_run = calc_min_merge(list_len);
        while self.pos < list_len {
            let pos = self.pos;
            let mut run_len = get_run(self.list.split_at_mut(pos).1, &self.c);
            let run_min_len = min(min_run, list_len - pos);
            if run_len < run_min_len {
                run_len = run_min_len;
                let l = self.list.split_at_mut(pos).1.split_at_mut(run_len).0;
                insort::sort(l, &self.c);
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
                debug_assert_eq!(run1.pos + run1.len, run2.pos);
                runs.remove(pos2);
                runs[pos1] = Run{
                    pos: run1.pos,
                    len: run1.len + run2.len,
                };
                let l = self.list.split_at_mut(run1.pos).1;
                let l = l.split_at_mut(run1.len + run2.len).0;
                merge(l, run1.len, &self.c);
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
            let (pos1, pos2) = if n > 0 && runs[n - 1].len < runs[n + 1].len {
                (n - 1, n)
            } else {
                (n, n + 1)
            };
            let (run1, run2) = (runs[pos1], runs[pos2]);
            debug_assert_eq!(run1.len, run2.pos);
            runs.remove(pos2);
            runs[pos1] = Run{
                pos: run1.pos,
                len: run1.len + run2.len,
            };
            let l = self.list.split_at_mut(run1.pos).1;
            let l = l.split_at_mut(run1.len + run2.len).0;
            merge(l, run1.len, &self.c);
        }
    }
}

/// Sorts the list using merge sort.
///
/// `c(a, b)` should return std::cmp::Ordering::Greater when `a` is greater than `b`.
pub fn sort<T, C: Fn(&T, &T) -> Ordering>(list: &mut [T], c: C) {
    if list.len() < MIN_MERGE {
        insort::sort(list, c);
    } else {
        let mut sort_state = SortState::new(list, c);
        sort_state.sort();
    }
}
