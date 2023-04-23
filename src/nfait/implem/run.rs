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
use maplit::{hashset,hashmap};

use crate::traits::letter::AutLetter;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::error::AutError;
use crate::traits::run::AutRunnable;

impl<Letter: AutLetter> AutRunnable<Letter> for AutNFAIT<Letter> {

    fn runs_trace(&self, trace : &[Letter]) -> Result<bool, AutError<Letter>> {
        let mut current_states = self.get_epsilon_closure(&self.initials);
        let mut next_states = HashSet::new();
        // ***
        for letter in trace {
            for orig_state in &current_states {
                if let Some(targets) = self.transitions[*orig_state].get(letter) {
                    next_states.extend(self.get_epsilon_closure(targets));
                }
            }
            // ***
            std::mem::swap(&mut current_states, &mut next_states);
            next_states.clear();
            // ***
            if current_states.is_empty() {
                return Ok(false);
            }
        }
        // ***
        let verdict = current_states.iter().any(|x| self.finals.contains(x));
        // ***
        Ok(verdict)
    }

    fn run_transition(&self,
                      initial_states: &HashSet<usize>,
                      letter: &Letter) -> Result<HashSet<usize>, AutError<Letter>> {
        let mut next_states = hashset!{};
        let initial_closure = self.get_epsilon_closure(initial_states);
        for init_state in initial_closure {
            match self.transitions.get(init_state) {
                None => {
                    return Err(AutError::InvalidStateToRun(init_state,
                                                                self.transitions.len()));
                },
                Some(outgoing_transitions) => {
                    match outgoing_transitions.get(letter) {
                        None => {},
                        Some(targets) => {
                            next_states.extend(targets.iter().cloned())
                        }
                    }
                }
            }
        }
        // ***
        Ok(next_states)
    }
}