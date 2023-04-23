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
use crate::traits::error::AutError;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter};

impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutNFAIT<Letter> {

    fn substitute_alphabet(self,
                           new_alphabet: HashSet<Letter>,
                           substitution: &dyn Fn(Letter) -> Letter) -> Result<Self, AutError<Letter>> {
        let mut new_transitions = vec![];
        for transition in self.transitions {
            let mut new_transition : HashMap<Letter, HashSet<usize>> = hashmap!{};
            for (letter, targets) in transition {
                let substituted_letter : Letter = substitution(letter);
                if let Some(tars) = new_transition.get_mut(&substituted_letter) {
                    tars.extend(targets);
                } else {
                    new_transition.insert( substituted_letter, targets);
                }
            }
            new_transitions.push(new_transition);
        }
        AutNFAIT::from_raw(new_alphabet,self.initials,self.finals,new_transitions,self.epsilon_trans)
    }

    fn substitute_letters_within_alphabet(self,
                                          substitution : &dyn Fn(Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        let alphabet = self.alphabet.clone();
        self.substitute_alphabet(alphabet, substitution)
    }

    fn hide_letters(self, should_hide: &dyn Fn(Letter) -> bool) -> Self {
        let mut new_transitions = vec![];
        let mut new_epsilon_trans = self.epsilon_trans;
        for (orig_id,transition) in self.transitions.into_iter().enumerate() {
            let mut new_transition = hashmap!{};
            for (letter, targets) in transition {
                if should_hide(letter) {
                    new_epsilon_trans.get_mut(orig_id).unwrap().extend(targets);
                } else {
                    new_transition.insert( letter, targets);
                }
            }
            new_transitions.push(new_transition);
        }
        AutNFAIT::from_raw(self.alphabet,self.initials,self.finals,new_transitions,new_epsilon_trans).unwrap()
    }
}