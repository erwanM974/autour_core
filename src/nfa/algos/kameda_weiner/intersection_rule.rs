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





use std::collections::{HashMap, HashSet};
use maplit::{hashmap, hashset};
use crate::dfa::dfa::AutDFA;
use crate::nfa::algos::kameda_weiner::states_map::KwStatesMap;
use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;


/// Here we assume the subset assignment associated with a states map
/// i.e. each state of the DFA (corresponding to elements at the title column of each individual rows)
/// is assigned to the set of states appearing within cells of the corresponding rows
fn get_subset_assignment_from_states_map(states_map : &KwStatesMap) -> HashMap<usize,HashSet<usize>> {
    let mut map = hashmap! {};
    for row_id in 0..states_map.rows_map_to_det_states.len() {
        let row_in_matrix = states_map.matrix_map_to_nfa_states.get(row_id).unwrap();
        let mut all_states : HashSet<usize>  = hashset!{};
        for nfa_states_in_cell in row_in_matrix.iter().flatten() {
            all_states.extend(nfa_states_in_cell.iter());
        }
        // ***
        let row_dfa_states = states_map.rows_map_to_det_states.get(row_id).unwrap();
        for dfa_st_id in row_dfa_states {
            map.insert( *dfa_st_id, all_states.clone());
        }
    }
    // ***
    map
}

/// the subset assignment f associates 1 state of the DFA to a set of states of the NFA
/// the reverse associates 1 state of the NFA to a set of states of the DFA
fn reverse_subset_assignment(f : &HashMap<usize,HashSet<usize>>, nfa_st_id : usize) -> HashSet<usize> {
    let mut dfa_states = hashset! {};
    for (key,val) in f {
        if val.contains(&nfa_st_id) {
            dfa_states.insert( *key);
        }
    }
    // ***
    dfa_states
}

/// Intersection rule
pub fn convert_states_map_to_nfa<Letter : AutLetter>(states_map : &KwStatesMap,
                                                dfa : &AutDFA<Letter>,
                                                     target_num_states : usize)
            -> AutNFA<Letter> {

    let f = get_subset_assignment_from_states_map(states_map);
    // ***
    let new_nfa_initials = f.get(&dfa.initial).unwrap().clone();

    // ***
    let mut new_nfa_finals = hashset!{};
    let mut new_nfa_transitions = vec![];

    // ***
    for nfa_st_id in 0..target_num_states {
        let rev_f = reverse_subset_assignment(&f,nfa_st_id);
        if rev_f.iter().all(|dfa_state| dfa.finals.contains(dfa_state)) {
            new_nfa_finals.insert(nfa_st_id);
        }
        // ***
        let mut outgoing = hashmap!{};
        for targ_nfa_st_id in 0..target_num_states {
            let targ_rev_f = reverse_subset_assignment(&f,targ_nfa_st_id);
            for letter in &dfa.alphabet {
                let mut all_transitions_in_dfa = true;
                for dfa_orig_id in &rev_f {
                    let dfa_outgoing = dfa.transitions.get(*dfa_orig_id).unwrap();
                    if let Some(dfa_targ_id) = dfa_outgoing.get(letter) {
                        if targ_rev_f.contains(dfa_targ_id) {
                            continue;
                        }
                    }
                    all_transitions_in_dfa = false;
                    break;
                }
                if all_transitions_in_dfa {
                    match outgoing.get_mut(letter) {
                        None => {
                            outgoing.insert(*letter,hashset!{targ_nfa_st_id});
                        },
                        Some(already) => {
                            already.insert(targ_nfa_st_id);
                        }
                    }
                }
            }
        }

        new_nfa_transitions.push(outgoing);
    }
    // ***
    AutNFA::from_raw(dfa.alphabet.clone(),new_nfa_initials,new_nfa_finals,new_nfa_transitions).unwrap()
}




