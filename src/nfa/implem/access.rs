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
use std::iter::FromIterator;

use crate::nfa::nfa::AutNFA;
use crate::traits::access::AutAccessible;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;


impl<Letter: AutLetter> AutAccessible for AutNFA<Letter> {

    fn is_accessible(&self) -> bool {
        self.get_all_accessible_states().len() == self.transitions.len()
    }

    fn get_all_accessible_states(&self) -> HashSet<usize> {
        let mut set_of_accessible_states: HashSet<usize> = self.initials.clone();
        let mut stack: Vec<usize> = self.initials.iter().cloned().collect();
        while let Some(origin_state) = stack.pop() {
            for target_states in self.transitions[origin_state].values() {
                for target in target_states {
                    if !set_of_accessible_states.contains(target) {
                        set_of_accessible_states.insert(*target);
                        stack.push(*target);
                    }
                }
            }
        }
        // ***
        set_of_accessible_states
    }

    /// make the NFA accessible by removing all states which are not accessible from the initials
    /// by definition the new automaton accepts the same set of words
    fn make_accessible(mut self) -> Self {
        let set_of_accessible_states = self.get_all_accessible_states();
        // ***
        let mut states_substitution = HashMap::new();
        let mut current_state_index = 0;
        let l = self.transitions.len();
        for i in 0..l {
            if set_of_accessible_states.contains(&i) {
                states_substitution.insert(i, current_state_index);
                self.transitions.swap(i, current_state_index);
                current_state_index += 1;
            }
        }
        self.transitions.truncate(current_state_index);
        // ***
        self.finals = self.finals
            .iter()
            .filter(|x| set_of_accessible_states.contains(x))
            .map(|x| *states_substitution.get(x).unwrap())
            .collect();
        // ***
        self.initials = self.initials
            .iter()
            .map(|x| *states_substitution.get(x).unwrap())
            .collect();
        // ***
        for transition_map in &mut self.transitions {
            for target_states in transition_map.values_mut() {
                let substituted_target_states = HashSet::from_iter(
                    target_states
                        .iter()
                        .map(|target| *states_substitution.get(target).unwrap()));
                *target_states = substituted_target_states;
            }
        }
        // ***
        self
    }

    fn is_coaccessible(&self) -> bool {
        self.get_all_coaccessible_states().len() == self.transitions.len()
    }

    fn get_all_coaccessible_states(&self) -> HashSet<usize> {
        let mut set_of_coaccessible_states = self.finals.clone();
        let mut stack: Vec<usize> = self.finals.iter().cloned().collect();
        while let Some(next_state) = stack.pop() {
            for (orig_state,transitions) in self.transitions.iter().enumerate() {
                for target_states in transitions.values() {
                    for target_state in target_states {
                        if next_state == *target_state && !set_of_coaccessible_states.contains(&orig_state) {
                            set_of_coaccessible_states.insert(orig_state);
                            stack.push(orig_state);
                        }
                    }
                }
            }
        }
        // ***
        set_of_coaccessible_states
    }

    fn make_coaccessible(self) -> Self {
        self.reverse().make_accessible().reverse()
    }

    fn is_trimmed(&self) -> bool {
        self.is_accessible() && self.is_coaccessible()
    }

    fn trim(self) -> Self {
        self.make_accessible().make_coaccessible()
    }
}