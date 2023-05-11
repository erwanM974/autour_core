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
use std::hash::Hash;
use maplit::{hashset,hashmap};
use crate::dfa::dfa::AutDFA;
use crate::tests::abstract_automaton_test::{AutomatonTestExample, TestAble};



impl TestAble<char> for AutDFA<char> {}

pub fn get_dfa1() -> AutomatonTestExample<AutDFA<char>> {
    let alphabet : HashSet<char> = vec![b'a',b'b',b'c'].into_iter().map(char::from).collect();
    let mut transitions: Vec<HashMap<char, usize>> = vec![hashmap!{};4];
    transitions[0].insert('a', 1);
    transitions[1].insert('b', 2);
    transitions[2].insert('c', 3);
    let dfa1 = AutDFA::<char>::from_raw(alphabet,
                                     0,
                                     hashset!{3},
                                     transitions).unwrap();
    // ***
    let some_accept_runs = hashset!{vec![char::from(b'a'),char::from(b'b'),char::from(b'c')]};
    let some_reject_runs = hashset!{vec![],
        vec![char::from(b'a')],
        vec![char::from(b'b')],
        vec![char::from(b'c')],
        vec![char::from(b'a'),char::from(b'a')],
        vec![char::from(b'a'),char::from(b'b')],
        vec![char::from(b'a'),char::from(b'c')],
        vec![char::from(b'b'),char::from(b'a')],
        vec![char::from(b'b'),char::from(b'b')],
        vec![char::from(b'b'),char::from(b'c')],
        vec![char::from(b'c'),char::from(b'a')],
        vec![char::from(b'c'),char::from(b'b')],
        vec![char::from(b'c'),char::from(b'c')],
    };
    let dfat1 = AutomatonTestExample::new("dfa1".to_string(),
                                          dfa1,
                                          false,
                                          true,
                                          true,
                                          true,
                                          false,
                                          false,
                                          some_accept_runs,
                                          some_reject_runs);
    return dfat1;
}



