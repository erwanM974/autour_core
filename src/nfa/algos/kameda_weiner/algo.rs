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



use std::collections::{BTreeSet};
use itertools::Itertools;

use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};
use maplit::{btreeset, hashset};
use crate::dfa::dfa::AutDFA;

use crate::nfa::algos::kameda_weiner::cover::{is_set_of_grids_covering_matrix, replace_states_map_content_with_cover};
use crate::nfa::algos::kameda_weiner::grid::search_maximal_prime_grids;
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
            -> (AutDFA<Letter>,KwStatesMap,KwStatesMap,Option<KwLegitCandidate<Letter>>) {
    let (sm,dfa) = KwStatesMap::from_nfa(&nfa);
    //println!("got initial state matrix of {:} rows and {:} columns", sm.rows_map_to_det_states.len(), sm.cols_map_to_dual_states.len());
    // ***
    // the number of states of the minimal NFA is
    // smaller than min( |NFA|, |minDFA|, |minDFAdual| ) where
    //      |NFA| is the number of states of the original NFA
    //      |minDFA| is the number of states of the minimal DFA obtained from minimizing the determinization of the NFA
    //      |minDFAdual| is the number of states of the minimal DFA obtained from minimizing the determinization of the dual of the NFA
    // we will search for a candidate with at worst this number of states
    let mut num_states_criterion = [nfa.transitions.len(),
                                            sm.rows_map_to_det_states.len(),
                                            sm.cols_map_to_dual_states.len()].iter().min().unwrap() + 1;
    let mut candidate : Option<KwLegitCandidate<Letter>> = None;
    // ***
    let rsm = sm.reduce_matrix();
    //println!("got reduced state matrix of {:} rows and {:} columns", rsm.rows_map_to_det_states.len(), rsm.cols_map_to_dual_states.len());
    // we store prime grids in a VEC and use their index in the vec as reference
    let prime_grids : Vec<(BTreeSet<usize>,BTreeSet<usize>)> = search_maximal_prime_grids(&rsm).into_iter().collect();
    //println!("got {:} prime grids", prime_grids.len());
    // ***
    // here the HashSet<usize> is the set of indexes of the selected grids within "prime_grids"
    let mut seen : BTreeSet < BTreeSet<usize> > = btreeset!{};
    // the number of states of the minimal NFA is
    // greater than partie_entiere(log_2(|minDFA|))
    // where |minDFA| is the number of states of the minimal DFA obtained from minimizing the determinization of the NFA
    let theoretical_min_state_num = sm.rows_map_to_det_states.len().checked_ilog2().unwrap() as usize;
    // we will hence start looking for minimum covers of this number of grids
    // using itertools combinations of size "theoretical_min_state_num"
    let mut queue: Vec< BTreeSet<usize> > =vec![];
    let prime_grid_indices : BTreeSet<usize> = (0..prime_grids.len()).into_iter().collect();
    let mut initial_combinator = prime_grid_indices.into_iter().combinations(theoretical_min_state_num);
    // ***
    while let Some(next_cover_candidate) = match initial_combinator.next() {
        None => {queue.pop()},
        Some(indices_as_vec) => {Some(indices_as_vec.into_iter().collect())}
    } {
        seen.insert(next_cover_candidate.clone());
        if next_cover_candidate.len() >= num_states_criterion {
            continue
        }
        let candidate_grids : BTreeSet<&(BTreeSet<usize>,BTreeSet<usize>)> =
            next_cover_candidate.iter().map(|id| prime_grids.get(*id).unwrap()).collect();
        if is_set_of_grids_covering_matrix(&rsm,&candidate_grids) {
            let rcm = replace_states_map_content_with_cover(&rsm,&candidate_grids);
            let rcm_as_nfa = convert_states_map_to_nfa(&rcm,&dfa,next_cover_candidate.len());
            if nfa.equals(&rcm_as_nfa) {
                num_states_criterion = rcm_as_nfa.transitions.len();
                let cloned_grids : BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> =
                    candidate_grids.into_iter().cloned().collect();
                candidate = Some(KwLegitCandidate::new(cloned_grids,rcm,rcm_as_nfa));
            }
        } else {
            for grid_id in 0..prime_grids.len() {
                if !next_cover_candidate.contains(&grid_id) {
                    let mut new_candidate = next_cover_candidate.clone();
                    new_candidate.insert(grid_id);
                    if !seen.contains(&new_candidate) && !queue.contains(&new_candidate) {
                        queue.push(new_candidate);
                    }
                }
            }
        }
    }
    // ***
    (dfa,sm,rsm, candidate)
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
    use std::time::Instant;
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
        transitions[1].insert('b', hashset!{1,2});
        transitions[2].insert('a', hashset!{0});
        transitions[2].insert('b', hashset!{2});
        AutNFA::<char>::from_raw(alphabet,
                                 hashset!{0},
                                 hashset!{1,2},
                                 transitions).unwrap()
    }

    fn get_bigger_example() -> AutNFA::<char> {
        let alphabet : HashSet<char> = vec![b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h'].into_iter().map(char::from).collect();
        let mut transitions: Vec<HashMap<char, HashSet<usize>>> = vec![hashmap!{};10];
        transitions[0].insert('a', hashset!{1});
        transitions[0].insert('e', hashset!{2});
        transitions[1].insert('b', hashset!{3});
        transitions[3].insert('c', hashset!{6});
        transitions[6].insert('d', hashset!{0});
        transitions[2].insert('f', hashset!{4});
        transitions[2].insert('g', hashset!{5});
        transitions[4].insert('g', hashset!{7});
        transitions[5].insert('f', hashset!{7});
        transitions[5].insert('h', hashset!{8});
        transitions[7].insert('h', hashset!{9});
        transitions[8].insert('f', hashset!{9});
        AutNFA::<char>::from_raw(alphabet,
                                 hashset!{0},
                                 hashset!{9},
                                 transitions).unwrap()
    }

    #[test]
    fn perf_test1() {
        let nfa = get_bigger_example();
        println!("testing Kameda-Weiner on a NFA with {:} states with alphabet of {:} characters", nfa.transitions.len(), nfa.alphabet.len());
        let now = Instant::now();
        let (dfa,sm,rsm,legit) = kameda_weiner_algorithm(&nfa);
        let elapsed = now.elapsed();
        let new_num_states = match legit {
            None => {nfa.transitions.len()},
            Some(cand) => {
                println!("got candidate");
                cand.nfa.transitions.len()
            }
        };
        println!("performed KW from NFA with {:} states in {:}μs to get a NFA with {:} states", nfa.transitions.len(), elapsed.as_micros(), new_num_states);
    }

    #[test]
    fn states_map_test1() {

        let parent_folder : Vec<String> = vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect();

        let nfa = get_example();
        let (dfa,sm,rsm,legit) = kameda_weiner_algorithm(&nfa);
        draw_kameda_weiner_process(&parent_folder,&"example".to_string(),&CharAsLetterPrinter{},&nfa,&dfa,&sm,&rsm,&legit.unwrap());

        let reversed_nfa = nfa.reverse();
        let (dfa,sm,rsm,legit) = kameda_weiner_algorithm(&reversed_nfa);
        draw_kameda_weiner_process(&parent_folder,&"example_reversed".to_string(),&CharAsLetterPrinter{},&reversed_nfa,&dfa,&sm,&rsm,&legit.unwrap());

    }
}
