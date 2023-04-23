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

use crate::nfa::nfa::AutNFA;
use crate::traits::error::AutError;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter};
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutNFA<Letter> {

    fn substitute_alphabet(self,
                           new_alphabet: HashSet<Letter>,
                           substitution: &dyn Fn(Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        let mut new_transitions = vec![];
        for transition in self.transitions {
            let mut new_transition : HashMap<Letter, HashSet<usize>> = hashmap!{};
            for (letter, targets) in transition {
                let substituted_letter : Letter = substitution(letter);
                // checking is the substituted_letter is in the new alphabet is done later when building the new NFA
                if let Some(tars) = new_transition.get_mut(&substituted_letter) {
                    tars.extend(targets);
                } else {
                    new_transition.insert( substituted_letter, targets);
                }
            }
            new_transitions.push(new_transition);
        }
        AutNFA::from_raw(new_alphabet,self.initials,self.finals,new_transitions)
    }

    fn substitute_letters_within_alphabet(self,
                                          substitution: &dyn Fn(Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        let mut new_transitions = vec![];
        for transition in self.transitions {
            let mut new_transition : HashMap<Letter, HashSet<usize>> = hashmap!{};
            for (letter, targets) in transition {
                let substituted_letter : Letter = substitution(letter);
                // checking is the substituted_letter is in the alphabet is done later when building the new NFA
                if let Some(tars) = new_transition.get_mut(&substituted_letter) {
                    tars.extend(targets);
                } else {
                    new_transition.insert( substituted_letter, targets);
                }
            }
            new_transitions.push(new_transition);
        }
        AutNFA::from_raw(self.alphabet,self.initials,self.finals,new_transitions)
    }

    fn hide_letters(self, should_hide: &dyn Fn(Letter) -> bool) -> Self {
        self.to_nfait().hide_letters(should_hide).to_nfa()
    }
}