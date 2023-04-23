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

use core::fmt;
use std::collections::HashSet;
use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;

use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, ExpBREPrintable};


impl<Letter, Printer> ExpBREPrintable<Letter, Printer> for ExpBRE<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn regexp_to_string(&self, use_html: bool) -> String {
        <TermBRE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(&self.term, use_html)
    }

}


impl<Letter : AutLetter> TermBRE<Letter> {

    pub fn is_string_repr_atomic<Printer : AbstractLanguagePrinter<Letter>>(&self) -> bool {
        match self {
            TermBRE::Empty => {true},
            TermBRE::Epsilon => {true},
            TermBRE::Literal(letter) => {Printer::is_letter_string_repr_atomic(letter)},
            TermBRE::Concat(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.get(0).unwrap().is_string_repr_atomic::<Printer>()
                    },
                    _ => { false }
                }
            },
            TermBRE::Union(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.iter().next().unwrap().is_string_repr_atomic::<Printer>()
                    },
                    _ => { false }
                }
            },
            TermBRE::Kleene(_) => {false}
        }
    }

}

impl<Letter, Printer> ExpBREPrintable<Letter, Printer> for TermBRE<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn regexp_to_string(&self, use_html: bool) -> String {
        match self {
            TermBRE::Empty => {Printer::get_empty_symbol(use_html).to_string()},
            TermBRE::Epsilon => {Printer::get_epsilon_symbol(use_html).to_string()},
            TermBRE::Literal(letter) => {Printer::get_letter_string_repr(letter)},
            TermBRE::Concat(sub_terms) => {
                let sub_strs_atoms : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (<TermBRE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(t, use_html),
                         t.is_string_repr_atomic::<Printer>()))
                        .collect();
                let sub_strs : Vec<String> =
                if Printer::get_concatenation_separator(false).len() > 0 {
                    sub_strs_atoms.into_iter()
                        .map(|(repr,is_atomic)|
                            if is_atomic {
                                repr
                            } else {
                                format!("({})",repr)
                            }
                    ).collect()
                } else {
                    sub_strs_atoms.into_iter()
                        .map(|(repr,_)|
                            repr
                        ).collect()
                };
                sub_strs.join(Printer::get_concatenation_separator(use_html))
            },
            TermBRE::Union(sub_terms) => {
                let sub_strs_atoms : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (<TermBRE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(t, use_html),
                         t.is_string_repr_atomic::<Printer>()))
                    .collect();
                let sub_strs : Vec<String> =
                    if Printer::get_alternation_separator(false).len() > 0 {
                        sub_strs_atoms.into_iter()
                            .map(|(repr,is_atomic)|
                                if is_atomic {
                                    repr
                                } else {
                                    format!("({})",repr)
                                }
                            ).collect()
                    } else {
                        sub_strs_atoms.into_iter()
                            .map(|(repr,_)|
                                repr
                            ).collect()
                    };
                sub_strs.join(Printer::get_alternation_separator(use_html))
            },
            TermBRE::Kleene(sub_term) =>{
                if sub_term.is_string_repr_atomic::<Printer>() {
                    format!("{:}*", <TermBRE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html))
                } else {
                    format!("({:})*", <TermBRE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html))
                }
            }
        }
    }

}

