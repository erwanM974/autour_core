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

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use maplit::{hashset,hashmap,btreeset};

use graphviz_dot_builder::traits::{GraphVizOutputFormat,DotPrintable};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyleItem;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::GraphvizEdgeStyleItem;

use crate::bre::term::TermBRE;
use crate::gnfa::gnfa::AutGNFA;

use crate::tests::printer::CharAsLetterPrinter;
use crate::traits::access::AutAccessible;
use crate::traits::repr::AutGraphvizDrawable;
use crate::traits::transform::AutTransformable;
use crate::traits::letter::AutAlphabetSubstitutable;


pub fn get_gnfa2() -> AutGNFA<char> {
    let alphabet : HashSet<char> = vec![b'a',b'b',b'c',b'd'].into_iter().map(char::from).collect();
    let mut transitions: HashMap<(usize,usize), TermBRE<char>> = hashmap!{};
    transitions.insert((0,1),
                       TermBRE::Kleene(Box::new(
                           TermBRE::Union(btreeset!{TermBRE::Literal(char::from(b'a')),TermBRE::Literal(char::from(b'b'))})
                       ))
    );
    transitions.insert((0,2), TermBRE::Literal(char::from(b'd')));
    transitions.insert((1,2),
                       TermBRE::Concat(vec![TermBRE::Literal(char::from(b'b')),TermBRE::Literal(char::from(b'c'))])
    );
    AutGNFA::<char>::from_raw(alphabet,
                                         3,
                                         0,2,
                                         transitions).unwrap()
}

fn hide_b(my_char : char) -> bool {
    my_char == char::from(b'b')
}

fn sub_b_by_c(my_char : char) -> char {
    if my_char == char::from(b'b') {
        char::from(b'c')
    } else {
        my_char
    }
}

#[test]
pub fn test_minimize() {
    // ***
    let init = get_gnfa2();
    let hide = init.clone().hide_letters(&hide_b);
    let subs = init.clone().substitute_letters_within_alphabet(&sub_b_by_c).unwrap();
    let init_graph =
        <AutGNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&init,
                                                                                 true,
                                                                                 &hashset!{});
    let hide_graph =
        <AutGNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&hide,
                                                                                 true,
                                                                                 &hashset!{});
    let subs_graph =
        <AutGNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&subs,
                                                                                 true,
                                                                                 &hashset!{});
    // ***
    let mut bridge_graph = GraphVizDiGraph::new(vec![]);
    bridge_graph.add_cluster(init_graph.as_cluster("init".to_string(),vec![],Some("init".to_string())));
    bridge_graph.add_cluster(hide_graph.as_cluster("hide".to_string(),vec![],Some("hide".to_string())));
    bridge_graph.add_cluster(subs_graph.as_cluster("subs".to_string(),vec![],Some("subs".to_string())));
    bridge_graph.add_edge(GraphVizEdge ::new("initS2".to_string(),
                                             Some("init".to_string()),
                                             "hideI0".to_string(),
                                             Some("hide".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("hide letters".to_string())]));
    bridge_graph.add_edge(GraphVizEdge ::new("initS2".to_string(),
                                             Some("init".to_string()),
                                             "subsI0".to_string(),
                                             Some("subs".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("substitute letters".to_string())]));
    let parent_folder : Vec<String> = vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect();


    bridge_graph.print_dot(&parent_folder,
                           &"hide".to_string(),
                           &GraphVizOutputFormat::svg);
}