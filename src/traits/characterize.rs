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


pub trait AutCharacterizable<Letter: AutLetter> {

    /// An automaton is *complete* if for all its *states* there are outgoing *transitions* corresponding to every *letter* of the *alphabet*
    fn is_complete(&self) -> bool;

    /// An automaton is said *empty* if it doesn't accept any word
    fn is_empty(&self) -> bool;

    /// An automaton is said *universal* if there are no words that it doesn't accept
    fn is_universal(&self) -> bool;

    /// 'self' contains 'other' if and only if each *word* accepted by 'self' is also accepted by 'other'
    fn contains(&self, other : &Self) -> bool;

    /// 'self' equals 'other' if and only if they define the same language
    fn equals(&self, other : &Self) -> bool {
        self.contains(other) && other.contains(self)
    }

}