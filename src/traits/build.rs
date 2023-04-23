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
use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;


pub trait AutBuildable<Letter: AutLetter> : Sized {
    /// Returns the automaton that accepts a word if and only if it is accepted by *self* or by *other*.
    fn unite(self, other: Self) -> Result<Self,AutError<Letter>>;

    /// Returns the automaton that accepts a word if and only if it is the concatenation of a word accepted by *self* and of a word accepted by *other*.
    fn concatenate(self, other: Self) -> Result<Self,AutError<Letter>>;

    /// Return the automaton that accepts a word if and only if it is the concatenation of a specific number *num* of words accepted by *self*.
    fn repeat(self, num : usize) -> Self;

    /// Returns the automaton that accepts a word if and only if it is the concatenation of any finite number of words accepted by *self* (possibly 0).
    fn kleene(self) -> Self;

    /// Returns the automaton that accepts a word if and only if it is the concatenation of at most *num* words accepted by *self*.
    fn at_most(self, num: usize) -> Self;

    /// Returns the automaton that accepts a word if and only if it is the concatenation of at least *num* words accepted by *self*.
    fn at_least(self, num: usize) -> Self;

    /// Returns the automaton that accepts a word if and only if it is the concatenation of a number in the range *r* of words accepted by *self*.
    fn repeat_range<R: RangeBounds<usize>>(self, r: R) -> Self;
}

