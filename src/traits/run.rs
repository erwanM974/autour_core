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

use std::fmt;
use std::collections::HashSet;
use crate::traits::error::AutError;
use crate::traits::letter::AutLetter;


pub trait AutRunnable<Letter: AutLetter> {
    fn runs_trace(&self, trace: &[Letter])
        -> Result<bool,AutError<Letter>>;
    fn run_transition(&self, initial_states : &HashSet<usize>, letter: &Letter)
        -> Result<HashSet<usize>,AutError<Letter>>;
}