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

use crate::gnfa::gnfa::AutGNFA;
use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::error::AutError;

impl<Letter: AutLetter> AutBuildable<Letter> for AutGNFA<Letter> {

    fn unite(mut self, other: Self) -> Result<Self,AutError<Letter>> {
        unimplemented!();
    }

    fn concatenate(mut self, mut other: Self) -> Result<Self,AutError<Letter>> {
        unimplemented!();
    }

    fn repeat(self, num : usize) -> Self {
        unimplemented!();
    }

    fn kleene(mut self) -> Self {
        unimplemented!();
    }

    fn at_most(mut self, num: usize) -> Self {
        unimplemented!();
    }

    fn at_least(self, num: usize) -> Self {
        unimplemented!();
    }

    fn repeat_range<R: RangeBounds<usize>>(self, r: R) -> Self {
        unimplemented!();
    }
}


