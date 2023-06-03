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
use maplit::btreeset;

use crate::nfa::algos::kameda_weiner::states_map::KwStatesMap;


pub fn is_set_of_grids_covering_matrix(states_map : &KwStatesMap,
                                       set_of_grids : &BTreeSet<&(BTreeSet<usize>,BTreeSet<usize>)>) -> bool {
    for (row_id,matrix_row) in states_map.matrix_map_to_nfa_states.iter().enumerate() {
        for (col_id, matrix_cell) in matrix_row.iter().enumerate() {
            // if there is a state intersection in the cell then..
            if matrix_cell.is_some() {
                // ..there must be a grid in which there is row and column
                let mut found_grid = false;
                for (grid_rows,grid_cols) in set_of_grids {
                    if grid_rows.contains(&row_id) && grid_cols.contains(&col_id) {
                        found_grid = true;
                        break
                    }
                }
                // ***
                if !found_grid {
                    return false;
                }
            }
        }
    }
    // ***
    true
}


pub fn replace_states_map_content_with_cover(states_map : &KwStatesMap,
                                             set_of_grids : &BTreeSet<&(BTreeSet<usize>,BTreeSet<usize>)>) -> KwStatesMap {
    // ***
    // structure set of grids into vec to give them unique ids
    let grids_as_vec : Vec<&(BTreeSet<usize>,BTreeSet<usize>)> = set_of_grids.iter().cloned().collect();
    // ***
    let new_rows_map_to_det_states = states_map.rows_map_to_det_states.clone();
    let new_cols_map_to_dual_states = states_map.cols_map_to_dual_states.clone();
    let mut new_matrix = vec![];
    for row_id in 0..new_rows_map_to_det_states.len() {
        let mut new_row = vec![];
        for col_id in 0..new_cols_map_to_dual_states.len() {
            let mut new_cell = btreeset!{};
            for (grid_id,(grid_rows,grid_cols)) in grids_as_vec.iter().enumerate() {
                if grid_rows.contains(&row_id) && grid_cols.contains(&col_id) {
                    new_cell.insert( grid_id);
                }
            }
            if new_cell.is_empty() {
                new_row.push(None);
            } else {
                new_row.push(Some(new_cell));
            }
        }
        new_matrix.push(new_row);
    }
    // ***
    KwStatesMap::new(new_rows_map_to_det_states,new_cols_map_to_dual_states,new_matrix)
}
