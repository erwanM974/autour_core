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

use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;

use crate::gnfa::gnfa::AutGNFA;
use crate::traits::access::AutAccessible;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, AUT_COLOR_ACCESSIBLE_STATE, AUT_COLOR_ACTIVE_STATE, AUT_COLOR_COACCESSIBLE_STATE, AUT_COLOR_OTHER_STATE, AUT_COLOR_TRIMMED_STATE, AutGraphvizDrawable, ExpBREPrintable};



impl<Letter, Printer> AutGraphvizDrawable<Letter, Printer> for AutGNFA<Letter> where
    Letter : AutLetter,
    Printer : AbstractLanguagePrinter<Letter> {

    fn to_dot(&self,
              draw_accessibility : bool,
              active_states : &HashSet<usize>,
              printer : &Printer) -> GraphVizDiGraph {
        let accessible_states = self.get_all_accessible_states();
        let coaccessible_states = self.get_all_coaccessible_states();
        // ***
        let mut digraph = GraphVizDiGraph::new(vec![]);
        // ***
        for stid in 0..self.states_num {
            let state_name = format!("S{}",stid);
            let shape = match stid {
                _ if stid == self.accept_state => GvNodeShape::DoubleCircle,
                _ => GvNodeShape::Circle
            };
            let mut style = vec![
                GraphvizNodeStyleItem::Shape(shape),
                GraphvizNodeStyleItem::Label(state_name.clone())];
            if draw_accessibility {
                let color = match (accessible_states.contains(&stid),
                                   coaccessible_states.contains(&stid)) {
                    (true,true) => AUT_COLOR_TRIMMED_STATE,
                    (true,false) => AUT_COLOR_ACCESSIBLE_STATE,
                    (false,true) => AUT_COLOR_COACCESSIBLE_STATE,
                    (false,false) => AUT_COLOR_OTHER_STATE
                };
                style.push(GraphvizNodeStyleItem::Color(color));
            }
            if active_states.contains(&stid) {
                style.push(GraphvizNodeStyleItem::FillColor(AUT_COLOR_ACTIVE_STATE))
            }
            // ***
            digraph.add_node(GraphVizNode::new(state_name.clone(),style));
            // ***
            if stid == self.start_state {
                let init_style = vec![
                    GraphvizNodeStyleItem::Shape(GvNodeShape::Point)];
                let init_name = format!("I{}",stid);
                digraph.add_node(GraphVizNode::new(init_name.clone(),init_style));
                // ***
                let edge = GraphVizEdge::new(init_name,
                                             None,
                                             state_name,
                                             None,
                                             vec![]);
                digraph.add_edge(edge);
            }
        }
        // ***
        for ((orig_stid,targ_stid), term) in &self.transitions {
            if !term.is_empty() {
                let orig_name = format!("S{}",orig_stid);
                let targ_name = format!("S{}",targ_stid);
                let term_as_str = term.regexp_to_string( true, printer);
                let edge_style = vec![
                    GraphvizEdgeStyleItem::Label(term_as_str)
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


