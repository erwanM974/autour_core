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


use crate::traits::letter::AutLetter;


pub trait AutTransformable<Letter: AutLetter> {

    /// An automaton is *complete* if for all its *states* there are outgoing *transitions* corresponding to every *letter* of the *alphabet*
    fn is_complete(&self) -> bool;
    fn complete(self) -> Self;

    /// An automaton is said *empty* if it doesn't accept any word
    /// e.g. there are no final states for a DFA
    fn is_empty(&self) -> bool;

    /// An automaton is said *universal* if there are no words that it doesn't accept
    /// e.g. all states are final for a DFA
    fn is_universal(&self) -> bool;

    /// Returns an automaton that accepts a word if and only if *self* doesn't accept this word
    fn negate(self) -> Self;

    /// Returns an automaton that accepts a word if and only if *self* accepts the reversed word
    fn reverse(self) -> Self;

    /// Returns a minimal automaton which accepts the same set of words
    fn minimize(self) -> Self;

    /// Returns an automaton that accepts a word if and only if this word is accepted by both *self* and *other*
    fn intersect(self, other: Self) -> Self;

    /// 'self' contains 'other' if and only if each *word* accepted by 'self' is also accepted by 'other'
    fn contains(&self, other : &Self) -> bool;

}

