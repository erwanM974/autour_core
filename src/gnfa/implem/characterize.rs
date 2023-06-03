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

use crate::gnfa::gnfa::AutGNFA;
use crate::traits::characterize::AutCharacterizable;
use crate::traits::letter::AutLetter;
use crate::traits::translate::AutTranslatable;


impl<Letter: AutLetter> AutCharacterizable<Letter> for AutGNFA<Letter> {

    fn is_complete(&self) -> bool {
        self.to_nfa().is_complete()
    }

    fn is_empty(&self) -> bool {
        self.to_nfa().is_empty()
    }

    fn is_universal(&self) -> bool {
        self.to_nfa().is_universal()
    }

    fn contains(&self,
                other: &Self) -> bool {
        self.to_nfa().contains(&other.to_nfa())
    }
}
