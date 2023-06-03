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

use std::collections::HashMap;

use crate::dfa::dfa::AutDFA;
use crate::traits::build::AutBuildable;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;


impl<Letter : AutLetter> AutTransformable<Letter> for AutDFA<Letter> {

    fn complete(mut self) -> Self {
        if self.is_complete() {
            return self;
        }
        let new_state_id = self.transitions.len();
        // we add a state which will be the target of all the added transitions
        // this state not being final, it won't change the set of accepted words
        self.transitions.push(HashMap::new());
        // we iterate on the states
        for map in &mut self.transitions {
            for letter in &self.alphabet {
                // check if there is an outgoing transition with latter v
                if !map.contains_key(letter) {
                    map.insert(*letter, new_state_id);
                }
            }
        }
        // ***
        self
    }

    fn negate(mut self) -> Self {
        self = self.complete();
        self.finals = (0..self.transitions.len())
            .filter(|x| !self.finals.contains(x))
            .collect();
        // ***
        self
    }

    fn reverse(self) -> Self {
        self.to_nfa().reverse().to_dfa()
    }

    // Brzozowski
    fn minimize(self) -> Self {
        self.reverse().reverse()
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
            Ok(got) => {Ok(got.to_dfa())}
        }
    }
}
