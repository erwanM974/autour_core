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
use maplit::{hashmap,hashset};

use graphviz_dot_builder::traits::{GraphVizOutputFormat,DotPrintable};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyleItem;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::GraphvizEdgeStyleItem;


use crate::nfa::nfa::AutNFA;
use crate::tests::printer::CharAsLetterPrinter;
use crate::traits::access::AutAccessible;
use crate::traits::repr::AutGraphvizDrawable;
use crate::traits::transform::AutTransformable;

pub fn get_dfa_mini() -> AutNFA<char> {
    let alphabet : HashSet<char> = vec![b'a',b'b'].into_iter().map(char::from).collect();
    let mut transitions: Vec<HashMap<char, HashSet<usize>>> = vec![hashmap!{};8];
    transitions[0].insert('b', hashset!{1});
    transitions[0].insert('a', hashset!{3});
    transitions[1].insert('a', hashset!{2});
    transitions[1].insert('b', hashset!{5});
    transitions[2].insert('a', hashset!{2});
    transitions[3].insert('a', hashset!{0});
    transitions[3].insert('b', hashset!{4});
    transitions[4].insert('a', hashset!{2});
    transitions[4].insert('b',hashset!{ 5});
    transitions[5].insert('a',hashset!{ 5});
    transitions[5].insert('b',hashset!{ 5});
    transitions[6].insert('b', hashset!{5});
    transitions[7].insert('b', hashset!{2});
    let dfa1 = AutNFA::<char>::from_raw(alphabet,
                                        hashset!{0},
                                        hashset!{1,2,4},
                                        transitions).unwrap();
    // ***
    return dfa1;
}

#[test]
pub fn test_minimize() {
    // ***
    let init = get_dfa_mini();
    let mini = init.clone().minimize();
    let acc = init.clone().make_accessible();
    let coacc = init.clone().make_coaccessible();
    let trim = init.clone().trim();
    // ***
    let init_graph =
        <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&init,
                                                                                 true,
                                                                                 &hashset!{});
    let mini_graph =
        <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&mini,
                                                                                 true,
                                                                                 &hashset!{});
    let acc_graph =
        <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&acc,
                                                                                 true,
                                                                                 &hashset!{});
    let coacc_graph =
        <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&coacc,
                                                                                 true,
                                                                                 &hashset!{});
    let trim_graph =
        <AutNFA<char> as AutGraphvizDrawable<char, CharAsLetterPrinter>>::to_dot(&trim,
                                                                                 true,
                                                                                 &hashset!{});
    // ***
    let mut bridge_graph = GraphVizDiGraph::new(vec![]);
    bridge_graph.add_cluster(init_graph.as_cluster("init".to_string(),vec![],Some("init".to_string())));
    bridge_graph.add_cluster(acc_graph.as_cluster("acc".to_string(),vec![],Some("acc".to_string())));
    bridge_graph.add_cluster(coacc_graph.as_cluster("coacc".to_string(),vec![],Some("coacc".to_string())));
    bridge_graph.add_cluster(trim_graph.as_cluster("trim".to_string(),vec![],Some("trim".to_string())));
    bridge_graph.add_cluster(mini_graph.as_cluster("mini".to_string(),vec![],Some("mini".to_string())));
    bridge_graph.add_edge(GraphVizEdge ::new("initS5".to_string(),
                                             Some("init".to_string()),
                                             "miniI0".to_string(),
                                             Some("mini".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("minimize".to_string())]));
    bridge_graph.add_edge(GraphVizEdge ::new("initS5".to_string(),
                                             Some("init".to_string()),
                                             "accI0".to_string(),
                                             Some("acc".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("make accessible".to_string())]));
    bridge_graph.add_edge(GraphVizEdge ::new("initS5".to_string(),
                                             Some("init".to_string()),
                                             "coaccI0".to_string(),
                                             Some("coacc".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("make coaccessible".to_string())]));
    bridge_graph.add_edge(GraphVizEdge ::new("initS5".to_string(),
                                             Some("init".to_string()),
                                             "trimI0".to_string(),
                                             Some("trim".to_string()),
                                             vec![GraphvizEdgeStyleItem::Label("trim".to_string())]));
    let parent_folder : Vec<String> = vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect();


    bridge_graph.print_dot(&parent_folder,
                           &"minimize".to_string(),
                           &GraphVizOutputFormat::svg);
}