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
use maplit::{hashset};

use crate::nfait::nfait::AutNFAIT;
use crate::traits::access::AutAccessible;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;


impl <Letter: AutLetter> AutNFAIT<Letter> {
    pub fn get_epsilon_closure(&self, states : &HashSet<usize>) -> HashSet<usize> {
        let mut closure = hashset!{};
        let mut to_iter: Vec<usize> = states.iter().cloned().collect();
        while let Some(next) = to_iter.pop() {
            if !closure.contains(&next) {
                let next_targets : &HashSet<usize> = self.epsilon_trans.get(next).unwrap();
                let mut as_target_vec : Vec<usize> = next_targets.iter().cloned().collect();
                to_iter.append( &mut as_target_vec);
                closure.insert(next);
            }
        }
        // ***
        closure
    }
}

impl<Letter: AutLetter> AutAccessible for AutNFAIT<Letter> {

    fn is_accessible(&self) -> bool {
        self.get_all_accessible_states().len() == self.transitions.len()
    }

    fn get_all_accessible_states(&self) -> HashSet<usize> {
        let mut accessible_closures : Vec<HashSet<usize>>;
        let mut stack: Vec<HashSet<usize>>;
        {
            let init = self.get_epsilon_closure( &self.initials);
            stack = vec![init.clone()];
            accessible_closures = vec![init];
        }
        // ***
        while let Some(current_closure) = stack.pop() {
            for origin_state in current_closure {
                for target_states in self.transitions[origin_state].values() {
                    for target in target_states {
                        let target_closure = self.get_epsilon_closure(&hashset!{*target});
                        if !accessible_closures.contains(&target_closure) {
                            accessible_closures.push(target_closure.clone());
                            stack.push(target_closure);
                        }
                    }
                }
            }
        }
        // ***
        let mut set_of_accessible_states = hashset!{};
        for closure in accessible_closures {
            set_of_accessible_states.extend(&closure);
        }
        // ***
        set_of_accessible_states
    }

    /// make the NFAIT accessible by removing all states which are not accessible from the initials
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
                self.epsilon_trans.swap(i, current_state_index);
                current_state_index += 1;
            }
        }
        self.transitions.truncate(current_state_index);
        self.epsilon_trans.truncate(current_state_index);
        // ***
        self.finals = self
            .finals
            .iter()
            .filter(|x| set_of_accessible_states.contains(x))
            .map(|x| *states_substitution.get(x).unwrap())
            .collect();
        // ***
        self.initials = self.initials.iter().map(|x| *states_substitution.get(x).unwrap()).collect();
        // ***
        for transition_map in &mut self.transitions {
            for target_states in transition_map.values_mut() {
                let substituted_target_states : HashSet<usize> = HashSet::from_iter(target_states.iter().map(|target| *states_substitution.get(target).unwrap()));
                *target_states = substituted_target_states;
                /*for target in target_states.values_mut() {
                    *target = *states_substitution.get(target).unwrap();
                }*/
            }
        }
        // ***
        self
    }

    fn is_coaccessible(&self) -> bool {
        self.get_all_coaccessible_states().len() == self.transitions.len()
    }

    fn get_all_coaccessible_states(&self) -> HashSet<usize> {
        let mut targets_of : Vec<HashSet<usize>> = vec![];
        for orig_state in 0..self.transitions.len() {
            let mut targets_of_orig : HashSet<usize> = self.epsilon_trans.get(orig_state).unwrap().clone();
            for lit_targs in self.transitions.get(orig_state).unwrap().values() {
                targets_of_orig.extend(lit_targs.iter().cloned());
            }
            targets_of.push(targets_of_orig);
        }
        // ***
        let mut set_of_coaccessible_states = self.finals.clone();
        let mut stack: Vec<usize> = self.finals.iter().cloned().collect();
        while let Some(target_state) = stack.pop() {
            for orig_state in 0..self.transitions.len() {
                if targets_of.get(orig_state).unwrap().contains(&target_state)
                        && !set_of_coaccessible_states.contains(&orig_state) {
                    set_of_coaccessible_states.insert(orig_state);
                    stack.push(orig_state);
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