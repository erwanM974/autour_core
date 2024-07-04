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


use maplit::btreeset;

use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;

use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter, get_new_alphabet_from_hiding, get_new_alphabet_from_substitution};


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for TermBRE<Letter> {


    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        match self {
            TermBRE::Epsilon => {TermBRE::Epsilon},
            TermBRE::Empty => {TermBRE::Empty},
            TermBRE::Literal(letter) => {
                TermBRE::Literal(substitution(&letter))
            },
            TermBRE::Kleene(sub_term) => {
                TermBRE::Kleene(Box::new(sub_term.substitute_letters(remove_from_alphabet,substitution)))
            },
            TermBRE::Union(sub_terms) => {
                let mut subbed_terms = btreeset!{};
                for sub_term in sub_terms {
                    subbed_terms.insert(sub_term.substitute_letters(remove_from_alphabet,substitution));
                }
                TermBRE::Union(subbed_terms)
            },
            TermBRE::Concat(sub_terms) => {
                let mut subbed_terms = vec!{};
                for sub_term in sub_terms {
                    subbed_terms.push(sub_term.substitute_letters(remove_from_alphabet,substitution));
                }
                TermBRE::Concat(subbed_terms)
            }
        }
    }

    fn hide_letters(self,
                    remove_from_alphabet : bool,
                    should_hide : &dyn Fn(&Letter) -> bool) -> Self {
        match self {
            TermBRE::Epsilon => {TermBRE::Epsilon},
            TermBRE::Empty => {TermBRE::Empty},
            TermBRE::Literal(letter) => {
                if should_hide(&letter) {
                    TermBRE::Epsilon
                } else {
                    TermBRE::Literal(letter)
                }
            },
            TermBRE::Kleene(sub_term) => {
                TermBRE::Kleene(Box::new(sub_term.hide_letters(remove_from_alphabet,should_hide)))
            },
            TermBRE::Union(sub_terms) => {
                let mut new_term = TermBRE::Empty;
                for sub_term in sub_terms {
                    new_term = new_term.unite(sub_term.hide_letters(remove_from_alphabet,should_hide));
                }
                new_term
            },
            TermBRE::Concat(sub_terms) => {
                let mut new_term = TermBRE::Epsilon;
                for sub_term in sub_terms {
                    new_term = new_term.concatenate(sub_term.hide_letters(remove_from_alphabet,should_hide));
                }
                new_term
            }
        }
    }
}


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for ExpBRE<Letter> {

    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        ExpBRE::from_raw(get_new_alphabet_from_substitution(&self.alphabet,remove_from_alphabet,substitution),
                         self.term.substitute_letters(remove_from_alphabet,substitution)
        ).unwrap()
    }

    fn hide_letters(self,
                    remove_from_alphabet : bool,
                    should_hide : &dyn Fn(&Letter) -> bool) -> Self {
        ExpBRE::from_raw(get_new_alphabet_from_hiding(&self.alphabet,remove_from_alphabet,should_hide),
                         self.term.hide_letters(remove_from_alphabet,should_hide)
        ).unwrap()
    }

}