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
use maplit::{hashmap, hashset};
use crate::nfa::algos::kameda_weiner::algo::kameda_weiner_algorithm;

use crate::nfa::nfa::AutNFA;
use crate::traits::transform::AutTransformable;
use crate::traits::build::AutBuildable;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutTransformable<Letter> for AutNFA<Letter> {

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
        kameda_weiner_algorithm(&self).3.nfa
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
        if self.alphabet != other.alphabet {
            return Err(AutError::OperationOnLanguagesOverDifferentAlphabets(self.alphabet,other.alphabet));
        }
        let mut new_initials : HashSet<usize> = hashset! {};
        let mut new_finals : HashSet<usize> = hashset! {};
        let cross_states_map : Vec<(usize,usize)> = {
            let mut csm = vec![];
            for x in 0..self.transitions.len() {
                for y in 0..other.transitions.len() {
                    let index = csm.len();
                    csm.push((x,y));
                    if self.initials.contains(&x) && other.initials.contains(&y) {
                        new_initials.insert(index);
                    }
                    if self.finals.contains(&x) && other.finals.contains(&y) {
                        new_finals.insert(index);
                    }
                }
            }
            csm
        };
        let mut new_transitions : Vec<HashMap<Letter, HashSet<usize>>>= vec![];
        for _ in 0..cross_states_map.len() {
            new_transitions.push(hashmap!{});
        }
        for y in 0..other.transitions.len() {
            for (x_orig,transitions) in self.transitions.iter().enumerate() {
                let new_orig = cross_states_map.iter().position(|&r| r == (x_orig,y)).unwrap();
                let outgoing = new_transitions.get_mut(new_orig).unwrap();
                for (letter,x_targets) in transitions {
                    let mut letter_map = match outgoing.remove(letter) {
                        None => {hashset!{}},
                        Some(got) => {got}
                    };
                    for x_targ in x_targets {
                        let new_targ = cross_states_map.iter().position(|&r| r == (*x_targ,y)).unwrap();
                        letter_map.insert(new_targ);
                    }
                    outgoing.insert(*letter,letter_map);
                }
            }
        }
        for x in 0..self.transitions.len() {
            for (y_orig,transitions) in other.transitions.iter().enumerate() {
                let new_orig = cross_states_map.iter().position(|&r| r == (x,y_orig)).unwrap();
                let outgoing = new_transitions.get_mut(new_orig).unwrap();
                for (letter,y_targets) in transitions {
                    let mut letter_map = match outgoing.remove(letter) {
                        None => {hashset!{}},
                        Some(got) => {got}
                    };
                    for y_targ in y_targets {
                        let new_targ = cross_states_map.iter().position(|&r| r == (x,*y_targ)).unwrap();
                        letter_map.insert(new_targ);
                    }
                    outgoing.insert(*letter,letter_map);
                }
            }
        }
        // ***
        Self::from_raw(self.alphabet,new_initials,new_finals,new_transitions)
    }
}
