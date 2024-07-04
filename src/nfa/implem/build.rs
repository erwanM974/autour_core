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
use std::ops::RangeBounds;
use std::ops::Bound::{Included,Excluded,Unbounded};
use crate::nfa::nfa::AutNFA;
use crate::traits::letter::AutLetter;
use crate::traits::build::AutBuildable;
use crate::traits::error::AutError;
use crate::utils::{shift_hashmaps_of_hashset_usize};


impl<Letter: AutLetter> AutBuildable<Letter> for AutNFA<Letter> {
    fn unite(mut self, other: Self) -> Result<Self,AutError<Letter>> {
        if self.alphabet != other.alphabet {
            return Err(AutError::OperationOnLanguagesOverDifferentAlphabets(self.alphabet,
                                                                            other.alphabet));
        }
        let l = self.transitions.len();
        // ***
        self.initials.extend(other.initials.into_iter().map(|x| x + l));
        self.finals.extend(other.finals.into_iter().map(|x| x + l));
        // ***
        let mut transitions = other.transitions;
        shift_hashmaps_of_hashset_usize(&mut transitions, l);
        self.transitions.append(&mut transitions);
        // ***
        Ok(self)
    }

    fn concatenate(mut self, mut other: Self) -> Result<Self,AutError<Letter>> {
        if self.alphabet != other.alphabet {
            return Err(AutError::OperationOnLanguagesOverDifferentAlphabets(self.alphabet,
                                                                            other.alphabet));
        }
        // we merge the final states of self with the initial states of other
        let l = self.transitions.len();
        other.shift_nfa(l);
        // ***
        for other_nfa_init_stid in &other.initials {
            for (letter, targets) in &mut other.transitions[other_nfa_init_stid - l] {
                // we substract l because of the shift above
                for f in &self.finals {
                    self.transitions[*f]
                        .entry(*letter)
                        .or_insert_with(HashSet::new)
                        .extend(targets.iter());
                }
            }
        }
        // ***
        if other.finals.is_disjoint(&other.initials) {
            self.finals = other.finals;
        } else {
            // because self.finals are all unified with elements in other.initials
            self.finals.extend(other.finals.into_iter());
        }
        self.transitions.append(&mut other.transitions);
        // ***
        Ok(self)
    }

    fn repeat(self, num : usize) -> Self {
        (0..num)
            .fold(AutNFA::new_empty_word(self.alphabet.clone()),
                  |acc, _| acc.concatenate(self.clone()).unwrap()
            )
    }

    fn kleene(mut self) -> Self {
        // because we must be able to express zero times the sub-automaton
        // we add a state that is both final and initial
        // and will also be the anchor for the loop
        let l = self.transitions.len();
        let mut outgoing_from_initials = HashMap::new();
        // ***
        for init_stid in &self.initials {
            for (letter, targets) in &self.transitions[*init_stid] {
                let out_with_letter = outgoing_from_initials.entry(*letter).or_insert_with(HashSet::new);
                out_with_letter.extend( targets)
            }
        }
        // ***
        // for all final states
        for final_stid in &self.finals {
            // for all transitions from initials
            for (letter, targets) in &outgoing_from_initials {
                let outgoing_from_final_with_letter : &mut HashSet<usize> = self.transitions[*final_stid]
                    .entry(*letter)
                    .or_insert_with(HashSet::new);
                // add transitions from final states to targets of initials
                outgoing_from_final_with_letter.extend(targets);
            }
        }
        // ***
        self.transitions.push(
            outgoing_from_initials.into_iter()
                .map(|(k, v)| (k, v.into_iter().collect()))
                .collect(),
        );
        // remove all initials and replace with our anchor state
        self.initials.clear();
        self.initials.insert(l);
        // add the anchor state as a final state
        self.finals.insert(l);
        // ***
        self
    }

    fn at_most(mut self, num: usize) -> Self {
        // if no initial state is also final then add one in order to accept the empty word
        if !self.initials.iter().any(|x| self.finals.contains(x)) {
            let l = self.transitions.len();
            self.initials.insert(l);
            self.finals.insert(l);
            self.transitions.push(HashMap::new());
        }
        // ***
        self.repeat(num)
    }

    fn at_least(self, num: usize) -> Self {
        self.clone().repeat(num).concatenate(self.kleene()).unwrap()
    }

    fn repeat_range<R: RangeBounds<usize>>(self, r: R) -> Self {
        let start = match r.start_bound() {
            Included(&a) => a,
            Excluded(&a) => a + 1,
            Unbounded => 0,
        };
        // ***
        let end_opt = match r.end_bound() {
            Included(&a) => Some(a),
            Excluded(&a) => Some(a - 1),
            Unbounded => None,
        };
        // ***
        match end_opt {
            None => {
                self.at_least(start)
            },
            Some(end) => {
                if end < start {
                    panic!()
                } else {
                    self.clone().repeat(start).concatenate(self.at_most(end - start)).unwrap()
                }
            }
        }
    }
}


