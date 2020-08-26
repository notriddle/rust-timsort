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

