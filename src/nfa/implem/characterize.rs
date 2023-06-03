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

use crate::nfa::nfa::AutNFA;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;

impl<Letter : AutLetter> AutCharacterizable<Letter> for AutNFA<Letter> {

    fn is_complete(&self) -> bool {
        if self.initials.is_empty() {
            return false;
        }
        for transition_map in &self.transitions {
            for letter in &self.alphabet {
                if match transition_map.get(letter) {
                    None => true,
                    Some(letter_targets) => letter_targets.is_empty(),
                } {
                    return false;
                }
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        // by def
        /* let set_of_accessible_states = self.get_all_accessible_states();
           return set_of_accessible_states.is_disjoint(&self.finals); */
        // more efficient
        if !self.initials.is_disjoint(&self.finals) {
            return false; // because accepts empty word
        }
        // ***
        let mut reached: HashSet<usize> = self.initials.clone().into_iter().collect();
        let mut stack: Vec<usize> = self.initials.clone().into_iter().collect();
        // ***
        while let Some(accessible_state_id) = stack.pop() {
            for target_states in self.transitions[accessible_state_id].values() {
                for target in target_states {
                    if self.finals.contains(target) {
                        return false;
                    }
                    if !reached.contains(target) {
                        reached.insert(*target);
                        stack.push(*target);
                    }
                }
            }
        }
        // ***
        true
    }

    fn is_universal(&self) -> bool {
        self.clone().negate().is_empty()
    }

    fn contains(&self,
                other: &Self) -> bool {
        self.clone().negate().intersect(other.clone()).unwrap().is_empty()
    }

}

