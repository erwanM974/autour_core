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

use std::collections::HashSet;


pub trait AutAccessible {
    /// An automaton is *accessible* if for all its *states* there is a (possibly empty) *path* from a *starting* state to that *state*
    fn is_accessible(&self) -> bool;
    fn get_all_accessible_states(&self) -> HashSet<usize>;
    fn make_accessible(self) -> Self;

    /// An automaton is *coaccessible* if for all its *states* there is a (possibly empty) *path* from that *state* to a *final state*
    fn is_coaccessible(&self) -> bool;
    fn get_all_coaccessible_states(&self) -> HashSet<usize>;
    fn make_coaccessible(self) -> Self;

    /// An automaton is said *trimmed* if it is *accessible* and *coaccessible*
    fn is_trimmed(&self) -> bool;
    fn trim(self) -> Self;
}


