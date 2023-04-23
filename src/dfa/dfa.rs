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

use crate::traits::letter::AutLetter;
use crate::traits::error::AutError;


#[derive(Debug, Clone)]
pub struct AutDFA<Letter: AutLetter> {
    pub alphabet : HashSet<Letter>,
    pub initial: usize,
    pub finals: HashSet<usize>,
    pub transitions: Vec<HashMap<Letter, usize>>,
}

impl<Letter: AutLetter> AutDFA<Letter> {

    /// Returns an empty automaton
    pub fn new_void_object(alphabet: HashSet<Letter>) -> AutDFA<Letter> {
        AutDFA {
            alphabet,
            initial: 0,
            finals: HashSet::new(),
            transitions: vec![HashMap::new()],
        }
    }

    /// Returns an automaton built from the raw arguments.
    pub fn from_raw(
        alphabet : HashSet<Letter>,
        initial: usize,
        finals: HashSet<usize>,
        transitions: Vec<HashMap<Letter, usize>>,
    ) -> Result<Self, AutError<Letter>> {
        let len = transitions.len();
        // ***
        if initial >= len {
            return Err(AutError::InvalidInitial(initial,len));
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
            if let Some((&letter, &destination)) = map.iter().find(|(_, &destination)| destination >= len) {
                return Err(AutError::InvalidTransition(state, letter, destination, len));
            }
        }
        // ***
        Ok(AutDFA {
            alphabet,
            initial,
            finals,
            transitions,
        })
    }
}

