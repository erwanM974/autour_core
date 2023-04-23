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
use maplit::hashmap;

use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;
use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;

use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::translate::AutTranslatable;


impl<Letter : AutLetter> AutTranslatable<Letter> for ExpBRE<Letter> {
    fn to_dfa(&self) -> AutDFA<Letter> {
        self.to_nfa().to_dfa()
    }

    fn to_nfa(&self) -> AutNFA<Letter> {
        term_bre_to_nfa(&self.term,&self.alphabet)
    }

    fn to_nfait(&self) -> AutNFAIT<Letter> {
        self.to_nfa().to_nfait()
    }

    fn to_gnfa(&self) -> AutGNFA<Letter> {
        AutGNFA::from_raw(self.alphabet.clone(),
                          2,
                          0,
                          1,
                          hashmap! {(0,1) => self.term.clone()}).unwrap()
    }

    fn to_bre(&self) -> ExpBRE<Letter> {
        self.clone()
    }
}


fn term_bre_to_nfa<Letter:AutLetter>(operation : &TermBRE<Letter>, alphabet: &HashSet<Letter>) -> AutNFA<Letter> {
    match operation {
        TermBRE::Union(sub_terms) => {
            sub_terms.iter()
                .fold(AutNFA::new_accepts_nothing(alphabet.clone()),
                      |acc, x|
                          acc.unite(term_bre_to_nfa(x,alphabet)).unwrap()
            )
        },
        TermBRE::Concat(sub_terms) => {
            sub_terms.iter()
                .fold(AutNFA::new_empty_word(alphabet.clone()),
                      |acc, x|
                          acc.concatenate(term_bre_to_nfa(x,alphabet)).unwrap()
                )
        },
        TermBRE::Kleene(sub_term) => {
            term_bre_to_nfa(sub_term,alphabet).kleene()
        },
        TermBRE::Literal(letter) => AutNFA::new_matching(alphabet.clone(), &[*letter]),
        TermBRE::Epsilon => AutNFA::new_empty_word(alphabet.clone()),
        TermBRE::Empty => AutNFA::new_accepts_nothing(alphabet.clone())
    }
}

