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
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;


use crate::dfa::dfa::AutDFA;
use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, AutGraphvizDrawable};
use crate::traits::translate::AutTranslatable;


impl<Letter, Printer> AutGraphvizDrawable<Letter, Printer> for AutDFA<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn to_dot(&self,
              draw_accessibility : bool,
              active_states : &HashSet<usize>,
              printer : &Printer) -> GraphVizDiGraph {
        <AutNFA<Letter> as AutGraphvizDrawable<Letter, Printer>>::to_dot(&self.to_nfa(), draw_accessibility, active_states, printer)
    }

}