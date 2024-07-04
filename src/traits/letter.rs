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
use std::fmt::Debug;
use std::hash::Hash;



pub trait AutLetter : Eq + Hash + Copy + Clone + Debug + Ord {}

impl<T : Eq + Hash + Copy + Clone + Debug + Ord> AutLetter for T {}

pub trait AutAlphabetSubstitutable <Letter: AutLetter>  : Sized {

    /// replaces specific letters occurring on the automaton/regular expression with some other letters
    /// the optional boolean specifies whether or not substituted letters should also be removed from the alphabet
    fn substitute_letters(self,
                          remove_from_alphabet : bool,
                          substitution : &dyn Fn(&Letter) -> Letter) -> Self;


    /// replaces specific letters occurring on the automaton/regular expression with the empty word
    /// the optional boolean specifies whether or not hidden letters should also be removed from the alphabet
    fn hide_letters(self,
                          remove_from_alphabet : bool,
                          should_hide : &dyn Fn(&Letter) -> bool) -> Self;

}

pub fn get_new_alphabet_from_substitution<Letter: AutLetter>(alphabet : &HashSet<Letter>,
                                          remove_from_alphabet : bool,
                                      substitution : &dyn Fn(&Letter) -> Letter) -> HashSet<Letter> {
    let transformed_alphabet : HashSet<Letter> = alphabet
        .iter().map(|letter| substitution(letter)).collect();
    if remove_from_alphabet {
        transformed_alphabet
    } else {
        let mut got = alphabet.clone();
        got.extend(transformed_alphabet.into_iter());
        got
    }
}


pub fn get_new_alphabet_from_hiding<Letter: AutLetter>(alphabet : &HashSet<Letter>,
                                                       remove_from_alphabet : bool,
                                                       should_hide : &dyn Fn(&Letter) -> bool) -> HashSet<Letter> {
    if remove_from_alphabet {
        alphabet.iter().filter(|l| !should_hide(l)).cloned().collect()
    }  else {
        alphabet.clone()
    }
}
