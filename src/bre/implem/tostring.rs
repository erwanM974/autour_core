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


use crate::bre::bre::ExpBRE;
use crate::bre::term::TermBRE;

use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, ExpBREPrintable};


impl<Letter, Printer> ExpBREPrintable<Letter, Printer> for ExpBRE<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn regexp_to_string(&self, use_html: bool, printer : &Printer) -> String {
        self.term.regexp_to_string(use_html, printer)
    }

}


impl<Letter : AutLetter> TermBRE<Letter> {

    pub fn is_string_repr_atomic(&self, printer : &dyn AbstractLanguagePrinter<Letter>) -> bool {
        match self {
            TermBRE::Empty => {true},
            TermBRE::Epsilon => {true},
            TermBRE::Literal(letter) => {printer.is_letter_string_repr_atomic(letter)},
            TermBRE::Concat(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.get(0).unwrap().is_string_repr_atomic(printer)
                    },
                    _ => { false }
                }
            },
            TermBRE::Union(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.iter().next().unwrap().is_string_repr_atomic(printer)
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

    fn regexp_to_string(&self, use_html: bool, printer : &Printer) -> String {
        match self {
            TermBRE::Empty => {printer.get_empty_symbol(use_html).to_string()},
            TermBRE::Epsilon => {printer.get_epsilon_symbol(use_html).to_string()},
            TermBRE::Literal(letter) => {printer.get_letter_string_repr(letter)},
            TermBRE::Concat(sub_terms) => {
                let sub_strs_atoms : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                             (t.regexp_to_string(use_html,printer), t.is_string_repr_atomic(printer)))
                        .collect();
                let sub_strs : Vec<String> =
                if !printer.get_concatenation_separator(false).is_empty() {
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
                sub_strs.join(printer.get_concatenation_separator(use_html))
            },
            TermBRE::Union(sub_terms) => {
                let sub_strs_atoms : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (t.regexp_to_string(use_html,printer), t.is_string_repr_atomic(printer)))
                    .collect();
                let sub_strs : Vec<String> =
                    if !printer.get_alternation_separator(false).is_empty() {
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
                sub_strs.join(printer.get_alternation_separator(use_html))
            },
            TermBRE::Kleene(sub_term) =>{
                if sub_term.is_string_repr_atomic(printer) {
                    format!("{:}*", sub_term.regexp_to_string(use_html,printer))
                } else {
                    format!("({:})*", sub_term.regexp_to_string(use_html,printer))
                }
            }
        }
    }

}

