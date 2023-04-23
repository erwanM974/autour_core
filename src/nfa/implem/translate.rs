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

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use maplit::hashset;
use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;
use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;

impl<Letter : AutLetter> AutTranslatable<Letter> for AutNFA<Letter> {
    fn to_dfa(&self) -> AutDFA<Letter> {
        // maps sets of states from the NFA to a corresponding new state in the DFA
        let mut states_map: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        let mut stack = VecDeque::new();
        // ***
        let mut new_dfa_finals = hashset!{};
        let mut new_dfa_transitions = vec![];
        // ***
        // All the initial states of the NFA are assigned to state "0" of the DFA
        let initial: BTreeSet<usize> = self.initials.iter().copied().collect();
        states_map.insert(initial.clone(), 0);
        // don't forget to add a hashmap for outgoing transitions from that initial state 0
        new_dfa_transitions.push(HashMap::new());
        stack.push_back(initial);
        // If any of the initial states of the NFA is also a final state then state "0" of the DFA should also be final
        if self.initials.iter().any(|x| self.finals.contains(x)) {
            new_dfa_finals.insert(0);
        }
        // ***
        while let Some(states_ids_in_nfa) = stack.pop_front() {
            // get ID of the state in the DFA that corresponds to the set of states from the NFA
            let state_id_in_dfa = *states_map.get(&states_ids_in_nfa).unwrap();
            for letter in &self.alphabet {
                let mut targets_in_nfa : BTreeSet<usize> = BTreeSet::new();
                for nfa_state in &states_ids_in_nfa {
                    if let Some(transitions) = self.transitions[*nfa_state].get(&letter) {
                        for t in transitions {
                            targets_in_nfa.insert(*t);
                        }
                    }
                }
                // ***
                if !targets_in_nfa.is_empty() {
                    if !states_map.contains_key(&targets_in_nfa) {
                        let new_dfa_state_id = new_dfa_transitions.len();
                        states_map.insert(targets_in_nfa.clone(), new_dfa_state_id);
                        if targets_in_nfa.iter().any(|x| self.finals.contains(x)) {
                            new_dfa_finals.insert(new_dfa_state_id);
                        }
                        stack.push_back(targets_in_nfa.clone());
                        new_dfa_transitions.push(HashMap::new());
                    }
                    new_dfa_transitions[state_id_in_dfa].insert(*letter, *states_map.get(&targets_in_nfa).unwrap());
                }
            }
        }
        // ***
        AutDFA::from_raw(self.alphabet.clone(),0,new_dfa_finals,new_dfa_transitions).unwrap()
    }

    fn to_nfa(&self) -> AutNFA<Letter> {
        self.clone()
    }

    fn to_nfait(&self) -> AutNFAIT<Letter> {
        let len = self.transitions.len();
        AutNFAIT::from_raw(self.alphabet.clone(),
                               self.initials.clone(),
                               self.finals.clone(),
                               self.transitions.clone(),
                               vec![hashset!{};len]).unwrap()
    }

    fn to_gnfa(&self) -> AutGNFA<Letter> {
        self.to_nfait().to_gnfa()
    }

    fn to_bre(&self) -> ExpBRE<Letter> {
        self.to_gnfa().to_bre()
    }
}

