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

use std::ops::RangeBounds;
use crate::dfa::dfa::AutDFA;
use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::error::AutError;
use crate::traits::translate::AutTranslatable;

impl<Letter: AutLetter> AutBuildable<Letter> for AutDFA<Letter> {
    fn unite(self, other : Self) -> Result<Self,AutError<Letter>> {
        match self.to_nfa().unite(other.to_nfa()) {
            Err(e) => {Err(e)},
            Ok(as_nfa) => {Ok(as_nfa.to_dfa())}
        }
    }

    fn concatenate(self, other : Self) -> Result<Self,AutError<Letter>> {
        match self.to_nfa().concatenate(other.to_nfa()) {
            Err(e) => {Err(e)},
            Ok(as_nfa) => {Ok(as_nfa.to_dfa())}
        }
    }

    fn repeat(self, num :usize) -> Self {
        self.to_nfa().repeat(num).to_dfa()
    }

    fn kleene(self) -> Self {
        self.to_nfa().kleene().to_dfa()
    }

    fn at_most(self, num : usize) -> Self {
        self.to_nfa().at_most(num).to_dfa()
    }

    fn at_least(self, num : usize) -> Self {
        self.to_nfa().at_least(num).to_dfa()
    }

    fn repeat_range<R: RangeBounds<usize>>(self, r: R) -> Self {
        self.to_nfa().repeat_range(r).to_dfa()
    }
}


