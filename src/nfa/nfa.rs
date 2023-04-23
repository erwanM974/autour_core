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

use crate::traits::letter::AutLetter;
use crate::traits::error::AutError;
use crate::utils::{shift_hashmaps_of_hashset_usize, shift_hashset_of_usize};

#[derive(Debug, Clone)]
pub struct AutNFA<Letter : AutLetter> {
    pub alphabet: HashSet<Letter>,
    pub initials: HashSet<usize>,
    pub finals: HashSet<usize>,
    pub transitions: Vec<HashMap<Letter, HashSet<usize>>>,
}

impl<Letter : AutLetter> AutNFA<Letter> {

    pub fn shift_nfa(&mut self, by_num: usize) {
        shift_hashset_of_usize(&mut self.initials, by_num);
        shift_hashset_of_usize(&mut self.finals, by_num);
        shift_hashmaps_of_hashset_usize(&mut self.transitions, by_num);
    }

    pub fn new_void_object(alphabet: HashSet<Letter>) -> Self {
        AutNFA {
            alphabet,
            initials: HashSet::new(),
            finals: HashSet::new(),
            transitions: Vec::new(),
        }
    }

    /// Returns a universal NFA with one state able to express all letters
    pub fn new_universal(alphabet: HashSet<Letter>) -> Self {
        AutNFA {
            transitions: vec![alphabet.iter().map(|v| (*v, hashset!{0})).collect()],
            alphabet,
            initials: (0..=0).collect(),
            finals: (0..=0).collect(),
        }
    }

    /// Returns a NFA that accepts all words of a given length.
    pub fn new_length(alphabet: HashSet<Letter>, len: usize) -> Self {
        let mut transitions = vec![hashmap!{};len];
        for (origin_state_id, transitions_map) in transitions.iter_mut().enumerate() {
            for letter in &alphabet {
                transitions_map.insert(*letter, hashset!{origin_state_id + 1});
            }
        }
        // add the final state with no outgoing transition
        transitions.push(hashmap!{});
        // ***
        AutNFA {
            alphabet,
            initials: (0..=0).collect(),
            finals: (len..=len).collect(),
            transitions,
        }
    }

    /// Returns a NFA that accepts only the empty word.
    pub fn new_empty_word(alphabet: HashSet<Letter>) -> Self {
        AutNFA::new_length(alphabet,0)
    }

    /// Returns a NFA that accepts no word
    pub fn new_accepts_nothing(alphabet: HashSet<Letter>) -> Self {
        AutNFA {
            alphabet,
            initials: hashset!{0},
            finals: hashset!{},
            transitions: vec![hashmap!{}],
        }
    }

    /// Returns a NFA that accepts only the given word.
    pub fn new_matching(alphabet: HashSet<Letter>, word: &[Letter]) -> Self {
        let l = word.len();
        let mut nfa = AutNFA {
            alphabet,
            initials: (0..=0).collect(),
            finals: (l..=l).collect(),
            transitions: vec![hashmap!{};l+1],
        };
        // ***
        for (letter_index, letter) in word.iter().enumerate() {
            nfa.transitions[letter_index].insert(*letter, hashset!{letter_index + 1});
        }
        // ***
        nfa
    }

    /// Returns an automaton built from the raw arguments.
    pub fn from_raw(
        alphabet: HashSet<Letter>,
        initials: HashSet<usize>,
        finals: HashSet<usize>,
        transitions: Vec<HashMap<Letter, HashSet<usize>>>,
    ) -> Result<Self, AutError<Letter>> {
        let len = transitions.len();
        // ***
        if let Some(state) = initials.iter().find(|&&state| state >= len) {
            return Err(AutError::InvalidInitial(*state,len));
        }
        // ***
        if let Some(state) = finals.iter().find(|&&state| state >= len) {
            return Err(AutError::InvalidFinal(*state,len));
        }
        // ***
        for (state, map) in transitions.iter().enumerate() {
            if let Some(&letter) = map.keys().find(|&x| !alphabet.contains(x)) {
                return Err(AutError::UnknownLetter(letter,alphabet.clone()));
            }
            // ***
            for (&letter, destinations) in map {
                if let Some(&destination) = destinations.iter().find(|&&x| x >= len) {
                    return Err(AutError::InvalidTransition(state, letter, destination, len));
                }
            }
        }
        // ***
        Ok(AutNFA {
            alphabet,
            initials,
            finals,
            transitions,
        })
    }
}


