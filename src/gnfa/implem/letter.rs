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

use crate::bre::term::TermBRE;
use crate::gnfa::gnfa::AutGNFA;
use crate::traits::error::AutError;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter};

impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutGNFA<Letter> {

    fn substitute_alphabet(self,
                           new_alphabet: HashSet<Letter>,
                           substitution: &dyn Fn(&Letter) -> Letter) -> Result<Self, AutError<Letter>> {
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for ((orig,targ),term) in self.transitions {
            match term.substitute_letters_within_alphabet(substitution) {
                Err(e) => {
                    return Err(e);
                },
                Ok(new_term) => {
                    new_transitions.insert((orig,targ),new_term);
                }
            }
        }
        AutGNFA::from_raw(self.alphabet,
                          self.states_num,
                          self.start_state,
                          self.accept_state,
                          new_transitions)
    }

    fn substitute_letters_within_alphabet(self,
                                          substitution : &dyn Fn(&Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        let alphabet = self.alphabet.clone();
        self.substitute_alphabet(alphabet, substitution)
    }

    fn hide_letters(self,hide_alphabet : bool, should_hide: &dyn Fn(&Letter) -> bool) -> Self {
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for ((orig,targ),term) in self.transitions {
            new_transitions.insert((orig,targ),term.hide_letters(hide_alphabet,should_hide));
        }
        AutGNFA::from_raw(self.alphabet,
                          self.states_num,
                          self.start_state,
                          self.accept_state,
                          new_transitions).unwrap()
    }
}