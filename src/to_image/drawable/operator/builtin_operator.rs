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



use std::hash::Hash;
use std::collections::HashSet;

use image::Rgb;
use image_colored_text::text::paragraph::ColoredTextParagraph;



pub struct DrawableOperator<LI : Eq + Hash + Copy + Clone> {
    pub frame_color : Rgb<u8>,
    pub kind : DrawableOperatorKind<LI>
}

impl<LI : Eq + Hash + Copy + Clone> DrawableOperator<LI> {

    pub fn new(frame_color : Rgb<u8>,kind : DrawableOperatorKind<LI>) -> Self {
        Self{frame_color,kind}
    }

} 



pub enum DrawableOperatorKind<LI : Eq + Hash + Copy + Clone> {
    Framed(ColoredTextParagraph),
    CoRegionLike(HashSet<LI>)
}
