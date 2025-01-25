/*
Copyright 2024 Erwan Mahe (github.com/erwanM974)

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




use crate::core::internal_representation::*;

#[derive(Debug,Clone)]
pub struct MinimalLangCioII {}


#[derive(Debug,Clone)]
pub enum MinimalLeafPattern {
    EMPTY,
    BROADCAST(MinimalBroadcastLeafPattern)
}


#[derive(Debug,Clone)]
pub struct MinimalBroadcastLeafPattern {
    pub origin_lf_id : Option<usize>,
    pub msg_id : usize,
    // we use a Vec instead of HashSet so that it is deterministic (we always have the same order when iterating)
    pub targets : Vec<usize>
}

impl MinimalBroadcastLeafPattern {
    pub fn new(origin_lf_id: Option<usize>, msg_id: usize, targets: Vec<usize>) -> Self {
        Self { origin_lf_id, msg_id, targets }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum MinimalOperators {
    Strict,Seq,Alt,Par,Loop
}

impl MinimalOperators {

    pub fn as_lowercase_string(&self) -> String {
        match self {
            MinimalOperators::Alt => {
                "alt"
            },
            MinimalOperators::Strict => {
                "strict"
            },
            MinimalOperators::Seq => {
                "seq"
            },
            MinimalOperators::Par => {
                "par"
            },
            MinimalOperators::Loop=> {
                "loop"
            }
        }.to_owned()
    }
    
}

impl InteractionOperatorRepresentation for MinimalOperators {

    fn arity(&self) -> usize {
        match &self {
            MinimalOperators::Strict => 2,
            MinimalOperators::Seq => 2,
            MinimalOperators::Alt => 2,
            MinimalOperators::Par => 2,
            MinimalOperators::Loop => 1
        }
    }

    fn is_associative(&self) -> bool {
        match &self {
            MinimalOperators::Strict => true,
            MinimalOperators::Seq => true,
            MinimalOperators::Alt => true,
            MinimalOperators::Par => true,
            MinimalOperators::Loop => false
        }
    }
}


impl CommonIoInteractionInterface for MinimalLangCioII {
    type InteractionLeafPatternType = MinimalLeafPattern;
    type InteractionOperatorType = MinimalOperators;
}