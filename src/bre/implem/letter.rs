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



use std::collections::HashSet;
use maplit::btreeset;

use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;

use crate::traits::error::AutError;
use crate::traits::letter::{AutAlphabetSubstitutable, AutLetter};


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for TermBRE<Letter> {

    fn substitute_alphabet(self,
                           _new_alphabet: HashSet<Letter>,
                           _substitution: &dyn Fn(&Letter) -> Letter) -> Result<Self, AutError<Letter>> {
        panic!("shouldn't be called");
    }

    fn substitute_letters_within_alphabet(self,
                                          substitution: &dyn Fn(&Letter) -> Letter) -> Result<Self, AutError<Letter>> {
        match self {
            TermBRE::Epsilon => {Ok(TermBRE::Epsilon)},
            TermBRE::Empty => {Ok(TermBRE::Empty)},
            TermBRE::Literal(letter) => {
                Ok(TermBRE::Literal(substitution(&letter)))
            },
            TermBRE::Kleene(sub_term) => {
                match sub_term.substitute_letters_within_alphabet(substitution) {
                    Err(e) => {Err(e)},
                    Ok(subbed) => {
                        Ok(TermBRE::Kleene(Box::new(subbed)))
                    }
                }
            },
            TermBRE::Union(sub_terms) => {
                let mut subbed_terms = btreeset!{};
                for sub_term in sub_terms {
                    match sub_term.substitute_letters_within_alphabet(substitution) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok(subbed) => {
                            subbed_terms.insert(subbed);
                        }
                    }
                }
                Ok(TermBRE::Union(subbed_terms))
            },
            TermBRE::Concat(sub_terms) => {
                let mut subbed_terms = vec!{};
                for sub_term in sub_terms {
                    match sub_term.substitute_letters_within_alphabet(substitution) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok(subbed) => {
                            subbed_terms.push(subbed)
                        }
                    }
                }
                Ok(TermBRE::Concat(subbed_terms))
            }
        }
    }

    fn hide_letters(self, _hide_alphabet : bool, should_hide: &dyn Fn(&Letter) -> bool) -> Self {
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
                TermBRE::Kleene(Box::new(sub_term.hide_letters(_hide_alphabet,should_hide)))
            },
            TermBRE::Union(sub_terms) => {
                let mut new_term = TermBRE::Empty;
                for sub_term in sub_terms {
                    new_term = new_term.unite(sub_term.hide_letters(_hide_alphabet,should_hide));
                }
                new_term
            },
            TermBRE::Concat(sub_terms) => {
                let mut new_term = TermBRE::Epsilon;
                for sub_term in sub_terms {
                    new_term = new_term.concatenate(sub_term.hide_letters(_hide_alphabet,should_hide));
                }
                new_term
            }
        }
    }
}


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for ExpBRE<Letter> {

    fn substitute_alphabet(self,
                           new_alphabet: HashSet<Letter>,
                           substitution: &dyn Fn(&Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        match self.term.substitute_letters_within_alphabet(substitution) {
            Err(e) => {Err(e)},
            Ok(new_term) => {
                ExpBRE::from_raw(new_alphabet,new_term)
            }
        }
    }

    fn substitute_letters_within_alphabet(self,
                                          substitution: &dyn Fn(&Letter) -> Letter) -> Result<Self,AutError<Letter>> {
        let alphabet = self.alphabet.clone();
        self.substitute_alphabet(alphabet,substitution)
    }

    fn hide_letters(self, hide_alphabet : bool, should_hide: &dyn Fn(&Letter) -> bool) -> Self {
        let new_alphabet : HashSet<Letter> = if hide_alphabet {
            self.alphabet.into_iter().filter(|l| !should_hide(l)).collect()
        }  else {
            self.alphabet
        };
        ExpBRE::from_raw(new_alphabet,self.term.hide_letters(hide_alphabet,should_hide)).unwrap()
    }

}