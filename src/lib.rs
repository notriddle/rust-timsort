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

//! This crate is a stable sorting algorithm with O(n) worst-case storage
//! requirements, O(n log n) worst-case comparisons, and O(n) comparisons
//! on an already-sorted list, smoothly becoming O(n log n) as the sorted
//! sections (runs) get smaller and smaller.

mod insort;
mod merge;
mod gallop;
mod find_run;

pub use merge::merge_by;
pub use merge::merge;
pub use gallop::gallop_left;
pub use gallop::gallop_left_by;
pub use gallop::gallop_right;
pub use gallop::gallop_right_by;
pub use find_run::find_run;
pub use find_run::find_run_by;
pub use find_run::get_run;
pub use find_run::get_run_by;
