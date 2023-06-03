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
use maplit::hashmap;

use crate::nfait::nfait::AutNFAIT;
use crate::traits::build::AutBuildable;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;





impl<Letter: AutLetter> AutTransformable<Letter> for AutNFAIT<Letter> {

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

    fn negate(self) -> Self {
        self.to_dfa().negate().to_nfait()
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
        self.to_dfa().minimize().to_nfait()
    }

    // De Morgan
    fn intersect(self,
                 other: Self) -> Result<Self,AutError<Letter>> {
        match self.negate().unite(other.negate()) {
            Err(e) => {Err(e)},
            Ok(got) => {Ok(got.negate())}
        }
    }

    fn interleave(self, other: Self) -> Result<Self,AutError<Letter>> {
        match self.to_nfa().interleave(other.to_nfa()) {
            Err(e) => {Err(e)},
            Ok(got) => {Ok(got.to_nfait())}
        }
    }

}
