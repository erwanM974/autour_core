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

use std::collections::{BTreeSet, HashSet, VecDeque};
use std::ops::{Add, AddAssign, Mul};
use crate::bre::term::TermBRE;

use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::error::AutError;

#[derive(Debug, Clone)]
pub struct ExpBRE<Letter: AutLetter> {
    pub alphabet: HashSet<Letter>,
    pub term: TermBRE<Letter>,
}

impl<Letter: AutLetter> ExpBRE<Letter> {
    pub fn from_raw(alphabet: HashSet<Letter>,
                    term: TermBRE<Letter>) ->  Result<Self, AutError<Letter>> {
        let appearing_letters = term.get_alphabet();
        if appearing_letters.is_subset(&alphabet) {
            Ok(ExpBRE{alphabet,term})
        } else {
            let not_in : HashSet<Letter> = appearing_letters.difference(&alphabet).cloned().collect();
            let got = not_in.iter().next().cloned().unwrap();
            Err(AutError::UnknownLetter(got,alphabet))
        }
    }
}
