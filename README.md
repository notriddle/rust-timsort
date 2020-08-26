A modified merge sort that's faster on almost-sorted data
=========================================================
    
This is an implementation of TimSort, the default sorting algorithm used in
Python and newer versions of Java.

[Full documentation here.](https://www.notriddle.com/rustdoc/timsort/)

[![Build Status](https://travis-ci.org/notriddle/rust-timsort.svg)](https://travis-ci.org/notriddle/rust-timsort)


Performance
-----------

This is still an extreme work-in-progress, and performance has vast room for
improvement.

The benchmarks are the only part that doesn't work in pure stable rust.
`benches/bench.rs` is for rust-TimSort, `benches/bench_default.rs` is for the
default MergeSort that comes with Rust.

```
     Running target/release/bench-dae501bb89de0c02

running 16 tests
test sort_big_random_large   ... bench:   3,310,775 ns/iter (+/- 232,896) = 96
MB/s
test sort_big_random_medium  ... bench:       7,755 ns/iter (+/- 118) = 412 MB/s
test sort_big_random_small   ... bench:         297 ns/iter (+/- 19) = 538 MB/s
test sort_big_sorted         ... bench:      18,038 ns/iter (+/- 251) = 17740
MB/s
test sort_equals             ... bench:       1,342 ns/iter (+/- 93) = 5961 MB/s
test sort_few_unique         ... bench:      66,582 ns/iter (+/- 1,882) = 60
MB/s
test sort_huge               ... bench: 137,064,148 ns/iter (+/- 2,159,033) = 5
MB/s
test sort_partially_sorted   ... bench:   1,488,792 ns/iter (+/- 19,015) = 53
MB/s
test sort_random_large       ... bench:   2,010,436 ns/iter (+/- 92,632) = 39
MB/s
test sort_random_medium      ... bench:       4,134 ns/iter (+/- 39) = 193 MB/s
test sort_random_small       ... bench:         194 ns/iter (+/- 163) = 206 MB/s
test sort_sorted             ... bench:      13,043 ns/iter (+/- 540) = 6133
MB/s
test sort_strings            ... bench:  11,845,502 ns/iter (+/- 247,501) = 24
MB/s
test sort_tiny_random_large  ... bench:   2,049,110 ns/iter (+/- 12,357) = 4
MB/s
test sort_tiny_random_medium ... bench:       4,227 ns/iter (+/- 98) = 23 MB/s
test sort_tiny_random_small  ... bench:         166 ns/iter (+/- 4) = 30 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 16 measured

     Running target/release/bench_default-a77273e7b72e7094

running 16 tests
test sort_big_random_large   ... bench:   1,383,147 ns/iter (+/- 31,362) = 231
MB/s
test sort_big_random_medium  ... bench:       7,107 ns/iter (+/- 162) = 450 MB/s
test sort_big_random_small   ... bench:         288 ns/iter (+/- 8) = 555 MB/s
test sort_big_sorted         ... bench:     464,047 ns/iter (+/- 7,236) = 689
MB/s
test sort_equals             ... bench:      15,959 ns/iter (+/- 949) = 501 MB/s
test sort_few_unique         ... bench:      66,069 ns/iter (+/- 2,187) = 60
MB/s
test sort_huge               ... bench:   9,503,439 ns/iter (+/- 377,908) = 84
MB/s
test sort_partially_sorted   ... bench:     501,055 ns/iter (+/- 7,756) = 159
MB/s
test sort_random_large       ... bench:     668,164 ns/iter (+/- 23,727) = 119
MB/s
test sort_random_medium      ... bench:       3,685 ns/iter (+/- 35) = 217 MB/s
test sort_random_small       ... bench:         187 ns/iter (+/- 15) = 213 MB/s
test sort_sorted             ... bench:     266,431 ns/iter (+/- 7,553) = 300
MB/s
test sort_strings            ... bench:   2,299,735 ns/iter (+/- 58,825) = 127
MB/s
test sort_tiny_random_large  ... bench:     737,314 ns/iter (+/- 18,718) = 13
MB/s
test sort_tiny_random_medium ... bench:       4,141 ns/iter (+/- 45) = 24 MB/s
test sort_tiny_random_small  ... bench:         160 ns/iter (+/- 11) = 31 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 16 measured
```


License
------

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
