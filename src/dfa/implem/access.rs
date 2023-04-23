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
use maplit::hashset;

use crate::dfa::dfa::AutDFA;
use crate::traits::access::AutAccessible;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutAccessible for AutDFA<Letter> {

    fn is_accessible(&self) -> bool {
        self.get_all_accessible_states().len() == self.transitions.len()
    }

    fn get_all_accessible_states(&self) -> HashSet<usize> {
        let mut set_of_accessible_states = hashset!{self.initial};
        let mut stack = vec![self.initial];
        while let Some(origin_state) = stack.pop() {
            for target_state in self.transitions[origin_state].values() {
                if !set_of_accessible_states.contains(target_state) {
                    set_of_accessible_states.insert(*target_state);
                    stack.push(*target_state);
                }
            }
        }
        // ***
        set_of_accessible_states
    }

    fn make_accessible(self) -> Self {
        self.to_nfa().make_accessible().to_dfa()
    }

    fn is_coaccessible(&self) -> bool {
        self.get_all_coaccessible_states().len() == self.transitions.len()
    }

    fn get_all_coaccessible_states(&self) -> HashSet<usize> {
        let mut set_of_coaccessible_states = self.finals.clone();
        let mut stack: Vec<usize> = self.finals.iter().cloned().collect();
        while let Some(next_state) = stack.pop() {
            for (orig_state,transitions) in self.transitions.iter().enumerate() {
                for target_state in transitions.values() {
                    if next_state == *target_state && !set_of_coaccessible_states.contains(&orig_state) {
                        set_of_coaccessible_states.insert(orig_state);
                        stack.push(orig_state);
                    }
                }
            }
        }
        // ***
        set_of_coaccessible_states
    }

    fn make_coaccessible(self) -> Self {
        self.to_nfa().make_coaccessible().to_dfa()
    }

    fn is_trimmed(&self) -> bool {
        self.to_nfa().is_trimmed()
    }

    fn trim(self) -> Self {
        self.to_nfa().trim().to_dfa()
    }
}
