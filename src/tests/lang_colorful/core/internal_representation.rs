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


use crate::internal_representation::*;
use crate::tests::lang_colorful::colorful_lang::*;


pub struct ColorfulLangCioII {}


#[derive(Debug,Clone)]
pub struct ColorfulLeafPattern {
    pub origin : ColorfulAction,
    pub msg_id : usize,
    pub target : ColorfulAction
}

impl ColorfulLeafPattern {
    pub fn new(origin : ColorfulAction, msg_id : usize, target: ColorfulAction) -> Self {
        Self { origin, msg_id, target }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ColorfulOperators {
    TPC,
    Rougail(String),
    Brocoli,
    Coreg(Option<usize>) 
}

impl InteractionOperatorRepresentation for ColorfulOperators {

    fn arity(&self) -> usize {
        match &self {
            ColorfulOperators::TPC => 1,
            ColorfulOperators::Rougail(_) => 2,
            ColorfulOperators::Brocoli => 2,
            ColorfulOperators::Coreg(_) => 2
        }
    }

    fn is_associative(&self) -> bool {
        match &self {
            ColorfulOperators::Rougail(_) => true,
            ColorfulOperators::Coreg(_) => true,
            _ => false,
        }
    }
}


impl CommonIoInteractionInterface for ColorfulLangCioII {
    type InteractionLeafPatternType = ColorfulLeafPattern;
    type InteractionOperatorType = ColorfulOperators;
}