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

use crate::traits::letter::AutLetter;
use crate::nfa::nfa::AutNFA;
use crate::traits::error::AutError;
use crate::traits::run::AutRunnable;


impl<Letter: AutLetter> AutRunnable<Letter> for AutNFA<Letter> {
    fn runs_trace(&self,
                  trace : &[Letter]) -> Result<bool, AutError<Letter>> {
        if self.initials.is_empty() {
            return Ok(false);
        }
        // ***
        let mut current_states = self.initials.clone();
        let mut next_states = HashSet::new();
        // ***
        for letter in trace {
            for orig_state in &current_states {
                if let Some(targets) = self.transitions[*orig_state].get(letter) {
                    for targ_state in targets {
                        next_states.insert(*targ_state);
                    }
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
        for init_state in initial_states {
            match self.transitions.get(*init_state) {
                None => {
                    return Err(AutError::InvalidStateToRun(*init_state,
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