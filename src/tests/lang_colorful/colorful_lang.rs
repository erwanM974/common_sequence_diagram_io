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

use image_colored_text::text::paragraph::ColoredTextParagraph;





#[derive(Clone, PartialEq, Debug)]
pub struct ColorfulContext {
    pub lf_names : Vec<String>,
    pub lf_descs : Vec<ColoredTextParagraph>,
    pub ms_names : Vec<String>,
    pub gt_names : Vec<String>,
}

#[derive(Debug,Clone)]
pub enum GateOrLifeline {
    Gate(usize),
    Lifeline(usize)
}

#[derive(Debug,Clone)]
pub struct ColorfulAction {
    pub item : GateOrLifeline,
    pub notes : (Option<ColoredTextParagraph>,Option<ColoredTextParagraph>)
}

impl ColorfulAction {

    pub fn new(
        item : GateOrLifeline,
        notes : (Option<ColoredTextParagraph>,Option<ColoredTextParagraph>)) -> ColorfulAction {
            ColorfulAction{item,notes}
    }
}

/* 
#[derive(Debug,Clone)]
pub enum ColorfulInteraction {
    Transmit(ColorfulAction,usize,ColorfulAction),
    TPC(Box<ColorfulInteraction>),
    Rougail(String,Box<ColorfulInteraction>,Box<ColorfulInteraction>),
    Brocoli(Box<ColorfulInteraction>,Box<ColorfulInteraction>),
    Coreg(Option<usize>,Box<ColorfulInteraction>,Box<ColorfulInteraction>)
}
*/



