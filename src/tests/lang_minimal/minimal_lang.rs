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





#[derive(Clone, PartialEq, Debug)]
pub struct GeneralContext {
    pub lf_names : Vec<String>,
    pub ms_names : Vec<String>
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum MinimalActionKind {
    Emission,
    Reception
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct MinimalAction {
    pub lf_id : usize,
    pub ms_id : usize,
    pub kind : MinimalActionKind
}

impl MinimalAction {

    pub fn new(
        lf_id : usize,
        ms_id : usize,
        kind : MinimalActionKind) -> MinimalAction {
            MinimalAction{lf_id,ms_id,kind}
    }
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum MinimalInteraction {
    Empty,
    Action(MinimalAction),
    Strict(Box<MinimalInteraction>,Box<MinimalInteraction>),
    Seq(Box<MinimalInteraction>,Box<MinimalInteraction>),
    Alt(Box<MinimalInteraction>,Box<MinimalInteraction>),
    Par(Box<MinimalInteraction>,Box<MinimalInteraction>),
    Loop(Box<MinimalInteraction>)
}




