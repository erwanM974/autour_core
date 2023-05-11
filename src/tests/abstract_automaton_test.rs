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
use maplit::hashset;

use graphviz_dot_builder::traits::{GraphVizOutputFormat,DotPrintable};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyleItem;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::GraphvizEdgeStyleItem;

use crate::bre::bre::ExpBRE;
use crate::dfa::dfa::AutDFA;
use crate::gnfa::gnfa::AutGNFA;
use crate::nfa::nfa::AutNFA;
use crate::nfait::nfait::AutNFAIT;
use crate::printers::p_chars::CharAsLetterPrinter;

use crate::traits::access::AutAccessible;
use crate::traits::build::AutBuildable;
use crate::traits::letter::AutLetter;
use crate::traits::repr::{AbstractLanguagePrinter, AutGraphvizDrawable, ExpBREPrintable};
use crate::traits::run::AutRunnable;
use crate::traits::transform::AutTransformable;
use crate::traits::translate::AutTranslatable;


pub trait TestAble<Letter : AutLetter>: AutRunnable<Letter>
        + AutAccessible
        + AutBuildable<Letter>
        + AutTransformable<Letter> + Clone
        + AutGraphvizDrawable<Letter, CharAsLetterPrinter>
        + AutTranslatable<Letter>
    where
        CharAsLetterPrinter: AbstractLanguagePrinter<Letter> {}


pub struct AutomatonTestExample<Automaton : TestAble<char>> {
    pub name : String,
    pub automaton : Automaton,
    /// An automaton is *complete* if for all its *states* there are outgoing *transitions* corresponding to every *letter* of the *alphabet*
    pub is_complete : bool,
    /// An automaton is *accessible* if for all its *states* there is a (possibly empty) *path* from a *starting* state to that *state*
    pub is_accessible : bool,
    /// An automaton is *coaccessible* if for all its *states* there is a (possibly empty) *path* from that *state* to a *final state*
    pub is_coaccessible : bool,
    /// An automaton is said *trimmed* if it is *accessible* and *coaccessible*
    pub is_trimmed : bool,
    /// An automaton is said *empty* if it doesn't accept any word
    pub is_empty : bool,
    /// An automaton is said *universal* if it accepts all the words of the language
    pub is_universal : bool,
    /*
    /// The Myhill-Nerode theorem implies that there is a unique minimal DFA for each regular language (Hopcroft and Ullman 1979)
    pub is_minimal : bool,
    to check equality w.r.t. minimization we would need equality up to permutation of indexes of states
    */
    // ***
    pub some_accept_runs : HashSet<Vec<char>>,
    pub some_reject_runs : HashSet<Vec<char>>,
}

impl<Automaton : TestAble<char>> AutomatonTestExample<Automaton> {

    pub fn new(name : String,
               automaton : Automaton,
               is_complete : bool,
               is_accessible : bool,
               is_coaccessible : bool,
               is_trimmed : bool,
               is_empty : bool,
               is_universal : bool,
               some_accept_runs : HashSet<Vec<char>>,
               some_reject_runs : HashSet<Vec<char>>) -> Self {
        AutomatonTestExample{name,
            automaton,
            is_complete,
            is_accessible,
            is_coaccessible,
            is_trimmed,
            is_empty,
            is_universal,
            some_accept_runs,
            some_reject_runs}
    }

    fn get_parent_folder() -> Vec<String> {
        vec!["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_core"].iter().map(|s| s.to_string()).collect()
    }

    pub fn test_translation(&self) {
        // ***
        let as_dfa = self.automaton.to_dfa();
        let as_nfa = self.automaton.to_nfa();
        let as_nfait = self.automaton.to_nfait();
        let as_gnfa = self.automaton.to_gnfa();
        let as_bre = self.automaton.to_bre();
        // ***
        let dfa_graph = as_dfa.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        let nfa_graph = as_nfa.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        let nfait_graph =as_nfait.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        let gnfa_graph = as_gnfa.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        let bre_string = as_bre.regexp_to_string(true,&CharAsLetterPrinter{});
        // ***
        let orig_graph = self.automaton.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        // ***
        let mut bridge_graph = GraphVizDiGraph::new(vec![]);
        bridge_graph.add_cluster(orig_graph.as_cluster("orig".to_string(),vec![],Some("orig".to_string())));
        bridge_graph.add_cluster(dfa_graph.as_cluster("dfa".to_string(),vec![],Some("dfa".to_string())));
        bridge_graph.add_cluster(nfa_graph.as_cluster("nfa".to_string(),vec![],Some("nfa".to_string())));
        bridge_graph.add_cluster(nfait_graph.as_cluster("nfait".to_string(),vec![],Some("nfait".to_string())));
        bridge_graph.add_cluster(gnfa_graph.as_cluster("gnfa".to_string(),vec![],Some("gnfa".to_string())));
        bridge_graph.add_node(GraphVizNode::new("bre".to_string(),vec![GraphvizNodeStyleItem::Label(bre_string)]));
        bridge_graph.add_edge(GraphVizEdge ::new("origS0".to_string(),
                                                 Some("orig".to_string()),
                                                 "dfaS0".to_string(),
                                                 Some("dfa".to_string()),
                                                 vec![GraphvizEdgeStyleItem::Label("to_dfa".to_string())]));
        bridge_graph.add_edge(GraphVizEdge ::new("origS0".to_string(),
                                                 Some("orig".to_string()),
                                                 "nfaS0".to_string(),
                                                 Some("nfa".to_string()),
                                                 vec![GraphvizEdgeStyleItem::Label("to_nfa".to_string())]));
        bridge_graph.add_edge(GraphVizEdge ::new("origS0".to_string(),
                                                 Some("orig".to_string()),
                                                 "nfaitS0".to_string(),
                                                 Some("nfait".to_string()),
                                                 vec![GraphvizEdgeStyleItem::Label("to_nfait".to_string())]));
        bridge_graph.add_edge(GraphVizEdge ::new("origS0".to_string(),
                                                 Some("orig".to_string()),
                                                 "gnfaS0".to_string(),
                                                 Some("gnfa".to_string()),
                                                 vec![GraphvizEdgeStyleItem::Label("to_gnfa".to_string())]));
        bridge_graph.add_edge(GraphVizEdge ::new("origS0".to_string(),
                                                 Some("orig".to_string()),
                                                 "bre".to_string(),
                                                 None,
                                                 vec![GraphvizEdgeStyleItem::Label("to_bre".to_string())]));
        bridge_graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_bridge", &self.name),
                        &GraphVizOutputFormat::svg);
    }

    fn test_characterization(&self) {
        let graph = self.automaton.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),&self.name,&GraphVizOutputFormat::svg);
        // ***
        assert_eq!(self.is_complete,self.automaton.is_complete());
        assert_eq!(self.is_accessible,self.automaton.is_accessible());
        assert_eq!(self.is_coaccessible,self.automaton.is_coaccessible());
        assert_eq!(self.is_trimmed,self.automaton.is_trimmed());
        assert_eq!(self.is_empty,self.automaton.is_empty());
        assert_eq!(self.is_universal,self.automaton.is_universal());
        assert!(self.automaton.contains(&self.automaton));
    }

    fn test_runs(&self) {
        for accepted in &self.some_accept_runs {
            assert!(self.automaton.runs_trace(accepted).unwrap());
        }
        for rejected in &self.some_reject_runs {
            assert!(!self.automaton.runs_trace(rejected).unwrap());
        }
    }

    fn test_negate(&self) {
        let negated = self.automaton.clone().negate();
        let graph = negated.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_negated", &self.name),
                        &GraphVizOutputFormat::svg);
        for accepted in &self.some_accept_runs {
            assert!(!negated.runs_trace(&accepted).unwrap());
        }
        for rejected in &self.some_reject_runs {
            assert!(negated.runs_trace(&rejected).unwrap());
        }
        if self.is_empty {
            assert!(negated.is_universal());
        }
        if self.is_universal {
            assert!(negated.is_empty());
        }
    }

    fn test_reverse(&self) {
        let reversed = self.automaton.clone().reverse();
        let graph = reversed.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_reversed", &self.name),
                        &GraphVizOutputFormat::svg);
        for accepted in &self.some_accept_runs {
            let rev_acc : Vec<char> = accepted.iter().cloned().rev().collect();
            assert!(reversed.runs_trace(&rev_acc).unwrap());
        }
        for rejected in &self.some_reject_runs {
            let rev_rej : Vec<char> = rejected.iter().cloned().rev().collect();
            assert!(!reversed.runs_trace(&rev_rej).unwrap(), "{}", format!("reversed automaton accepted '{:?}'", rev_rej));
        }
    }

    fn test_transform(&self) {
        // ***
        {
            let made_accessible = self.automaton.clone().make_accessible();
            let graph = made_accessible.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
            graph.print_dot(&Self::get_parent_folder(),
                            &format!("{:}_made_accessible", &self.name),
                            &GraphVizOutputFormat::svg);
            assert!(made_accessible.is_accessible());
            assert!(self.automaton.contains(&made_accessible));
            assert!(made_accessible.contains(&self.automaton));

        }
        // ***
        {
            let made_coaccessible = self.automaton.clone().make_coaccessible();
            let graph = made_coaccessible.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
            graph.print_dot(&Self::get_parent_folder(),
                            &format!("{:}_made_coaccessible", &self.name),
                            &GraphVizOutputFormat::svg);
            assert!(made_coaccessible.is_coaccessible());
            assert!(self.automaton.contains(&made_coaccessible));
            assert!(made_coaccessible.contains(&self.automaton));
        }
        // ***
        {
            let made_trimmed = self.automaton.clone().trim();
            let graph = made_trimmed.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
            graph.print_dot(&Self::get_parent_folder(),
                            &format!("{:}_made_trimmed", &self.name),
                            &GraphVizOutputFormat::svg);
            assert!(made_trimmed.is_trimmed());
            assert!(self.automaton.contains(&made_trimmed));
            assert!(made_trimmed.contains(&self.automaton));
        }
        // ***
        {
            let made_complete = self.automaton.clone().complete();
            let graph = made_complete.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
            graph.print_dot(&Self::get_parent_folder(),
                            &format!("{:}_made_complete", &self.name),
                            &GraphVizOutputFormat::svg);
            assert!(made_complete.is_complete());
            assert!(self.automaton.contains(&made_complete));
            assert!(made_complete.contains(&self.automaton));
        }
    }

    fn test_minimize(&self) {
        let minimized = self.automaton.clone().minimize();
        let graph = minimized.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_minimized", &self.name),
                        &GraphVizOutputFormat::svg);
        assert!(self.automaton.contains(&minimized));
        assert!(minimized.contains(&self.automaton));
    }

    pub fn test(&self) {
        self.test_translation();
        self.test_characterization();
        self.test_runs();
        self.test_negate();
        self.test_reverse();
        self.test_minimize();
        self.test_transform();
    }

    fn test_concatenate(&self,other : &Self) {
        let concatenated_automaton = self.automaton.clone()
            .concatenate(other.automaton.clone()).unwrap();
        let graph = concatenated_automaton.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_concat_{:}", &self.name, &other.name),
                        &GraphVizOutputFormat::svg);
        for accepted2 in &other.some_accept_runs {
            let mut concat_accept_runs = HashSet::new();
            for accepted1 in &self.some_accept_runs {
                let mut new_trace = accepted1.clone();
                new_trace.append(&mut accepted2.clone());
                concat_accept_runs.insert(new_trace);
            }
            for concat_accept in concat_accept_runs {
                assert!(concatenated_automaton.runs_trace(&concat_accept).unwrap());
            }
        }
    }

    fn test_unite(&self,other : &Self) {
        let united_automaton = self.automaton.clone()
            .unite(other.automaton.clone()).unwrap();
        let graph = united_automaton.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
        graph.print_dot(&Self::get_parent_folder(),
                        &format!("{:}_unite_{:}", &self.name, &other.name),
                        &GraphVizOutputFormat::svg);
        // ***
        let mut acc_runs = self.some_accept_runs.clone();
        acc_runs.extend(other.some_accept_runs.clone());
        // ***
        for acc_run in &acc_runs {
            assert!(united_automaton.runs_trace(acc_run).unwrap());
        }
        // ***
        let mut rej_runs = vec![];
        for rej1_run in &self.some_reject_runs {
            if !other.automaton.runs_trace(rej1_run).unwrap() {
                rej_runs.push(rej1_run);
            }
        }
        for rej2_run in &other.some_reject_runs {
            if !self.automaton.runs_trace(rej2_run).unwrap() {
                rej_runs.push(rej2_run);
            }
        }
        // ***
        for rej_run in &rej_runs {
            assert!(!united_automaton.runs_trace(rej_run).unwrap());
        }
    }

    fn test_intersect(&self, other : &Self) {
        {
            let intersect_automaton = self.automaton.clone().intersect(other.automaton.clone());
            let graph = intersect_automaton.to_dot(true,&hashset!{},&CharAsLetterPrinter{});
            graph.print_dot(&Self::get_parent_folder(),
                            &format!("{:}_intersect_{:}", &self.name, &other.name),
                            &GraphVizOutputFormat::svg);
            // ***
            let mut acc_by_both = HashSet::new();
            for acc1 in &self.some_accept_runs {
                if other.some_accept_runs.contains(acc1) {
                    acc_by_both.insert(acc1.clone());
                }
            }
            // ***
            for acc in acc_by_both {
                assert!(intersect_automaton.runs_trace(&acc).unwrap());
            }
        }
        // ***
        {
            let other_negated = other.automaton.clone().negate();
            let intersect = self.automaton.clone().intersect(other_negated);
            // ***
            let mut acc_by_both = HashSet::new();
            for acc1 in &self.some_accept_runs {
                if other.some_reject_runs.contains(acc1) {
                    acc_by_both.insert(acc1.clone());
                }
            }
            // ***
            for acc in acc_by_both {
                assert!(intersect.runs_trace(&acc).unwrap());
            }
        }
        // ***
        {
            let other_reversed = other.automaton.clone().reverse();
            let acc_by_other_reversed : HashSet<Vec<char>> =
                other.some_accept_runs.iter().map(|x| x.iter().cloned().rev().collect()).collect();
            let intersect = self.automaton.clone().intersect(other_reversed);
            // ***
            let mut acc_by_both = HashSet::new();
            for acc1 in &self.some_accept_runs {
                if acc_by_other_reversed.contains(acc1) {
                    acc_by_both.insert(acc1.clone());
                }
            }
            // ***
            for acc in acc_by_both {
                assert!(intersect.runs_trace(&acc).unwrap());
            }
        }

    }

    pub fn test_with_other(&self, other : &Self) {
        self.test_concatenate(other);
        self.test_unite(other);
        self.test_intersect(other);
    }
}









