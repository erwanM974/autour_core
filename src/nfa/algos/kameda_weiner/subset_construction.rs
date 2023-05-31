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

use crate::dfa::dfa::AutDFA;
use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;

/// determinizes the input NFA and returns it with a map mapping states of the DFA to set of states from the original NFA
pub fn determinize_nfa_and_get_states_map<Letter : AutLetter>(nfa : &AutNFA<Letter>) -> (AutDFA<Letter>,HashMap<usize,BTreeSet<usize>>){
    // maps sets of states from the NFA to a corresponding new state in the DFA
    let mut states_map: HashMap<BTreeSet<usize>, usize> = HashMap::new();
    let mut stack = VecDeque::new();
    // ***
    let mut new_dfa_finals = hashset!{};
    let mut new_dfa_transitions = vec![];
    // ***
    // All the initial states of the NFA are assigned to state "0" of the DFA
    let initial: BTreeSet<usize> = nfa.initials.iter().copied().collect();
    states_map.insert(initial.clone(), 0);
    // don't forget to add a hashmap for outgoing transitions from that initial state 0
    new_dfa_transitions.push(HashMap::new());
    stack.push_back(initial);
    // If any of the initial states of the NFA is also a final state then state "0" of the DFA should also be final
    if nfa.initials.iter().any(|x| nfa.finals.contains(x)) {
        new_dfa_finals.insert(0);
    }
    // ***
    while let Some(states_ids_in_nfa) = stack.pop_front() {
        // get ID of the state in the DFA that corresponds to the set of states from the NFA
        let state_id_in_dfa = *states_map.get(&states_ids_in_nfa).unwrap();
        for letter in &nfa.alphabet {
            let mut targets_in_nfa : BTreeSet<usize> = BTreeSet::new();
            for nfa_state in &states_ids_in_nfa {
                if let Some(transitions) = nfa.transitions[*nfa_state].get(letter) {
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
                    if targets_in_nfa.iter().any(|x| nfa.finals.contains(x)) {
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
    let dfa = AutDFA::from_raw(nfa.alphabet.clone(),0,new_dfa_finals,new_dfa_transitions).unwrap();
    let mut reverse_map : HashMap<usize,BTreeSet<usize>> = hashmap!{};
    for (nfa_states,dfa_state) in states_map {
        reverse_map.insert(dfa_state,nfa_states);
    }
    // ***
    (dfa,reverse_map)
}



