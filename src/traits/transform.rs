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


use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;


pub trait AutTransformable<Letter: AutLetter> : Sized {

    /// An automaton is *complete* if for all its *states* there are outgoing *transitions* corresponding to every *letter* of the *alphabet*
    fn complete(self) -> Self;

    /// Returns an automaton that accepts a word if and only if *self* doesn't accept this word
    fn negate(self) -> Self;

    /// Returns an automaton that accepts a word if and only if *self* accepts the reversed word
    fn reverse(self) -> Self;

    /// Returns a minimal automaton which accepts the same set of words
    fn minimize(self) -> Self;

    /// Returns an automaton that accepts a word if and only if this word is accepted by both *self* and *other*
    fn intersect(self, other: Self) -> Result<Self,AutError<Letter>>;

    /// Returns an automaton that accepts words which are interleavings of words of *self* and *other*
    fn interleave(self, other: Self) -> Result<Self,AutError<Letter>>;

}

