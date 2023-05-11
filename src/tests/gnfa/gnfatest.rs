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



use crate::tests::gnfa::ex1::get_gnfa1;
use crate::traits::repr::AutGraphvizDrawable;
use crate::traits::translate::AutTranslatable;

#[test]
fn gnfa_tests() {
    let examples = vec![get_gnfa1()];
    for gnfa1 in &examples {
        println!("test");
        gnfa1.test_translation();
        for gnfa2 in &examples {
            //gnfa1.test_with_other(gnfa2);
        }
    }
}
