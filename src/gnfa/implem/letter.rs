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
use maplit::hashmap;

use crate::bre::term::TermBRE;
use crate::gnfa::gnfa::AutGNFA;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter, get_new_alphabet_from_hiding, get_new_alphabet_from_substitution};

impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutGNFA<Letter> {

    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for ((orig,targ),term) in self.transitions {
            new_transitions.insert(
                (orig,targ),
                term.substitute_letters(remove_from_alphabet,substitution)
            );
        }
        AutGNFA::from_raw(
            get_new_alphabet_from_substitution(&self.alphabet,remove_from_alphabet,substitution),
            self.states_num,
            self.start_state,
            self.accept_state,
            new_transitions).unwrap()
    }

    fn hide_letters(self,
                    remove_from_alphabet : bool,
                    should_hide : &dyn Fn(&Letter) -> bool) -> Self {
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for ((orig,targ),term) in self.transitions {
            new_transitions.insert(
                (orig,targ),
                term.hide_letters(remove_from_alphabet,should_hide)
            );
        }
        AutGNFA::from_raw(
            get_new_alphabet_from_hiding(&self.alphabet,remove_from_alphabet,should_hide),
            self.states_num,
            self.start_state,
            self.accept_state,
            new_transitions).unwrap()
    }
}