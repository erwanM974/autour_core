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





fn is_grid_prime(states_map : &KwStatesMap, grid : &(BTreeSet<usize>, BTreeSet<usize>)) -> bool {
    for row_id in &grid.0 {
        let matrix_row = states_map.matrix_map_to_nfa_states.get(*row_id).unwrap();
        for col_id in &grid.1 {
            let matrix_cell = matrix_row.get(*col_id).unwrap();
            if !matrix_cell.is_some() {
                return false;
            }
        }
    }
    // ***
    true
}

fn is_grid_covered_by(small_grid : &(BTreeSet<usize>, BTreeSet<usize>),
                      big_grid : &(BTreeSet<usize>, BTreeSet<usize>)) -> bool {
    small_grid.0.is_subset(&big_grid.0) && small_grid.1.is_subset(&big_grid.1)
}

fn is_grid_covered_by_element_of_set(small_grid : &(BTreeSet<usize>, BTreeSet<usize>),
                                     big_grids : &BTreeSet<(BTreeSet<usize>, BTreeSet<usize>)>) -> bool {
    for big_grid in big_grids {
        if is_grid_covered_by(small_grid,big_grid) {
            return true;
        }
    }
    false
}

pub fn search_maximal_prime_grids(states_map : &KwStatesMap) -> BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> {
    let mut grids : BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> = btreeset!{};
    let mut seen =  btreeset!{};
    let mut queue = vec![];
    {
        let init_rows : BTreeSet<usize> = (0..states_map.rows_map_to_det_states.len()).collect();
        let init_columns : BTreeSet<usize> = (0..states_map.cols_map_to_dual_states.len()).collect();
        queue.push( (init_rows,init_columns) );
    }
    while let Some(new_grid_candidate) = queue.pop() {
        seen.insert( new_grid_candidate.clone() );
        if is_grid_covered_by_element_of_set(&new_grid_candidate,&grids) {
            continue
        }
        if is_grid_prime(states_map,&new_grid_candidate) {
            // remove all previously discovered grids
            // that are strictly covered by the new grid
            grids = grids.into_iter()
                .filter(|old_grid| !is_grid_covered_by(old_grid,&new_grid_candidate))
                .collect();
            grids.insert( new_grid_candidate );
        } else {
            for removed_row in &new_grid_candidate.0 {
                let mut rows_copy = new_grid_candidate.0.clone();
                rows_copy.remove(removed_row);
                let new = (rows_copy, new_grid_candidate.1.clone());
                if !seen.contains(&new) && !queue.contains(&new) {
                    queue.push(new);
                }
            }
            for removed_column in &new_grid_candidate.1 {
                let mut columns_copy = new_grid_candidate.1.clone();
                columns_copy.remove(removed_column);
                let new = (new_grid_candidate.0.clone(),columns_copy);
                if !seen.contains(&new) && !queue.contains(&new) {
                    queue.push(new);
                }
            }
        }
    }
    // ***
    grids
}
