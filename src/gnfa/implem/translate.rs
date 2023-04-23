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
use maplit::{hashset,hashmap};
use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;
use crate::dfa::dfa::AutDFA;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::access::AutAccessible;


impl<Letter : AutLetter> AutTranslatable<Letter> for AutGNFA<Letter> {
    fn to_dfa(&self) -> AutDFA<Letter> {
        self.to_nfait().to_dfa()
    }

    fn to_nfa(&self) -> AutNFA<Letter> {
        self.to_nfait().to_nfa()
    }

    fn to_nfait(&self) -> AutNFAIT<Letter> {
        let new_nfa_initials = hashset!{0};
        let new_nfa_finals = hashset!{1};
        let mut current_number_of_states = 2;
        let mut gnfa_to_nfait_states_map : HashMap<usize,usize> = hashmap!{self.start_state => 0, self.accept_state => 1};
        // ***
        let mut transitions_to_add = vec![];
        for ((orig_stid,targ_stid), term) in &self.transitions {
            if !term.is_empty() {
                transitions_to_add.push((orig_stid,targ_stid,term));
            }
        }
        // ***
        let mut new_nfa_transitions = vec![hashmap!{},hashmap!{}];
        let mut new_nfa_epstrans = vec![hashset!{},hashset!{}];
        // ***
        while !transitions_to_add.is_empty() {
            let mut pop_index : Option<usize> = None;
            for (idx,(orig,_,_)) in transitions_to_add.iter().enumerate() {
                if gnfa_to_nfait_states_map.contains_key(orig) {
                    pop_index = Some(idx);
                    break;
                }
            }
            match pop_index {
                None => {
                    break;
                },
                Some(idx) => {
                    let (orig_in_gnfa,targ_in_gnfa,term) = transitions_to_add.remove(idx);
                    //println!("{:?}", term);
                    let regexp = ExpBRE::from_raw(self.alphabet.clone(),term.clone()).unwrap();
                    // at first to DFA so as to simplify the NFAIT
                    let mut regexp_as_nfait = regexp.to_dfa().to_nfait();
                    regexp_as_nfait.shift_nfait(current_number_of_states);
                    //println!("{:?}", regexp_as_nfait);
                    current_number_of_states += regexp_as_nfait.transitions.len();
                    new_nfa_transitions.append(&mut regexp_as_nfait.transitions);
                    new_nfa_epstrans.append(&mut regexp_as_nfait.epsilon_trans);
                    // ***
                    let orig_in_nfait = gnfa_to_nfait_states_map.get(orig_in_gnfa).unwrap();
                    let targets_of_orig : &mut HashSet<usize> = new_nfa_epstrans.get_mut(*orig_in_nfait).unwrap();
                    targets_of_orig.extend(regexp_as_nfait.initials);
                    // ***
                    let targ_in_nfait = match gnfa_to_nfait_states_map.get(targ_in_gnfa) {
                        None => {
                            gnfa_to_nfait_states_map.insert(*targ_in_gnfa,current_number_of_states);
                            let targ = current_number_of_states;
                            current_number_of_states += 1;
                            new_nfa_transitions.push(hashmap!{});
                            new_nfa_epstrans.push(hashset!{});
                            targ
                        },
                        Some(targ) => {
                            *targ
                        }
                    };
                    for regexp_final_stid in regexp_as_nfait.finals {
                        let targets : &mut HashSet<usize> = new_nfa_epstrans.get_mut(regexp_final_stid).unwrap();
                        targets.insert(targ_in_nfait);
                    }
                    // ***
                }
            }
        }
        AutNFAIT::from_raw(self.alphabet.clone(),
                           new_nfa_initials,
                           new_nfa_finals,
                           new_nfa_transitions,
                           new_nfa_epstrans).unwrap()
    }

    fn to_gnfa(&self) -> AutGNFA<Letter> {
        self.clone()
    }

    fn to_bre(&self) -> ExpBRE<Letter> {
        println!("initial : {:?}", self);
        let mut new_gnfa = self.clone().trim();
        println!("trimmed then got : {:?}", new_gnfa);
        while new_gnfa.states_num > 2 {
            let mut rem_states : HashSet<usize> = (0..new_gnfa.states_num).collect();
            rem_states.remove(&new_gnfa.start_state);
            rem_states.remove(&new_gnfa.accept_state);
            let to_rip_id = rem_states.iter().next().unwrap();
            new_gnfa = new_gnfa.rip_state(*to_rip_id).unwrap();
            println!("removing state {:?} then got : {:?}", to_rip_id, new_gnfa);
            new_gnfa = new_gnfa.trim();
            println!("trimmed then got : {:?}", new_gnfa);
        }
        // ***
        let bre = new_gnfa.transitions.get(&(0,1)).unwrap();
        // ***
        ExpBRE::from_raw(self.alphabet.clone(),bre.clone()).unwrap()
    }
}