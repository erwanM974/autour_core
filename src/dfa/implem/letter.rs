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



use crate::dfa::dfa::AutDFA;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter};
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for AutDFA<Letter> {

    fn substitute_letters(self,
                             remove_from_alphabet : bool,
                             substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        // can't do it directly as substitution may cause non-determinism
        self.to_nfa().substitute_letters(remove_from_alphabet, substitution).to_dfa()
    }

    fn hide_letters(self, hide_alphabet : bool, should_hide: &dyn Fn(&Letter) -> bool) -> Self {
        self.to_nfait().hide_letters(hide_alphabet,should_hide).to_dfa()
    }
}