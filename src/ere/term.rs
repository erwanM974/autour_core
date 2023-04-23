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

use std::collections::{BTreeSet, HashSet, VecDeque};
use std::ops::{Add, AddAssign, Mul};
use maplit::hashset;

use crate::traits::letter::AutLetter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TermERE<Letter: AutLetter> {
    Empty,
    Epsilon,
    Literal(Letter),
    Union(BTreeSet<TermERE<Letter>>),
    Concat(Vec<TermERE<Letter>>),
    Repeat(Box<TermERE<Letter>>, usize, Option<usize>),
    Intersection(BTreeSet<TermERE<Letter>>),
    Negation(Box<TermERE<Letter>>),
    Wildcard,
}


impl<Letter: AutLetter> TermERE<Letter> {

    pub fn get_alphabet(&self) -> HashSet<Letter> {
        let mut stack = vec![self];
        let mut alphabet = hashset!{};
        // ***
        while let Some(x) = stack.pop() {
            match x {
                TermERE::Literal(l) => {
                    alphabet.insert(*l);
                }
                TermERE::Union(sub_terms) => {
                    sub_terms.iter().for_each(|x| stack.push(x))
                },
                TermERE::Concat(sub_terms) => {
                    sub_terms.iter().for_each(|x| stack.push(x))
                },
                TermERE::Intersection(sub_terms) => {
                    sub_terms.iter().for_each(|x| stack.push(x))
                },
                TermERE::Repeat(sub_term,_,_) => {
                    stack.push(&**sub_term)
                },
                TermERE::Negation(sub_term) => {
                    stack.push(&**sub_term)
                },
                _ => {}
            }
        }
        // ***
        alphabet
    }

    pub fn is_empty(&self) -> bool {
        match self {
            TermERE::Empty => true,
            TermERE::Epsilon => false,
            TermERE::Literal(_) => false,
            TermERE::Wildcard => false,
            TermERE::Union(sub_terms) => {
                sub_terms.iter().all(|t| t.is_empty())
            },
            TermERE::Concat(sub_terms) => {
                sub_terms.iter().any(|t| t.is_empty())
            },
            TermERE::Intersection(sub_terms) => {
                unimplemented!("to implement need to check intersection empty");
                sub_terms.iter().any(|t| t.is_empty())
            },
            TermERE::Negation(sub_term) => {
                unimplemented!("to implement")
            },
            TermERE::Repeat(sub_term,min,_) => {
                match min {
                    0 => false,
                    _ => sub_term.is_empty()
                }
            }
        }
    }

    pub fn expresses_epsilon(&self) -> bool {
        match self {
            TermERE::Empty => false,
            TermERE::Epsilon => true,
            TermERE::Literal(_) => false,
            TermERE::Wildcard => false,
            TermERE::Union(sub_terms) => {
                sub_terms.iter().any(|t| t.expresses_epsilon())
            },
            TermERE::Concat(sub_terms) => {
                sub_terms.iter().all(|t| t.expresses_epsilon())
            },
            TermERE::Intersection(sub_terms) => {
                sub_terms.iter().all(|t| t.expresses_epsilon())
            },
            TermERE::Negation(sub_term) => {
                !sub_term.expresses_epsilon()
            },
            TermERE::Repeat(sub_term,min,_) => {
                match min {
                    0 => true,
                    _ => sub_term.expresses_epsilon()
                }
            }
        }
    }

}



