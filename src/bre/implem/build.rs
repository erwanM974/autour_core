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
use std::ops::Bound::{Included,Excluded,Unbounded};

use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;

use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::error::AutError;

impl<Letter: AutLetter> AutBuildable<Letter> for ExpBRE<Letter> {

    fn unite(mut self, other : Self) -> Result<Self,AutError<Letter>> {
        if self.alphabet != other.alphabet {
            return Err(AutError::OperationOnLanguagesOverDifferentAlphabets(self.alphabet,
                                                                            other.alphabet));
        }
        self.term = self.term.unite(other.term);
        Ok(self)
    }

    fn concatenate(mut self, other: Self) -> Result<Self,AutError<Letter>> {
        if self.alphabet != other.alphabet {
            return Err(AutError::OperationOnLanguagesOverDifferentAlphabets(self.alphabet,
                                                                            other.alphabet));
        }
        self.term = self.term.concatenate(other.term);
        Ok(self)
    }

    fn repeat(mut self, num : usize) -> Self {
        self.term = (0..num)
            .fold(TermBRE::Epsilon,
                  |acc, _| acc.concatenate(self.term.clone())
            );
        self
    }

    fn kleene(mut self) -> Self {
        self.term = match self.term {
            TermBRE::Empty => TermBRE::Empty,
            TermBRE::Epsilon => TermBRE::Epsilon,
            TermBRE::Kleene(t) => TermBRE::Kleene(t),
            _ => TermBRE::Kleene(Box::new(self.term))
        };
        self
    }

    fn at_most(mut self, num: usize) -> Self {
        self.term = self.term.unite(TermBRE::Epsilon);
        self.repeat(num)
    }

    fn at_least(self, num: usize) -> ExpBRE<Letter> {
        self.clone().repeat(num).concatenate(self.kleene()).unwrap()
    }

    fn repeat_range<R: RangeBounds<usize>>(self, r: R) -> Self {
        let start = match r.start_bound() {
            Included(&a) => a,
            Excluded(&a) => a + 1,
            Unbounded => 0,
        };
        // ***
        let end_opt = match r.end_bound() {
            Included(&a) => Some(a),
            Excluded(&a) => Some(a - 1),
            Unbounded => None,
        };
        // ***
        match end_opt {
            None => {
                self.at_least(start)
            },
            Some(end) => {
                if end < start {
                    panic!()
                } else {
                    self.clone().repeat(start).concatenate(self.at_most(end - start)).unwrap()
                }
            }
        }
    }

}

