//! The galloping search algorithm. 

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum Mode {
    Forward,
    Reverse
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the beginning of any cluster of equal items.
pub fn gallop_left<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], mode: Mode, c: C) -> usize {
    let (mut base, mut lim) = gallop(key, list, mode, &c);
    while lim != 0 {
        let ix = base + (lim / 2);
        match c(&list[ix], key) {
            Ordering::Less => {
                base = ix + 1;
                lim -= 1;
            },
            Ordering::Greater => (),
            Ordering::Equal => {
                if ix == 0 || c(&list[ix - 1], key) == Ordering::Less {
                    base = ix;
                    break;
                }
            },
        };
        lim /= 2;
    }
    base
}

/// Returns the index where key should be inserted, assuming it shoul be placed
/// at the end of any cluster of equal items.
pub fn gallop_right<T, C: Fn(&T, &T) -> Ordering>(key: &T, list: &[T], mode: Mode, c: C) -> usize {
    let list_len = list.len();
    let (mut base, mut lim) = gallop(key, list, mode, &c);
    while lim != 0 {
        let ix = base + (lim / 2);
        match c(&list[ix], key) {
            Ordering::Less => {
                base = ix + 1;
                lim -= 1;
            },
            Ordering::Greater => (),
            Ordering::Equal => {
                base = ix + 1;
                if ix == list_len - 1 || c(&list[ix + 1], key) == Ordering::Greater {
                    break;
                } else {
                    lim -= 1;
                }
            },
        };
        lim /= 2;
    }
    base
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
            (prev_val, next_val - prev_val)
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
            (next_val, prev_val - next_val)
        }
    }
}
