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


use crate::tests::dfa::ex1_abc::get_dfa1;

#[test]
fn dfa_tests() {
    let examples = vec![get_dfa1()];
    for dfa1 in &examples {
        dfa1.test();
        for dfa2 in &examples {
            dfa1.test_with_other(dfa2);
        }
    }
}
