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

use std::collections::{BTreeSet, HashSet};
use maplit::{btreeset, hashset};

use crate::traits::letter::AutLetter;


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TermBRE<Letter: AutLetter> {
    Empty,
    Epsilon,
    Literal(Letter),
    Union(BTreeSet<TermBRE<Letter>>),
    Concat(Vec<TermBRE<Letter>>),
    Kleene(Box<TermBRE<Letter>>)
}

impl<Letter: AutLetter> TermBRE<Letter> {

    pub fn get_alphabet(&self) -> HashSet<Letter> {
        let mut stack = vec![self];
        let mut alphabet = hashset!{};
        // ***
        while let Some(x) = stack.pop() {
            match x {
                TermBRE::Literal(l) => {
                    alphabet.insert(*l);
                }
                TermBRE::Union(sub_terms) => {
                    sub_terms.iter().for_each(|x| stack.push(x))
                },
                TermBRE::Concat(sub_terms) => {
                    sub_terms.iter().for_each(|x| stack.push(x))
                },
                TermBRE::Kleene(sub_term) => {
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
            TermBRE::Empty => true,
            TermBRE::Epsilon => false,
            TermBRE::Literal(_) => false,
            TermBRE::Union(sub_terms) => {
                sub_terms.iter().all(|t| t.is_empty())
            },
            TermBRE::Concat(sub_terms) => {
                sub_terms.iter().any(|t| t.is_empty())
            },
            TermBRE::Kleene(_) => false
        }
    }

    pub fn expresses_epsilon(&self) -> bool {
        match self {
            TermBRE::Empty => false,
            TermBRE::Epsilon => true,
            TermBRE::Literal(_) => false,
            TermBRE::Union(sub_terms) => {
                sub_terms.iter().any(|t| t.expresses_epsilon())
            },
            TermBRE::Concat(sub_terms) => {
                sub_terms.iter().all(|t| t.expresses_epsilon())
            },
            TermBRE::Kleene(_) => true
        }
    }

    pub fn unite(mut self, other : Self) -> Self {
        self = match (self,other) {
            (TermBRE::Union(mut sub1), TermBRE::Union(sub2)) => {
                for t in sub2 {
                    sub1.insert(t);
                }
                TermBRE::Union(sub1)
            },
            (TermBRE::Empty, t) => t,
            (t, TermBRE::Empty) => t,
            (TermBRE::Union(mut sub1), t) => {
                sub1.insert(t);
                TermBRE::Union(sub1)
            },
            (t, TermBRE::Union(mut sub2)) => {
                sub2.insert(t);
                TermBRE::Union(sub2)
            },
            (t1, t2) => {
                TermBRE::Union(btreeset!{t1,t2})
            }
        };
        self
    }

    pub fn concatenate(mut self, other: Self) -> Self {
        self = match (self,other) {
            (TermBRE::Concat(mut sub1), TermBRE::Concat(mut sub2)) => {
                sub1.append(&mut sub2);
                TermBRE::Concat(sub1)
            },
            (TermBRE::Epsilon, t) => t,
            (t, TermBRE::Epsilon) => t,
            (TermBRE::Empty, _) => TermBRE::Empty,
            (_, TermBRE::Empty) => TermBRE::Empty,
            (TermBRE::Concat(mut sub1), t) => {
                sub1.push(t);
                TermBRE::Concat(sub1)
            },
            (t, TermBRE::Concat(mut sub2)) => {
                sub2.insert(0,t);
                TermBRE::Concat(sub2)
            },
            (t1, t2) => TermBRE::Concat(vec![t1, t2]),
        };
        self
    }

}

