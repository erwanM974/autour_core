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


use crate::printers::commons::*;


use crate::traits::repr::AbstractLanguagePrinter;


pub struct CharAsLetterPrinter {}

impl AbstractLanguagePrinter<char> for CharAsLetterPrinter {

    fn is_letter_string_repr_atomic(_letter: &char) -> bool {
        true
    }

    fn get_letter_string_repr(letter: &char) -> String {
        letter.to_string()
    }

    fn get_concatenation_separator(_use_html: bool) -> &'static str {
        SYNTAX_CONCATENATION_EMPTY
    }

    fn get_alternation_separator(_use_html: bool) -> &'static str {
        SYNTAX_ALTERNATION
    }

    fn get_intersection_separator(use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_INTERSECTION_HTML
        } else {
            SYNTAX_INTERSECTION_CLEAR
        }
    }

    fn get_wildcard_symbol(_use_html: bool) -> &'static str {
        SYNTAX_WILDCARD_DOT
    }

    fn get_negate_symbol(use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_NEGATION_HTML
        } else {
            SYNTAX_NEGATION_CLEAR
        }
    }

    fn get_empty_symbol(use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EMPTY_HTML
        } else {
            SYNTAX_EMPTY_CLEAR
        }
    }

    fn get_epsilon_symbol(use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EPSILON_HTML
        } else {
            SYNTAX_EPSILON_CLEAR
        }
    }
}