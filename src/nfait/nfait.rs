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

use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;
use crate::traits::error::AutError;
use crate::utils::{shift_hashmaps_of_hashset_usize, shift_hashset_of_usize, shift_vec_of_hashset_usize};


#[derive(Debug, Clone)]
pub struct AutNFAIT<Letter : AutLetter> {
    pub alphabet: HashSet<Letter>,
    pub initials: HashSet<usize>,
    pub finals: HashSet<usize>,
    pub transitions: Vec<HashMap<Letter, HashSet<usize>>>,
    pub epsilon_trans : Vec<HashSet<usize>>
}

impl<Letter : AutLetter> AutNFAIT<Letter> {

    pub fn shift_nfait(&mut self, by_num: usize) {
        shift_hashset_of_usize(&mut self.initials, by_num);
        shift_hashset_of_usize(&mut self.finals, by_num);
        shift_hashmaps_of_hashset_usize(&mut self.transitions, by_num);
        shift_vec_of_hashset_usize(&mut self.epsilon_trans, by_num);
    }

    pub fn new_void_object(alphabet: HashSet<Letter>) -> Self {
        AutNFAIT {
            alphabet,
            initials: HashSet::new(),
            finals: HashSet::new(),
            transitions: Vec::new(),
            epsilon_trans: Vec::new(),
        }
    }

    /// Returns a NFA that accepts only the empty word.
    pub fn new_empty_word(alphabet: HashSet<Letter>) -> Self {
        AutNFAIT {
            alphabet,
            initials: (0..=0).collect(),
            finals: (0..=0).collect(),
            transitions: vec![hashmap!{}],
            epsilon_trans: vec![hashset!{}]
        }
    }

    /// Returns an automaton built from the raw arguments.
    pub fn from_raw(
        alphabet: HashSet<Letter>,
        initials: HashSet<usize>,
        finals: HashSet<usize>,
        transitions: Vec<HashMap<Letter, HashSet<usize>>>,
        epsilon_trans : Vec<HashSet<usize>>,
    ) -> Result<Self, AutError<Letter>> {
        match AutNFA::<Letter>::from_raw(alphabet,initials,finals,transitions) {
            Err(e) => {
                Err(e)
            },
            Ok(as_nfa) => {
                let len = as_nfa.transitions.len();
                let eplen = epsilon_trans.len();
                if eplen > len {
                    return Err(AutError::InvalidEpsilonTrans(eplen,None,len));
                } else {
                    for (orig_stid,destinations) in epsilon_trans.iter().enumerate() {
                        if let Some(&destination) = destinations.iter().find(|&&x| x >= len) {
                            return Err(AutError::InvalidEpsilonTrans(orig_stid, Some(destination), len));
                        }
                    }
                }
                // ***
                Ok(AutNFAIT {
                    alphabet:as_nfa.alphabet,
                    initials:as_nfa.initials,
                    finals:as_nfa.finals,
                    transitions:as_nfa.transitions,
                    epsilon_trans
                })
            }
        }
    }
}



