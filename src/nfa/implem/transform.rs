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
use maplit::hashmap;
use crate::dfa::dfa::AutDFA;
use crate::nfa::nfa::AutNFA;
use crate::traits::transform::AutTransformable;
use crate::traits::build::AutBuildable;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;




impl<Letter: AutLetter> AutTransformable<Letter> for AutNFA<Letter> {

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

    fn complete(mut self) -> Self {
        if self.is_complete() {
            return self;
        }
        let new_state_id = self.transitions.len();
        // we add a state which will be the target of all the added transitions
        // this state not being final, it won't change the set of accepted words
        self.transitions.push(HashMap::new());
        for transition_map in &mut self.transitions {
            for letter in &self.alphabet {
                let letter_targets = transition_map.entry(*letter).or_insert_with(HashSet::new);
                if letter_targets.is_empty() {
                    letter_targets.insert(new_state_id);
                }
            }
        }
        // ***
        if self.initials.is_empty() {
            self.initials.insert(new_state_id);
        }
        // ***
        self
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
        if self.initials.is_disjoint(&self.finals) {
            return false; // because doesn't accept empty word
        }
        // ***
        let mut reached: HashSet<usize> = self.initials.clone().into_iter().collect();
        let mut stack: Vec<usize> = self.initials.clone().into_iter().collect();
        // ***
        while let Some(accessible_state_id) = stack.pop() {
            for target_states in self.transitions[accessible_state_id].values() {
                for target in target_states {
                    if !self.finals.contains(target) {
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

    fn negate(self) -> Self {
        self.to_dfa().negate().to_nfa()
    }

    fn reverse(mut self) -> Self {
        let mut transitions  = vec![hashmap!{};self.transitions.len()];
        // ***
        for origin_state in 0..self.transitions.len() {
            for (letter, target_states) in &self.transitions[origin_state] {
                for target_state in target_states {
                    transitions[*target_state].entry(*letter).or_insert_with(HashSet::new).insert(origin_state);
                }
            }
        }
        // ***
        self.transitions = transitions;
        std::mem::swap(&mut self.initials, &mut self.finals);
        // ***
        self
    }

    fn minimize(self) -> Self {
        self.to_dfa().minimize().to_nfa()
    }

    // De Morgan
    fn intersect(self,
                 other: Self) -> Self {
        self.negate().unite(other.negate()).unwrap().negate()
    }

    fn contains(&self,
                other: &Self) -> bool {
        self.clone().negate().intersect(other.clone()).is_empty()
    }
}
