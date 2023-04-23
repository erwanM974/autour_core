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
use maplit::{hashset,hashmap};
use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::traits::access::AutAccessible;
use crate::traits::build::AutBuildable;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;





impl<Letter: AutLetter> AutTransformable<Letter> for AutGNFA<Letter> {

    fn is_complete(&self) -> bool {
        self.to_nfait().is_complete()
    }

    fn complete(mut self) -> Self {
        if self.is_complete() {
            return self;
        }
        self.to_nfait().complete().to_gnfa()
    }

    fn is_empty(&self) -> bool {
        self.to_nfait().is_empty()
    }

    fn is_universal(&self) -> bool {
        self.to_nfait().is_universal()
    }

    fn negate(self) -> Self {
        self.to_nfait().negate().to_gnfa()
    }

    fn reverse(mut self) -> Self {
        self.to_nfait().reverse().to_gnfa()
    }

    fn minimize(self) -> Self {
        self.to_dfa().minimize().to_bre().to_gnfa()
    }

    // De Morgan
    fn intersect(self,
                 other: Self) -> Self {
        self.negate().unite(other.negate()).unwrap().negate()
    }

    fn contains(&self,
                other: &Self) -> bool {
        self.clone().negate().intersect(other.clone()).is_empty()
    }
}
