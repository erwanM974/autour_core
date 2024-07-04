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
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter, get_new_alphabet_from_substitution};
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutNFA<Letter> {

    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        let mut new_transitions = vec![];
        for transition in self.transitions {
            let mut new_transition : HashMap<Letter, HashSet<usize>> = hashmap!{};
            for (letter, targets) in transition {
                let substituted_letter : Letter = substitution(&letter);
                if let Some(tars) = new_transition.get_mut(&substituted_letter) {
                    tars.extend(targets);
                } else {
                    new_transition.insert( substituted_letter, targets);
                }
            }
            new_transitions.push(new_transition);
        }
        AutNFA::from_raw(
            get_new_alphabet_from_substitution(&self.alphabet,remove_from_alphabet,substitution),
            self.initials,
            self.finals,
            new_transitions).unwrap()
    }

    fn hide_letters(self, remove_from_alphabet : bool, should_hide: &dyn Fn(&Letter) -> bool) -> Self {
        self.to_nfait().hide_letters(remove_from_alphabet,should_hide).to_nfa()
    }
}