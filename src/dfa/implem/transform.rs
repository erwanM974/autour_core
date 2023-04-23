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
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;


impl<Letter : AutLetter> AutTransformable<Letter> for AutDFA<Letter> {

    fn is_complete(&self) -> bool {
        for map in &self.transitions {
            for letter in &self.alphabet {
                if !map.contains_key(letter) {
                    return false;
                }
            }
        }
        // ***
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

    fn is_empty(&self) -> bool {
        self.to_nfa().is_empty()
    }

    fn is_universal(&self) -> bool {
        self.to_nfa().is_universal()
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
                 other: Self) -> Self {
        self.negate().unite(other.negate()).unwrap().negate()
    }

    fn contains(&self,
                other: &Self) -> bool {
        self.to_nfa().contains(&other.to_nfa())
    }

}
