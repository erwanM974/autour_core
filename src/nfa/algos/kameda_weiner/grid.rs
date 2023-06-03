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


fn is_grid_prime(states_map : &KwStatesMap, rows : &BTreeSet<usize>, columns : &BTreeSet<usize>) -> bool {
    for row_id in rows {
        let matrix_row = states_map.matrix_map_to_nfa_states.get(*row_id).unwrap();
        for col_id in columns {
            let matrix_cell = matrix_row.get(*col_id).unwrap();
            if !matrix_cell.is_some() {
                return false;
            }
        }
    }
    // ***
    true
}

pub fn search_all_prime_grids(states_map : &KwStatesMap) -> BTreeSet<(BTreeSet<usize>,BTreeSet<usize>)> {
    let mut grids = btreeset!{};
    let mut seen =  btreeset!{};
    let mut queue = vec![];
    {
        let init_rows : BTreeSet<usize> = (0..states_map.rows_map_to_det_states.len()).collect();
        let init_columns : BTreeSet<usize> = (0..states_map.cols_map_to_dual_states.len()).collect();
        queue.push( (init_rows,init_columns) );
    }
    while let Some((next_rows,next_columns)) = queue.pop() {
        seen.insert( (next_rows.clone(),next_columns.clone()) );
        if is_grid_prime(states_map,&next_rows,&next_columns) {
            grids.insert( (next_rows,next_columns) );
        } else {
            for removed_row in &next_rows {
                let mut rows_copy = next_rows.clone();
                rows_copy.remove(removed_row);
                let new = (rows_copy, next_columns.clone());
                if !seen.contains(&new) && !queue.contains(&new) {
                    queue.push(new);
                }
            }
            for removed_column in &next_columns {
                let mut columns_copy = next_columns.clone();
                columns_copy.remove(removed_column);
                let new = (next_rows.clone(),columns_copy);
                if !seen.contains(&new) && !queue.contains(&new) {
                    queue.push(new);
                }
            }
        }
    }
    // ***
    grids
}
