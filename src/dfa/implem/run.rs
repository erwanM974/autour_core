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
use crate::traits::run::AutRunnable;
use crate::dfa::dfa::AutDFA;
use crate::traits::error::AutError;


impl<Letter: AutLetter> AutRunnable<Letter> for AutDFA<Letter> {
    fn runs_trace(&self, trace : &[Letter]) -> Result<bool,AutError<Letter>> {
        let mut current_state = self.initial;
        for letter in trace {
            match self.transitions[current_state].get(letter) {
                None => {
                    return Ok(false);
                },
                Some(target_state) => {
                    current_state = *target_state;
                }
            }
        }
        // ***
        Ok(self.finals.contains(&current_state))
    }

    fn run_transition(&self,
                      active_states: &HashSet<usize>,
                      letter: &Letter) ->  Result<HashSet<usize>,AutError<Letter>> {
        if active_states.len() != 1 {
            return Err(AutError::Other(format!("dfa has at most one active state and not {:?}", active_states)));
        }
        let init_state = *active_states.iter().next().unwrap();
        match self.transitions[init_state].get(letter) {
            None => {
                Ok(hashset!{})
            },
            Some(target_state) => {
                Ok(hashset!{*target_state})
            }
        }
    }
}