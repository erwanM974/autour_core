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



use std::collections::BTreeSet;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};
use maplit::{btreeset, hashset};
use crate::dfa::dfa::AutDFA;

use crate::nfa::algos::kameda_weiner::cover::{is_set_of_grids_covering_matrix, replace_states_map_content_with_cover};
use crate::nfa::algos::kameda_weiner::grid::search_all_prime_grids;
use crate::nfa::algos::kameda_weiner::intersection_rule::convert_states_map_to_nfa;
use crate::nfa::algos::kameda_weiner::states_map::KwStatesMap;
use crate::nfa::nfa::AutNFA;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, AutGraphvizDrawable};



pub struct KwLegitCandidate<Letter : AutLetter> {
    pub grids : BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)>,
    pub rcm : KwStatesMap,
    pub nfa : AutNFA<Letter>
}

impl<Letter: AutLetter> KwLegitCandidate<Letter> {
    pub fn new(grids: BTreeSet<(BTreeSet<usize>, BTreeSet<usize>)>, rcm: KwStatesMap, nfa: AutNFA<Letter>) -> Self {
        Self { grids, rcm, nfa }
    }
}

pub fn kameda_weiner_algorithm<Letter : AutLetter>(nfa : &AutNFA<Letter>)
            -> (AutDFA<Letter>,KwStatesMap,KwStatesMap,KwLegitCandidate<Letter>) {
    let (sm,dfa) = KwStatesMap::from_nfa(&nfa);
    let rsm = sm.reduce_matrix();
    let all_prime_grids = search_all_prime_grids(&rsm);
    // ***
    // we will search for a candidate with at worst the same number of states as the original nfa
    let mut num_states_criterion = nfa.transitions.len() + 1;
    let mut candidate : Option<KwLegitCandidate<Letter>> = None;
    // ***
    let mut seen : BTreeSet < BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> > = btreeset!{};
    let mut queue: Vec< BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> > = vec![btreeset!{}];
    while let Some(next_cover_candidate) = queue.pop() {
        seen.insert(next_cover_candidate.clone());
        if next_cover_candidate.len() >= num_states_criterion {
            continue
        }
        if is_set_of_grids_covering_matrix(&rsm,&next_cover_candidate) {
            let rcm = replace_states_map_content_with_cover(&rsm,&next_cover_candidate);
            let rcm_as_nfa = convert_states_map_to_nfa(&rcm,&dfa,next_cover_candidate.len());
            if nfa.equals(&rcm_as_nfa) && rcm_as_nfa.transitions.len() < num_states_criterion {
                num_states_criterion = rcm_as_nfa.transitions.len();
                candidate = Some(KwLegitCandidate::new(next_cover_candidate,rcm,rcm_as_nfa));
            }
        } else {
            for grid in &all_prime_grids {
                if !next_cover_candidate.contains(grid) {
                    let mut new_candidate = next_cover_candidate.clone();
                    new_candidate.insert(grid.clone());
                    if !seen.contains(&new_candidate) && !queue.contains(&new_candidate) {
                        queue.push(new_candidate);
                    }
                }
            }
        }
    }
    // ***
    (dfa,sm,rsm, candidate.unwrap())
}


pub fn draw_kameda_weiner_process<Letter : AutLetter,Printer : AbstractLanguagePrinter<Letter>>
                    (parent_folder : &Vec<String>,
                     name : &String,
                     printer : &Printer,
                     nfa : &AutNFA<Letter>,
                     dfa : &AutDFA<Letter>,
                     sm : &KwStatesMap,
                     rsm : &KwStatesMap,
                     legit : &KwLegitCandidate<Letter>) {
    let sm_nfa = convert_states_map_to_nfa(&sm,&dfa,nfa.transitions.len());
    let rsm_nfa = convert_states_map_to_nfa(&rsm,&dfa,nfa.transitions.len());
    let orig_graph = nfa.to_dot(true,&hashset!{},printer);
    let sm_graph = sm_nfa.to_dot(true,&hashset!{},printer);
    let rsm_graph = rsm_nfa.to_dot(true,&hashset!{},printer);
    let min_graph = legit.nfa.to_dot(true,&hashset!{},printer);
    orig_graph.print_dot(parent_folder,
                         "orig",
                         &GraphVizOutputFormat::png);
    sm_graph.print_dot(parent_folder,
                       "sm",
                       &GraphVizOutputFormat::png);
    rsm_graph.print_dot(parent_folder,
                        "rsm",
                        &GraphVizOutputFormat::png);
    min_graph.print_dot(parent_folder,
                        "min",
                        &GraphVizOutputFormat::png);

    let mut grid_as_str = "".to_string();
    for (grid_rows,grid_cols) in &legit.grids {
        grid_as_str.push_str(&format!("({:?} x {:?})\n", grid_rows,grid_cols));
    }

    let mut bridge_graph = GraphVizDiGraph::new(vec![]);
    bridge_graph.add_node(GraphVizNode::new("orig".to_string(),vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),GraphvizNodeStyleItem::Image("orig.png".to_string()),GraphvizNodeStyleItem::Label("".to_string())]));
    bridge_graph.add_node(GraphVizNode::new("min".to_string(),vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),GraphvizNodeStyleItem::Image("min.png".to_string()),GraphvizNodeStyleItem::Label("".to_string())]));
    bridge_graph.add_node(GraphVizNode::new("sm".to_string(),vec![GraphvizNodeStyleItem::FontName("Courier".to_string()),GraphvizNodeStyleItem::Label(sm.to_ascii_str(false))]));
    bridge_graph.add_node(GraphVizNode::new("rsm".to_string(),vec![GraphvizNodeStyleItem::FontName("Courier".to_string()),GraphvizNodeStyleItem::Label(rsm.to_ascii_str(false))]));
    bridge_graph.add_node(GraphVizNode::new("grids".to_string(),vec![GraphvizNodeStyleItem::FontName("Courier".to_string()),GraphvizNodeStyleItem::Label(grid_as_str)]));
    bridge_graph.add_node(GraphVizNode::new("rcm".to_string(),vec![GraphvizNodeStyleItem::FontName("Courier".to_string()),GraphvizNodeStyleItem::Label(legit.rcm.to_ascii_str(true))]));
    bridge_graph.add_node(GraphVizNode::new("sm_a".to_string(),vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),GraphvizNodeStyleItem::Image("sm.png".to_string()),GraphvizNodeStyleItem::Label("".to_string())]));
    bridge_graph.add_node(GraphVizNode::new("rsm_a".to_string(),vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),GraphvizNodeStyleItem::Image("rsm.png".to_string()),GraphvizNodeStyleItem::Label("".to_string())]));
    bridge_graph.add_edge(GraphVizEdge ::new("orig".to_string(),
                                             None,
                                             "sm".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("sm".to_string(),
                                             None,
                                             "sm_a".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("sm".to_string(),
                                             None,
                                             "rsm".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("rsm".to_string(),
                                             None,
                                             "rsm_a".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("rsm".to_string(),
                                             None,
                                             "grids".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("grids".to_string(),
                                             None,
                                             "rcm".to_string(),
                                             None,
                                             vec![]));
    bridge_graph.add_edge(GraphVizEdge ::new("rcm".to_string(),
                                             None,
                                             "min".to_string(),
                                             None,
                                             vec![]));

    bridge_graph.print_dot(&parent_folder,
                           &name,
                           &GraphVizOutputFormat::svg);
}


#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use maplit::{hashmap, hashset};
    use crate::nfa::algos::kameda_weiner::algo::{draw_kameda_weiner_process, kameda_weiner_algorithm};
    use crate::nfa::nfa::AutNFA;
    use crate::printers::p_chars::CharAsLetterPrinter;
    use crate::traits::transform::AutTransformable;

    fn get_example() -> AutNFA::<char> {
        let alphabet : HashSet<char> = vec![b'a',b'b'].into_iter().map(char::from).collect();
        let mut transitions: Vec<HashMap<char, HashSet<usize>>> = vec![hashmap!{};3];
        transitions[0].insert('a', hashset!{0,2});
        transitions[0].insert('b', hashset!{1});
        transitions[1].insert('a', hashset!{0});
        transitions[1].insert('b', hashset!{1,2});;
        transitions[2].insert('a', hashset!{0});
        transitions[2].insert('b', hashset!{2});
        AutNFA::<char>::from_raw(alphabet,
                                 hashset!{0},
                                 hashset!{1,2},
                                 transitions).unwrap()
    }

    #[test]
    fn states_map_test1() {

        let parent_folder : Vec<String> = vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect();

        let nfa = get_example();
        let (dfa,sm,rsm,legit) = kameda_weiner_algorithm(&nfa);
        draw_kameda_weiner_process(&parent_folder,&"example".to_string(),&CharAsLetterPrinter{},&nfa,&dfa,&sm,&rsm,&legit);

        let reversed_nfa = nfa.reverse();
        let (dfa,sm,rsm,legit) = kameda_weiner_algorithm(&reversed_nfa);
        draw_kameda_weiner_process(&parent_folder,&"example_reversed".to_string(),&CharAsLetterPrinter{},&reversed_nfa,&dfa,&sm,&rsm,&legit);

    }
}
