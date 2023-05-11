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
use maplit::{btreeset, hashset};
use graphviz_dot_builder::traits::{GraphVizOutputFormat,DotPrintable};
use crate::bre::bre::ExpBRE;

use crate::bre::term::TermBRE;
use crate::dfa::dfa::AutDFA;
use crate::nfa::nfa::AutNFA;
use crate::tests::printer::CharAsLetterPrinter;
use crate::traits::repr::AutGraphvizDrawable;
use crate::traits::translate::AutTranslatable;

#[test]
fn bre_tests() {

    let alphabet : HashSet<char> = vec![b'a',b'b'].into_iter().map(char::from).collect();

    let parent_folder : Vec<String> = vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect();

    let t_union = TermBRE::Union(btreeset!{TermBRE::Literal(char::from(b'a')),TermBRE::Literal(char::from(b'b'))});

    let as_bre = ExpBRE::from_raw(alphabet.clone(),t_union.clone()).unwrap();

    <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&as_bre.to_nfa(),
                                                                             true,
                                                                             &hashset!{})
        .print_dot(&parent_folder,
                   &"t_union".to_string(),
                   &GraphVizOutputFormat::svg);

    let t_kleene = TermBRE::Kleene(Box::new(t_union ));

    let as_bre = ExpBRE::from_raw(alphabet.clone(),t_kleene.clone()).unwrap();
    <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&as_bre.to_nfa(),
                                                                             true,
                                                                             &hashset!{})
        .print_dot(&parent_folder,
                   &"t_kleene".to_string(),
                   &GraphVizOutputFormat::svg);

}


