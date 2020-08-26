use gallop::{self, Mode};

macro_rules! test_both {
    ($v:ident, $($x:expr);*) => {{
        let $v = Mode::Forward;
        $($x;)*;
        let $v = Mode::Reverse;
        $($x;)*;
    }}
}

#[test]
fn gallop_empty() {
    let list: &[usize] = &[];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn gallop_single_greater() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn gallop_single_equal() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn gallop_single_less() {
    let list: &[usize] = &[1];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn gallop_start_less() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn gallop_start_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn gallop_middle_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 2)
    }
}

#[test]
fn gallop_end_equal() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&3, list, mode), 2);
        assert_eq!(gallop_right(&3, list, mode), 3)
    }
}

#[test]
fn gallop_end_greater() {
    let list: &[usize] = &[1, 2, 3];
    test_both!{mode,
        assert_eq!(gallop_left(&4, list, mode), 3);
        assert_eq!(gallop_right(&4, list, mode), 3)
    }
}

#[test]
fn gallop_end_middle_before() {
    let list: &[usize] = &[1, 3, 5];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn gallop_end_middle_after() {
    let list: &[usize] = &[1, 3, 5];
    test_both!{mode,
        assert_eq!(gallop_left(&4, list, mode), 2);
        assert_eq!(gallop_right(&4, list, mode), 2)
    }
}

#[test]
fn gallop_large_start_before() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&0, list, mode), 0);
        assert_eq!(gallop_right(&0, list, mode), 0)
    }
}

#[test]
fn gallop_large_start_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&1, list, mode), 0);
        assert_eq!(gallop_right(&1, list, mode), 1)
    }
}

#[test]
fn gallop_large_start_after() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&2, list, mode), 1);
        assert_eq!(gallop_right(&2, list, mode), 1)
    }
}

#[test]
fn gallop_large_center_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&21, list, mode), 5);
        assert_eq!(gallop_right(&21, list, mode), 6)
    }
}

#[test]
fn gallop_large_center_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&20, list, mode), 5);
        assert_eq!(gallop_right(&20, list, mode), 5)
    }
}

#[test]
fn gallop_large_end_less() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&100, list, mode), 13);
        assert_eq!(gallop_right(&100, list, mode), 13)
    }
}

#[test]
fn gallop_large_end_equal() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&101, list, mode), 13);
        assert_eq!(gallop_right(&101, list, mode), 14)
    }
}

#[test]
fn gallop_large_end_greater() {
    let list: &[usize] = &[1, 3, 5, 7, 11, 21, 31, 41, 51, 61, 71, 81, 91, 101];
    test_both!{mode,
        assert_eq!(gallop_left(&102, list, mode), 14);
        assert_eq!(gallop_right(&102, list, mode), 14)
    }
}

pub fn gallop_left<T: Ord>(key: &T, list: &[T], mode: Mode) -> usize {
    gallop::gallop_left(key, list, mode, |a, b| a.cmp(b) )
}

pub fn gallop_right<T: Ord>(key: &T, list: &[T], mode: Mode) -> usize {
    gallop::gallop_right(key, list, mode, |a, b| a.cmp(b) )
}

