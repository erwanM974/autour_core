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


use crate::{ere::term::TermERE, traits::letter::{get_new_alphabet_from_hiding, get_new_alphabet_from_substitution, AutAlphabetSubstitutable, AutLetter}};


impl<Letter: AutLetter> AutAlphabetSubstitutable<Letter> for TermERE<Letter> {


    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self {
        match self {
            TermERE::Empty => {TermERE::Empty},
            TermERE::Epsilon => {TermERE::Epsilon},
            TermERE::Literal(letter) => {
                TermERE::Literal(substitution(&letter))
            },
            TermERE::Union(sub_terms) => {
                let mut subbed_terms = btreeset!{};
                for sub_term in sub_terms {
                    subbed_terms.insert(sub_term.substitute_letters(remove_from_alphabet,substitution));
                }
                TermERE::Union(subbed_terms)
            },
            TermERE::Concat(sub_terms) => {
                let mut subbed_terms = vec!{};
                for sub_term in sub_terms {
                    subbed_terms.push(sub_term.substitute_letters(remove_from_alphabet,substitution));
                }
                TermERE::Concat(subbed_terms)
            },
            TermERE::Repeat(sub_term,min,max) => {
                TermERE::Repeat(Box::new(sub_term.substitute_letters(remove_from_alphabet,substitution)), min, max)
            },
            TermERE::Intersection(sub_terms) => {
                let mut subbed_terms = btreeset!{};
                for sub_term in sub_terms {
                    subbed_terms.insert(sub_term.substitute_letters(remove_from_alphabet,substitution));
                }
                TermERE::Intersection(subbed_terms)
            },
            TermERE::Negation(sub_term) => {
                TermERE::Negation(Box::new(sub_term.substitute_letters(remove_from_alphabet,substitution)))
            },
            TermERE::Wildcard => {
                if remove_from_alphabet {
                    TermERE::Wildcard
                } else {
                    unimplemented!()
                }
            },
        }
    }

    fn hide_letters(self,
                    remove_from_alphabet : bool,
                    should_hide : &dyn Fn(&Letter) -> bool) -> Self {
        match self {
            TermERE::Epsilon => {TermERE::Epsilon},
            TermERE::Empty => {TermERE::Empty},
            TermERE::Literal(letter) => {
                if should_hide(&letter) {
                    TermERE::Epsilon
                } else {
                    TermERE::Literal(letter)
                }
            },
            TermERE::Union(sub_terms) => {
                let mut new_term = TermERE::Empty;
                for sub_term in sub_terms {
                    new_term = new_term.unite(sub_term.hide_letters(remove_from_alphabet,should_hide));
                }
                new_term
            },
            TermERE::Concat(sub_terms) => {
                let mut new_term = TermERE::Epsilon;
                for sub_term in sub_terms {
                    new_term = new_term.concatenate(sub_term.hide_letters(remove_from_alphabet,should_hide));
                }
                new_term
            },
            TermERE::Repeat(sub_term,min,max) => {
                let hidden_sub_term = sub_term.hide_letters(remove_from_alphabet,should_hide);
                unimplemented!()
            },
            TermERE::Intersection(sub_terms) => {
                unimplemented!()
            },
            TermERE::Negation(sub_term) => {
                unimplemented!()
            },
            TermERE::Wildcard => {
                if remove_from_alphabet {
                    TermERE::Wildcard
                } else {
                    unimplemented!()
                }
            },
        }
    }
}


