/*
Copyright 2023 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn shift_hashset_of_usize(a: &mut HashSet<usize>, l: usize) {
    for e in a.drain().collect::<Vec<usize>>() {
        a.insert(e + l);
    }
}

pub fn shift_hashmaps_of_hashset_usize<V: Eq + Hash>(a: &mut Vec<HashMap<V, HashSet<usize>>>, l: usize) {
    for map in a {
        for original_hashset in map.values_mut() {
            shift_hashset_of_usize(original_hashset,l);
        }
    }
}

pub fn shift_vec_of_hashset_usize(a: &mut Vec<HashSet<usize>>, l: usize) {
    for original_hashset in a {
        shift_hashset_of_usize(original_hashset,l);
    }
}





