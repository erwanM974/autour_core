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


use std::collections::{BTreeSet, HashMap, HashSet};
use maplit::{btreeset, hashmap, hashset};

use crate::dfa::dfa::AutDFA;
use crate::nfa::algos::kameda_weiner::subset_construction::determinize_nfa_and_get_states_map;
use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;
use crate::traits::transform::AutTransformable;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwStatesMap {
    // each row corresponds to a set of states in a DFA
    pub rows_map_to_det_states : Vec<BTreeSet<usize>>,
    // each column corresponds to a set of states in a dual DFA
    pub cols_map_to_dual_states : Vec<BTreeSet<usize>>,
    // each cell in the matrix corresponds to a set of states in an NFA such that
    // there are some that are subsets to a certain corresponding state in the row's DFA states
    // and some others that are subsets to a certain corresponding state in the column's DFA states
    pub matrix_map_to_nfa_states : Vec<Vec<Option<BTreeSet<usize>>>>
}

impl KwStatesMap {

    pub fn new(rows_map_to_det_states: Vec<BTreeSet<usize>>,
               cols_map_to_dual_states: Vec<BTreeSet<usize>>,
               matrix_map_to_nfa_states: Vec<Vec<Option<BTreeSet<usize>>>>) -> Self {
        Self { rows_map_to_det_states, cols_map_to_dual_states, matrix_map_to_nfa_states }
    }

    pub(crate) fn to_ascii_str(&self,is_cover : bool) -> String {

        fn print_cell_states(x : &Option<BTreeSet<usize>>,is_cover : bool) -> String {
            match x {
                None => {
                    "".to_string()
                },
                Some(nfa_states) => {
                    let states : Vec<String> =  if is_cover {
                        // considering a cover matrix
                        nfa_states.iter().map(|x| format!("g{:}", x)).collect()
                    } else {
                        // considering a states map
                        nfa_states.iter().map(|x| format!("s{:}", x)).collect()
                    };
                    format!("{{{:}}}",states.join(","))
                }
            }
        }

        fn print_dfa_states(x : &BTreeSet<usize>, is_dual : bool) -> String {
            let states : Vec<String> = if is_dual {
                x.iter().map(|x| format!("q{:}", x)).collect()
            } else {
                x.iter().map(|x| format!("p{:}", x)).collect()
            };
            format!("{{{:}}}",states.join(","))
        }

        let mut string_table : Vec<Vec<String>> = vec![];
        {
            let mut row_0 = vec!["".to_string()];
            row_0.append(&mut self.cols_map_to_dual_states.iter().map(|x| print_dfa_states(x,true)).collect() );
            string_table.push(row_0)
        }
        for row_id in 0..self.rows_map_to_det_states.len() {
            let mut row = vec![print_dfa_states(self.rows_map_to_det_states.get(row_id).unwrap(),false)];
            row.append(&mut self.matrix_map_to_nfa_states.get(row_id).unwrap()
                .iter()
                .map(|x| print_cell_states(x,is_cover))
                .collect());
            string_table.push(row)
        }
        // ***
        let mut max_col_widths : Vec<usize> = vec![];
        for col_id in 0..=self.cols_map_to_dual_states.len() {
            let mut max_width = 0;
            for str_row in &string_table {
                max_width = max_width.max(str_row.get(col_id).unwrap().len());
            }
            max_col_widths.push(max_width);
        }
        // ***
        let mut as_str = "".to_string();
        for row_strings in string_table {
            for (col_id,cell_string) in row_strings.into_iter().enumerate() {
                let max_width = max_col_widths.get(col_id).unwrap();
                as_str.push_str(&format!("|{:}{:}",cell_string," ".repeat(max_width - cell_string.len())));
            }
            as_str.push_str("|\n");
        }
        // ***
        as_str
    }

    pub(crate) fn from_nfa<Letter : AutLetter>(nfa : &AutNFA<Letter>) -> (Self,AutDFA<Letter>) {
        let (determinized,det_map) = determinize_nfa_and_get_states_map(nfa);
        let (dual,dual_map) = determinize_nfa_and_get_states_map(&nfa.clone().reverse());
        // ***
        let rows_map_to_det_states : Vec<BTreeSet<usize>> = (0..determinized.transitions.len()).map(|x| btreeset!{x}).collect();
        let cols_map_to_dual_states : Vec<BTreeSet<usize>> = (0..dual.transitions.len()).map(|x| btreeset!{x}).collect();
        let mut matrix_map_to_nfa_states : Vec<Vec<Option<BTreeSet<usize>>>> = vec![];
        // ***
        for det_state in 0..determinized.transitions.len() {
            let det_nfa_states = det_map.get(&det_state).unwrap();
            let mut matrix_row = vec![];
            for dual_state in 0..dual.transitions.len() {
                let dual_nfa_states = dual_map.get(&dual_state).unwrap();
                let intersect : BTreeSet<usize> = det_nfa_states.intersection(dual_nfa_states).cloned().collect();
                if intersect.is_empty() {
                    matrix_row.push(None);
                } else {
                    matrix_row.push(Some(intersect));
                }
            }
            matrix_map_to_nfa_states.push(matrix_row);
        }
        // ***
        (Self::new(rows_map_to_det_states,cols_map_to_dual_states,matrix_map_to_nfa_states),determinized)
    }

    fn get_cols_ones(&self) -> HashMap<Vec<bool>,HashSet<usize>> {
        let mut cols_ones = hashmap! {};
        for col_id in 0..self.cols_map_to_dual_states.len() {
            let mut col_ones = vec![];
            for row in &self.matrix_map_to_nfa_states {
                let cell = row.get(col_id).unwrap();
                col_ones.push( cell.is_some() );
            }
            match cols_ones.get_mut(&col_ones) {
                None => {
                    cols_ones.insert( col_ones, hashset!{col_id});
                },
                Some(already) => {
                    already.insert(col_id);
                }
            }
        }
        cols_ones
    }

    fn get_rows_ones(&self) -> HashMap<Vec<bool>,HashSet<usize>> {
        let mut rows_ones = hashmap! {};
        for (row_id,row) in self.matrix_map_to_nfa_states.iter().enumerate() {
            let mut row_ones = vec![];
            for cell in row {
                row_ones.push( cell.is_some() );
            }
            match rows_ones.get_mut(&row_ones) {
                None => {
                    rows_ones.insert( row_ones, hashset!{row_id});
                },
                Some(already) => {
                    already.insert(row_id);
                }
            }
        }
        rows_ones
    }

    fn merge_rows(&self, rows_to_merge : &HashSet<usize>) -> Self {
        let mut sorted_rows_to_merge = rows_to_merge.iter().cloned().collect::<Vec<usize>>();
        sorted_rows_to_merge.sort();
        // ***
        let mut modified_row_det_states = btreeset! {};
        let mut modified_row_in_matrix = vec![None;self.cols_map_to_dual_states.len()];
        for to_merge_row in &sorted_rows_to_merge {
            let row_det_states = self.rows_map_to_det_states.get(*to_merge_row).unwrap();
            modified_row_det_states.append(&mut row_det_states.clone());
            let row_in_matrix = self.matrix_map_to_nfa_states.get(*to_merge_row).unwrap();
            for (col_id, cell) in row_in_matrix.iter().enumerate() {
                if let Some(nfa_states) = cell {
                    let in_new_row = modified_row_in_matrix.get_mut(col_id).unwrap();
                    match in_new_row {
                        None => {
                            *in_new_row = Some(nfa_states.clone());
                        },
                        Some(already) => {
                            already.append(&mut nfa_states.clone());
                        }
                    }
                }
            }
        }
        // ***
        let mut new_rows_map_to_det_states = vec![];
        let mut new_matrix_map_to_nfa_states= vec![];
        for row_id in 0..self.matrix_map_to_nfa_states.len() {
            if sorted_rows_to_merge.contains(&row_id) {
                if &row_id == sorted_rows_to_merge.first().unwrap() {
                    // need clone for compiler but should only happen once
                    new_rows_map_to_det_states.push(modified_row_det_states.clone());
                    new_matrix_map_to_nfa_states.push(modified_row_in_matrix.clone());
                }
            } else {
                new_rows_map_to_det_states.push(self.rows_map_to_det_states.get(row_id).unwrap().clone());
                new_matrix_map_to_nfa_states.push(self.matrix_map_to_nfa_states.get(row_id).unwrap().clone());
            }
        }
        // ***
        Self::new(new_rows_map_to_det_states,self.cols_map_to_dual_states.clone(),new_matrix_map_to_nfa_states)
    }

    fn merge_cols(&self, cols_to_merge : &HashSet<usize>) -> Self {
        let mut sorted_cols_to_merge = cols_to_merge.iter().cloned().collect::<Vec<usize>>();
        sorted_cols_to_merge.sort();
        // ***
        let mut modified_col_dual_states = btreeset!{};
        for col_id in &sorted_cols_to_merge {
            modified_col_dual_states.append(&mut self.cols_map_to_dual_states.get(*col_id).unwrap().clone());
        }
        // ***
        let mut new_cols_map_to_dual_states = vec![];
        for (col_id,dual_states) in self.cols_map_to_dual_states.iter().enumerate() {
            if sorted_cols_to_merge.contains(&col_id) {
                if &col_id == sorted_cols_to_merge.first().unwrap() {
                    // need clone for compiler but should only happen once
                    new_cols_map_to_dual_states.push(modified_col_dual_states.clone());
                }
            } else {
                new_cols_map_to_dual_states.push(dual_states.clone());
            }
        }
        // ***
        let mut new_matrix_map_to_nfa_states= vec![];
        for row in &self.matrix_map_to_nfa_states {
            let mut new_row = vec![];
            let mut newcol_cell : Option<BTreeSet<usize>> = None;
            for (col_id,cell) in row.iter().enumerate().rev() {
                if sorted_cols_to_merge.contains(&col_id) {
                    if let Some(to_add) = cell {
                        if let Some(already) = newcol_cell {
                            let mut new_content = already;
                            new_content.append(&mut to_add.clone());
                            newcol_cell = Some(new_content);
                        } else {
                            newcol_cell = Some(to_add.clone());
                        }
                    }
                    if &col_id == sorted_cols_to_merge.first().unwrap() {
                        // need clone for compiler but should only happen once
                        new_row.insert(0,newcol_cell.clone());
                    }
                } else {
                    new_row.insert(0,cell.clone());
                }
            }
            new_matrix_map_to_nfa_states.push(new_row);
        }
        // ***
        Self::new(self.rows_map_to_det_states.clone(),new_cols_map_to_dual_states,new_matrix_map_to_nfa_states)
    }

    pub fn reduce_matrix(&self) -> Self {
        // ***
        let row_ones = self.get_rows_ones();
        for rows_to_merge in row_ones.values() {
            if rows_to_merge.len() > 1 {
                let merged = self.merge_rows(rows_to_merge);
                return merged.reduce_matrix();
            }
        }
        // ***
        let col_ones = self.get_cols_ones();
        for cols_to_merge in col_ones.values() {
            if cols_to_merge.len() > 1 {
                let merged = self.merge_cols(cols_to_merge);
                return merged.reduce_matrix();
            }
        }
        // ***
        self.clone()
    }

}







