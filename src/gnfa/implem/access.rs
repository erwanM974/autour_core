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
use std::iter::FromIterator;
use maplit::{hashmap,hashset};
use crate::bre::term::TermBRE;
use crate::gnfa::gnfa::AutGNFA;

use crate::nfait::nfait::AutNFAIT;
use crate::traits::access::AutAccessible;
use crate::traits::build::AutBuildable;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;




impl<Letter: AutLetter> AutAccessible for AutGNFA<Letter> {

    fn is_accessible(&self) -> bool {
        self.get_all_accessible_states().len() == self.states_num
    }

    fn get_all_accessible_states(&self) -> HashSet<usize> {
        let mut set_of_accessible_states: HashSet<usize> = hashset!{self.start_state};
        let mut stack: Vec<usize> = vec![self.start_state];
        while let Some(origin_state) = stack.pop() {
            for target_state in 0..self.states_num {
                if let Some(regexp) = self.transitions.get(&(origin_state,target_state)) {
                    if !regexp.is_empty() {
                        if !set_of_accessible_states.contains(&target_state) {
                            set_of_accessible_states.insert(target_state);
                            stack.push(target_state);
                        }
                    }
                }
            }
        }
        set_of_accessible_states
    }

    fn make_accessible(mut self) -> Self {
        unimplemented!("not implemented")
    }

    fn is_coaccessible(&self) -> bool {
        self.get_all_coaccessible_states().len() == self.states_num
    }

    fn get_all_coaccessible_states(&self) -> HashSet<usize> {
        let mut set_of_coaccessible_states: HashSet<usize> = hashset!{self.accept_state};
        let mut stack: Vec<usize> = vec![self.accept_state];
        while let Some(target_state) = stack.pop() {
            for origin_state in 0..self.states_num {
                if let Some(regexp) = self.transitions.get(&(origin_state,target_state)) {
                    if !regexp.is_empty() {
                        if !set_of_coaccessible_states.contains(&origin_state) {
                            set_of_coaccessible_states.insert(origin_state);
                            stack.push(origin_state);
                        }
                    }
                }
            }
        }
        set_of_coaccessible_states
    }

    fn make_coaccessible(self) -> Self {
        self.reverse().make_accessible().reverse()
    }

    fn is_trimmed(&self) -> bool {
        self.is_accessible() && self.is_coaccessible()
    }

    fn trim(self) -> Self {
        let mut new_start_state = 0;
        let mut new_accept_state = 1;
        let mut new_states_num = 2 ;
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        let mut states_map = hashmap!{self.start_state => 0, self.accept_state => 1};
        let mut stack = vec![self.start_state];
        while !stack.is_empty() {
            let orig_in_old = stack.pop().unwrap();
            let orig_in_new : usize = *states_map.get(&orig_in_old).unwrap();
            for targ_in_old in 0..self.states_num {
                if let Some(term) = self.transitions.get(&(orig_in_old,targ_in_old)) {
                    if !term.is_empty() {
                        let targ_in_new = match states_map.get(&targ_in_old) {
                            None => {
                                let targ_in_new = new_states_num;
                                new_states_num += 1;
                                states_map.insert(targ_in_old,targ_in_new);
                                stack.push(targ_in_old);
                                targ_in_new
                            },
                            Some(targ_in_new) => {
                                *targ_in_new
                            }
                        };
                        new_transitions.insert((orig_in_new,targ_in_new),term.clone());
                    }
                }
            }
        }
        Self::from_raw(self.alphabet,new_states_num,new_start_state,new_accept_state,new_transitions).unwrap()
    }
}