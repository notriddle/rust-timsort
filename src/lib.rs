// This file is a part of Timsort-Rust. Based heavily on TimSort.java in
// Android.
// 
// Copyright (C) 2015 Michael Howell
// Copyright (C) 2008 The Android Open Source Project
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

