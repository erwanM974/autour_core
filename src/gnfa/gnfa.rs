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

use crate::traits::letter::AutLetter;
use crate::traits::error::AutError;


#[derive(Debug, Clone)]
pub struct AutGNFA<Letter : AutLetter> {
    pub alphabet: HashSet<Letter>,
    pub states_num : usize,
    pub start_state : usize,
    pub accept_state : usize,
    pub transitions: HashMap<(usize,usize), TermBRE<Letter>>
}


impl<Letter: AutLetter> AutGNFA<Letter> {

    pub fn rip_state(&self, to_rip_id : usize) -> Result<Self,AutError<Letter>> {
        if self.states_num <= 2 {
            return Err(AutError::Other("cannot rip any more states in GNFA".to_string()));
        }
        if to_rip_id == self.start_state {
            return Err(AutError::Other("cannot rip start state from GNFA".to_string()));
        }
        if to_rip_id == self.accept_state {
            return Err(AutError::Other("cannot rip accept state from GNFA".to_string()));
        }
        // ***
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        // ***
        let mut incoming : HashSet<(usize,TermBRE<Letter>)> = hashset!{};
        let mut outgoing : HashSet<(usize,TermBRE<Letter>)> = hashset!{};
        let mut on_self : Option<TermBRE<Letter>> = None;
        for ((orig,targ),term) in &self.transitions {
            if orig == targ {
                if *orig == to_rip_id {
                    if !term.is_empty() {
                        on_self = Some(term.clone());
                    }
                }
            } else {
                if *targ == to_rip_id {
                    incoming.insert((*orig,term.clone()));
                }
                if *orig == to_rip_id {
                    outgoing.insert((*targ,term.clone()));
                }
            }
        }
        // ***
        let mut new_transitions : HashMap<(usize,usize), TermBRE<Letter>> = hashmap!{};
        for ((orig,targ),term) in &self.transitions {
            if *orig != to_rip_id && *targ != to_rip_id {
                new_transitions.insert((*orig,*targ), term.clone());
            }
        }
        // ***
        for (orig_id,orig_term) in &incoming {
            for (targ_id,targ_tem) in &outgoing {
                let middle : TermBRE<Letter> =
                match &on_self {
                    None => {
                        TermBRE::Epsilon
                    },
                    Some(t) => {
                        t.clone()
                    }
                };
                //let new_tr_term = TermBRE::Concat(vec![orig_term.clone(),middle,targ_tem.clone()]);
                let mut new_tr_term = orig_term.clone().concatenate(middle);
                new_tr_term = new_tr_term.concatenate(targ_tem.clone());
                let old_tr_term = self.transitions.get(&(*orig_id,*targ_id)).unwrap();
                new_tr_term = new_tr_term.unite(old_tr_term.clone());
                new_transitions.insert((*orig_id,*targ_id), new_tr_term);
            }
        }
        // ***
        Self::from_raw(self.alphabet.clone(),
                       self.states_num,
                       self.start_state,
                       self.accept_state,
                       new_transitions)
    }

    pub fn from_raw(
        alphabet: HashSet<Letter>,
        states_num: usize,
        start_state : usize,
        accept_state : usize,
        raw_transitions: HashMap<(usize,usize), TermBRE<Letter>>,
    ) -> Result<Self, AutError<Letter>> {
        if start_state >= states_num {
            return Err(AutError::InvalidInitial(start_state, states_num));
        }
        if accept_state >= states_num {
            return Err(AutError::InvalidFinal(accept_state, states_num));
        }
        let mut transitions = hashmap!{};
        for i in 0..states_num {
            for j in 0..states_num {
                match raw_transitions.get(&(i,j)) {
                    None => {
                        if i != accept_state && j != start_state {
                            if i != j {
                                transitions.insert( (i,j),
                                                    TermBRE::Empty);
                            } else {
                                transitions.insert( (i,j),
                                                    TermBRE::Epsilon);
                            }
                        }
                    },
                    Some( op) => {
                        if i == accept_state {
                            return Err(AutError::Other(format!("should not have outgoing transitions from GNFA accept state {:}", i)));
                        }
                        if j == start_state {
                            return Err(AutError::Other(format!("should not have incoming transitions towards GNFA start state {:}", j)));
                        }
                        match ExpBRE::from_raw(alphabet.clone(),op.clone()) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( got) => {
                                transitions.insert( (i,j), got.term);
                            }
                        }
                    }
                }
            }
        }
        // ***
        Ok(Self{alphabet,states_num,start_state,accept_state,transitions})
    }
}



