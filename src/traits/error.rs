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

use std::collections::HashSet;
use std::fmt;

use crate::traits::letter::AutLetter;

#[derive(Debug)]
pub enum AutError<Letter : AutLetter> {
    UnknownLetter(Letter,HashSet<Letter>),
    InvalidStateToRun(usize,usize),
    InvalidInitial(usize,usize),
    InvalidFinal(usize,usize),
    InvalidEpsilonTrans(usize,Option<usize>,usize),
    InvalidTransition(usize, Letter, usize, usize),
    Other(String),
    OperationOnLanguagesOverDifferentAlphabets(HashSet<Letter>, HashSet<Letter>)
}

impl<Letter : AutLetter> fmt::Display for AutError<Letter> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AutError::Other(msg) => {
                write!(f, "{}", msg)
            },
            AutError::OperationOnLanguagesOverDifferentAlphabets(al1,al2) => {
                write!(f, "attempting operation on two languages defined over different alphabets '{:?}' and '{:?}'", al1, al2)
            },
            AutError::UnknownLetter(letter,alphabet) => {
                write!(f, "letter '{:?}' not in alphabet '{:?}'", letter, alphabet)
            },
            AutError::InvalidStateToRun(run_stid,num_states) => {
                write!(f, "running through state id '{}' which is not in set of states '{:?}'", run_stid, 0..*num_states)
            },
            AutError::InvalidInitial(init_stid,num_states) => {
                write!(f, "initial state id '{}' not in set of states '{:?}'", init_stid, 0..*num_states)
            },
            AutError::InvalidFinal(final_stid,num_states) => {
                write!(f, "final state id '{}' not in set of states '{:?}'", final_stid, 0..*num_states)
            },
            AutError::InvalidEpsilonTrans(orig_stid,targ_opt, num_states) => {
                match targ_opt {
                    None => {
                        write!(f, "origin state '{}' of epsilon transition is not in set of states '{:?}'", orig_stid, 0..*num_states)
                    },
                    Some(targ_stid) => {
                        write!(f, "epsilon transition '{} -> {}' invalid because target is not in set of states '{:?}'", orig_stid, targ_stid, 0..*num_states)
                    }
                }
            },
            AutError::InvalidTransition(orig_stid, letter, targ_stid,num_states) => {
                write!(f, "target of transition '{} -- {:?} -> {}' not in set of states '{:?}'",orig_stid, letter, targ_stid, 0..*num_states)
            }
        }
    }
}

