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
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::traits::error::AutError;

pub trait AutLetter : Eq + Hash + Copy + Clone + Debug + Ord {}


pub trait AutAlphabetSubstitutable <Letter: AutLetter>  : Sized {

    fn substitute_alphabet(self,
                           new_alphabet : HashSet<Letter>,
                           substitution : &dyn Fn(Letter) -> Letter) -> Result<Self,AutError<Letter>>;

    fn substitute_letters_within_alphabet(self,
                                          substitution : &dyn Fn(Letter) -> Letter) -> Result<Self,AutError<Letter>>;

    fn hide_letters(self,
                    should_hide : &dyn Fn(Letter) -> bool) -> Self;

}

