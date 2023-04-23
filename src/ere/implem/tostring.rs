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



use crate::ere::term::TermERE;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, ExpBREPrintable};

impl<Letter : AutLetter> TermERE<Letter> {

    pub fn is_string_repr_atomic<Printer : AbstractLanguagePrinter<Letter>>(&self) -> bool {
        match self {
            TermERE::Empty => {true},
            TermERE::Epsilon => {true},
            TermERE::Literal(letter) => {Printer::is_letter_string_repr_atomic(letter)},
            TermERE::Wildcard => {true},
            TermERE::Concat(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.get(0).unwrap().is_string_repr_atomic::<Printer>()
                    },
                    _ => { false }
                }
            },
            TermERE::Union(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.iter().next().unwrap().is_string_repr_atomic::<Printer>()
                    },
                    _ => { false }
                }
            },
            TermERE::Intersection(sub_terms) => {
                match sub_terms.len() {
                    0 => { true },
                    1 => {
                        sub_terms.iter().next().unwrap().is_string_repr_atomic::<Printer>()
                    },
                    _ => { false }
                }
            },
            TermERE::Negation(_) => {false},
            TermERE::Repeat(_,_,_) => {false}
        }
    }

}

impl<Letter, Printer> ExpBREPrintable<Letter, Printer> for TermERE<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn regexp_to_string(&self, use_html: bool) -> String {
        match self {
            TermERE::Empty => {Printer::get_empty_symbol(use_html).to_string()},
            TermERE::Epsilon => {Printer::get_epsilon_symbol(use_html).to_string()},
            TermERE::Literal(letter) => {Printer::get_letter_string_repr(letter)},
            TermERE::Wildcard => {Printer::get_wildcard_symbol(use_html).to_string()},
            TermERE::Negation(sub_term) => {
                let sub_term_as_string = <TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html);
                if sub_term.is_string_repr_atomic::<Printer>() {
                    format!("{:}{:}", Printer::get_negate_symbol(use_html), sub_term_as_string)
                } else {
                    format!("{:}({:})", Printer::get_negate_symbol(use_html), sub_term_as_string)
                }
            },
            TermERE::Concat(sub_terms) => {
                let sub_strs : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (<TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(t, use_html),
                         t.is_string_repr_atomic::<Printer>()))
                    .collect();
                sub_strs.iter().fold("".to_owned(),
                                     |x,(repr,is_atomic)|
                                         if *is_atomic {
                                             x + Printer::get_concatenation_separator(use_html) + repr
                                         } else {
                                             x + Printer::get_concatenation_separator(use_html) + "(" + repr + ")"
                                         })
            },
            TermERE::Union(sub_terms) => {
                let sub_strs : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (<TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(t, use_html),
                         t.is_string_repr_atomic::<Printer>()))
                    .collect();
                sub_strs.iter().fold("".to_owned(),
                                     |x,(repr,is_atomic)|
                                         if *is_atomic {
                                             x + Printer::get_alternation_separator(use_html) + repr
                                         } else {
                                             x + Printer::get_alternation_separator(use_html) + "(" + repr + ")"
                                         })
            },
            TermERE::Intersection(sub_terms) => {
                let sub_strs : Vec<(String,bool)> = sub_terms.iter()
                    .map(|t|
                        (<TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(t, use_html),
                         t.is_string_repr_atomic::<Printer>()))
                    .collect();
                sub_strs.iter().fold("".to_owned(),
                                     |x,(repr,is_atomic)|
                                         if *is_atomic {
                                             x + Printer::get_intersection_separator(use_html) + repr
                                         } else {
                                             x + Printer::get_intersection_separator(use_html) + "(" + repr + ")"
                                         })
            },
            TermERE::Repeat(sub_term, min, None) => {
                let sub_term_as_string = <TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html);
                match min {
                    0 => {
                        if sub_term.is_string_repr_atomic::<Printer>() {
                            format!("{:}*",sub_term_as_string)
                        } else {
                            format!("({:})*",sub_term_as_string)
                        }
                    },
                    1 => {
                        if sub_term.is_string_repr_atomic::<Printer>() {
                            format!("{:}+",sub_term_as_string)
                        } else {
                            format!("({:})+",sub_term_as_string)
                        }
                    },
                    _ => {
                        if sub_term.is_string_repr_atomic::<Printer>() {
                            format!("{:}{{{},}}",sub_term_as_string,min)
                        } else {
                            format!("({:}){{{},}}",sub_term_as_string,min)
                        }
                    }
                }
            },
            TermERE::Repeat(sub_term, 0, Some(1)) => {
                let sub_term_as_string = <TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html);
                if sub_term.is_string_repr_atomic::<Printer>() {
                    format!("{:}?",sub_term_as_string)
                } else {
                    format!("({:})?",sub_term_as_string)
                }
            },
            TermERE::Repeat(sub_term, min,  Some(max)) => {
                let mut sub_term_as_string = <TermERE<Letter> as ExpBREPrintable<Letter, Printer>>::regexp_to_string(sub_term, use_html);
                if !sub_term.is_string_repr_atomic::<Printer>() {
                    sub_term_as_string = format!("({:})",sub_term_as_string)
                }
                match min {
                    0 => {
                        format!("{}{{,{}}}", sub_term_as_string, max)
                    },
                    _ => {
                        if min == max {
                            format!("{}{{{}}}", sub_term_as_string, min)
                        } else {
                            format!("{}{{{},{}}}", sub_term_as_string, min, max)
                        }
                    }
                }
            }
        }
    }

}


