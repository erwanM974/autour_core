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

use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GvEdgeLineStyle,GraphvizEdgeStyleItem};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;

use crate::nfait::nfait::AutNFAIT;
use crate::traits::access::AutAccessible;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, AUT_COLOR_ACCESSIBLE_STATE, AUT_COLOR_ACTIVE_STATE, AUT_COLOR_COACCESSIBLE_STATE, AUT_COLOR_OTHER_STATE, AUT_COLOR_TRIMMED_STATE, AutGraphvizDrawable};


impl<Letter, Printer> AutGraphvizDrawable<Letter, Printer> for AutNFAIT<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn to_dot(&self,
              draw_accessibility : bool,
              active_states : &HashSet<usize>) -> GraphVizDiGraph {
        let accessible_states = self.get_all_accessible_states();
        let coaccessible_states = self.get_all_coaccessible_states();
        // ***
        let mut digraph = GraphVizDiGraph::new(vec![]);
        // ***
        for (orig_stid, transitions_map) in self.transitions.iter().enumerate() {
            let shape = match self.finals.contains(&orig_stid) {
                true => GvNodeShape::DoubleCircle,
                false => GvNodeShape::Circle
            };
            // ***
            let orig_name = format!("S{}",orig_stid);
            let mut style = vec![
                GraphvizNodeStyleItem::Shape(shape),
                GraphvizNodeStyleItem::Label(orig_name.clone())];
            // ***
            if draw_accessibility {
                let color = match (accessible_states.contains(&orig_stid),
                                   coaccessible_states.contains(&orig_stid)) {
                    (true,true) => AUT_COLOR_TRIMMED_STATE,
                    (true,false) => AUT_COLOR_ACCESSIBLE_STATE,
                    (false,true) => AUT_COLOR_COACCESSIBLE_STATE,
                    (false,false) => AUT_COLOR_OTHER_STATE
                };
                style.push(GraphvizNodeStyleItem::Color(color));
            }
            if active_states.contains(&orig_stid) {
                style.push(GraphvizNodeStyleItem::FillColor(AUT_COLOR_ACTIVE_STATE))
            }
            // ***
            digraph.add_node(GraphVizNode::new(orig_name.clone(),style));
            // ***
            if self.initials.contains(&orig_stid) {
                let init_style = vec![
                    GraphvizNodeStyleItem::Shape(GvNodeShape::Point)];
                let init_name = format!("I{}",orig_stid);
                digraph.add_node(GraphVizNode::new(init_name.clone(),init_style));
                // ***
                let edge = GraphVizEdge::new(init_name,
                                             None,
                                             orig_name.clone(),
                                             None,
                                             vec![]);
                digraph.add_edge(edge);
            }
            for (letter, target_states) in transitions_map {
                for targ_stid in target_states {
                    let targ_name = format!("S{}",targ_stid);
                    let edge_style = vec![GraphvizEdgeStyleItem::Label(Printer::get_letter_string_repr(letter))];
                    let edge = GraphVizEdge::new(orig_name.clone(),
                                                 None,
                                                 targ_name,
                                                 None,
                                                 edge_style);
                    digraph.add_edge(edge);
                }
            }
        }
        // ***
        for (orig_stid,targets) in self.epsilon_trans.iter().enumerate() {
            let orig_name = format!("S{}",orig_stid);
            for targ_stid in targets {
                let targ_name = format!("S{}",targ_stid);
                let edge_style = vec![
                    GraphvizEdgeStyleItem::Label(Printer::get_epsilon_symbol(true).to_string()),
                    GraphvizEdgeStyleItem::LineStyle(GvEdgeLineStyle::Dashed)
                ];
                let edge = GraphVizEdge::new(orig_name.clone(),
                                             None,
                                             targ_name,
                                             None,
                                             edge_style);
                digraph.add_edge(edge);
            }
        }
        // ***
        digraph
    }

}


/*


    fn to_dot(&self) -> String {
        let mut ret_str = String::new();
        ret_str.push_str("digraph {");
        // ***
        if !self.finals.is_empty() {
            ret_str.push_str("    node [shape = doublecircle];");
            for e in &self.finals {
                ret_str.push_str(&format!(" S_{}", e));
            }
            ret_str.push_str(";");
        }
        // ***
        if !self.initials.is_empty() {
            ret_str.push_str("    node [shape = point];");
            for e in &self.initials {
                ret_str.push_str(&format!(" I_{}", e));
            }
            ret_str.push_str(";");
        }
        // ***
        ret_str.push_str("    node [shape = circle];");
        let mut tmp_map = HashMap::new();
        for (origin_state_id, outgoing_transitions_map) in self.transitions.iter().enumerate() {
            if outgoing_transitions_map.is_empty() {
                if !self.finals.contains(&origin_state_id) {
                    ret_str.push_str(&format!("    S_{};", &origin_state_id));
                }
            }
            for (letter, target_states) in outgoing_transitions_map {
                for target_id in target_states {
                    tmp_map.entry(target_id).or_insert_with(Vec::new).push(letter);
                }
            }
            for (target_state_id, letters_vec) in tmp_map.drain() {
                let letters_as_str : Vec<String> = letters_vec.iter().map(|l| l.to_string()).collect();
                let transition_str = format!("{}", letters_as_str.join(",") );
                ret_str.push_str(&format!("    S_{} -> S_{} [label = \"{}\"];",
                                          origin_state_id,
                                          target_state_id,
                                          transition_str));
            }
        }
        // ***
        for (origin_state_id,targets) in self.epsilon_trans.iter().enumerate() {
            for target_state_id in targets {
                ret_str.push_str(&format!("    S_{} -> S_{} [dashed,label = \"ε\"];",
                                          origin_state_id,
                                          target_state_id));
            }
        }
        // ***
        for initial_state_id in &self.initials {
            ret_str.push_str(&format!("    I_{0} -> S_{0};", initial_state_id));
        }
        // ***
        ret_str.push_str("}");
        // ***
        return ret_str;
    }

*/
