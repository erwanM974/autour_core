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
use maplit::{hashset,hashmap,btreeset};

use crate::bre::term::TermBRE;
use crate::gnfa::gnfa::AutGNFA;
use crate::tests::abstract_automaton_test::{AutomatonTestExample, TestAble};



impl TestAble<char> for AutGNFA<char> {}

pub fn get_gnfa1() -> AutomatonTestExample<AutGNFA<char>> {
    let alphabet : HashSet<char> = vec![b'a',b'b',b'c'].into_iter().map(char::from).collect();
    let mut transitions: HashMap<(usize,usize), TermBRE<char>> = hashmap!{};
    transitions.insert((0,1), TermBRE::Literal(char::from(b'a')));
    transitions.insert((0,2), TermBRE::Literal(char::from(b'c')));
    transitions.insert((1,2),
                       TermBRE::Kleene(Box::new(
                            TermBRE::Union(btreeset!{TermBRE::Literal(char::from(b'a')),TermBRE::Literal(char::from(b'b'))})
                        ))
    );
    let gnfa = AutGNFA::<char>::from_raw(alphabet,
                                     3,
                                     0,2,
                                     transitions).unwrap();
    // ***
    let some_accept_runs = hashset!{vec![char::from(b'c')],
        vec![char::from(b'a')],
        vec![char::from(b'a'),char::from(b'a')],
        vec![char::from(b'a'),char::from(b'b')]
    };
    let some_reject_runs = hashset!{vec![],
        vec![char::from(b'b')],
    };
    AutomatonTestExample::new("gnfa1".to_string(),
                                          gnfa,
                                          false,
                                          true,
                                          true,
                                          true,
                                          false,
                                          false,
                                          some_accept_runs,
                                          some_reject_runs)
}



