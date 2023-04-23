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
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use crate::traits::letter::AutLetter;



pub trait AbstractLanguagePrinter<Letter : AutLetter> {

    fn is_letter_string_repr_atomic(letter : &Letter) -> bool;

    fn get_letter_string_repr(letter : &Letter) -> String;

    fn get_concatenation_separator(use_html : bool) -> &'static str;

    fn get_alternation_separator(use_html : bool) -> &'static str;

    fn get_intersection_separator(use_html : bool) -> &'static str;

    fn get_wildcard_symbol(use_html : bool) -> &'static str;

    fn get_negate_symbol(use_html : bool) -> &'static str;

    fn get_empty_symbol(use_html : bool) -> &'static str;

    fn get_epsilon_symbol(use_html : bool) -> &'static str;

}



pub trait ExpBREPrintable<Letter, Printer> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn regexp_to_string(&self, use_html : bool) -> String;

}





pub const AUT_COLOR_TRIMMED_STATE: GraphvizColor = GraphvizColor::green;
pub const AUT_COLOR_ACCESSIBLE_STATE: GraphvizColor = GraphvizColor::purple;
pub const AUT_COLOR_COACCESSIBLE_STATE: GraphvizColor = GraphvizColor::navy;
pub const AUT_COLOR_OTHER_STATE: GraphvizColor = GraphvizColor::red;

pub const AUT_COLOR_ACTIVE_STATE: GraphvizColor = GraphvizColor::grey;


pub trait AutGraphvizDrawable<Letter, Printer> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn to_dot(&self,
              draw_accessibility : bool,
              active_states : &HashSet<usize>) -> GraphVizDiGraph;

}
