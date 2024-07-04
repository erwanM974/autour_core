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

use std::collections::{BTreeSet, HashMap, VecDeque};
use maplit::{hashmap, hashset};

use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;
use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;

impl<Letter : AutLetter> AutTranslatable<Letter> for AutNFAIT<Letter> {

    fn to_dfa(&self) -> AutDFA<Letter> {
        // maps sets of states from the NFAIT to a corresponding new state in the DFA
        let mut states_map: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        // ***
        let mut new_dfa_finals = hashset!{};
        let mut new_dfa_transitions = vec![];
        // ***
        // All the initial states of the NFAIT (and their epsilon closure) are assigned to state "0" of the DFA
        let initial: BTreeSet<usize> = self.get_epsilon_closure(&self.initials).into_iter().collect();
        states_map.insert(initial.clone(), 0);
        // don't forget to add a hashmap for outgoing transitions from that initial state 0
        new_dfa_transitions.push(HashMap::new());
        // If any of the initial states of the NFAIT is also a final state then state "0" of the DFA should also be final
        if initial.iter().any(|x| self.finals.contains(x)) {
            new_dfa_finals.insert(0);
        }
        // ***
        // treat reachable states progressively
        let mut stack = VecDeque::new();
        stack.push_back(initial);
        // ***
        while let Some(states_ids_in_nfait) = stack.pop_front() {
            // get ID of the state in the DFA that corresponds to the set of states from the NFAIT
            let state_id_in_dfa = *states_map.get(&states_ids_in_nfait).unwrap();
            for letter in &self.alphabet {
                let mut new_dfa_states : BTreeSet<usize> = BTreeSet::new();
                for nfait_state in &states_ids_in_nfait {
                    if let Some(letter_transitions_targets) = self.transitions[*nfait_state].get(letter) {
                        new_dfa_states.extend(self.get_epsilon_closure(letter_transitions_targets));
                    }
                }
                // ***
                if !new_dfa_states.is_empty() {
                    if !states_map.contains_key(&new_dfa_states) {
                        let new_dfa_state_id = new_dfa_transitions.len();
                        states_map.insert(new_dfa_states.clone(), new_dfa_state_id);
                        if new_dfa_states.iter().any(|x| self.finals.contains(x)) {
                            new_dfa_finals.insert(new_dfa_state_id);
                        }
                        stack.push_back(new_dfa_states.clone());
                        new_dfa_transitions.push(HashMap::new());
                    }
                    new_dfa_transitions[state_id_in_dfa].insert(*letter, *states_map.get(&new_dfa_states).unwrap());
                }
            }
        }
        // ***
        AutDFA::from_raw(self.alphabet.clone(),0,new_dfa_finals,new_dfa_transitions).unwrap()
    }

    fn to_nfa(&self) -> AutNFA<Letter> {
        let mut has_no_epsilon_transitions = true;
        for (orig,epstrans) in self.epsilon_trans.iter().enumerate() {
            match epstrans.len() {
                0 => {},
                1 => {
                    let targ = epstrans.iter().next().unwrap();
                    if orig != *targ {
                        has_no_epsilon_transitions = false;
                        break;
                    }
                },
                _ => {
                    has_no_epsilon_transitions = false;
                    break;
                }
            }
        }
        if has_no_epsilon_transitions {
            AutNFA::from_raw(self.alphabet.clone(),
                             self.initials.clone(),
                             self.finals.clone(),
                             self.transitions.clone()).unwrap()
        } else {
            self.to_dfa().to_nfa()
        }
    }

    fn to_nfait(&self) -> AutNFAIT<Letter> {
        self.clone()
    }

    fn to_gnfa(&self) -> AutGNFA<Letter> {
        let states_num = self.transitions.len() + 2;
        let start_state = self.transitions.len();
        let accept_state = self.transitions.len() + 1;
        let mut raw_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for start in &self.initials {
            raw_transitions.insert((start_state,*start), TermBRE::Epsilon);
        }
        for end in &self.finals {
            raw_transitions.insert((*end,accept_state), TermBRE::Epsilon);
        }
        for (origin,transitions) in self.transitions.iter().enumerate() {
            for (letter,targets) in transitions.iter() {
                for target in targets {
                    if let Some(got) = raw_transitions.get_mut(&(origin,*target)) {
                        *got = got.clone().unite(TermBRE::Literal(*letter));
                    } else {
                        raw_transitions.insert((origin,*target),TermBRE::Literal(*letter) );
                    }
                }
            }
        }
        for (origin,targets) in self.epsilon_trans.iter().enumerate() {
            for target in targets {
                if let Some(got) = raw_transitions.get_mut(&(origin,*target)) {
                    *got = got.clone().unite(TermBRE::Epsilon);
                } else {
                    raw_transitions.insert((origin,*target),TermBRE::Epsilon );
                }
            }
        }
        AutGNFA::from_raw(self.alphabet.clone(),
                          states_num,
                          start_state,
                          accept_state,
                          raw_transitions).unwrap()
    }

    fn to_bre(&self) -> ExpBRE<Letter> {
        self.to_nfa().to_bre()
    }

}

