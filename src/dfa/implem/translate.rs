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
use maplit::hashset;
use crate::bre::bre::ExpBRE;

use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;


impl<Letter : AutLetter> AutTranslatable<Letter> for AutDFA<Letter> {

    fn to_dfa(&self) -> AutDFA<Letter> {
        self.clone()
    }

    fn to_nfa(&self) -> AutNFA<Letter> {
        let mut initials = HashSet::new();
        initials.insert(self.initial);
        let mut transitions = Vec::new();
        for dfa_transitions_map in &self.transitions {
            transitions.push(dfa_transitions_map.iter().map(|(letter, target_stid)| (*letter, hashset!{*target_stid})).collect());
        }
        return AutNFA::from_raw(self.alphabet.clone(), initials,self.finals.clone(), transitions).unwrap();
    }

    fn to_nfait(&self) -> AutNFAIT<Letter> {
        self.to_nfa().to_nfait()
    }

    fn to_gnfa(&self) -> AutGNFA<Letter> {
        self.to_nfait().to_gnfa()
    }

    fn to_bre(&self) -> ExpBRE<Letter> {
        self.to_nfa().to_bre()
    }

}