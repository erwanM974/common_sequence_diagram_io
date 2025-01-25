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
use std::collections::HashMap;




use image_colored_text::text::paragraph::ColoredTextParagraph;

use crate::to_image::drawable::operator::builtin_operator::DrawableOperator;
use crate::to_image::drawable::leaf::broadcast::DrawableBroadcastLeafPattern;
use crate::to_image::drawable::leaf::info::{BroadcastLeafPatternIntermediateInformation, LifelineRequiredHorizontalSpaceInDiagram};









pub struct CompleteBroadcastLeafPatternDrawInstruction<LI : Eq + Hash + Copy + Clone> {
    pub pattern : DrawableBroadcastLeafPattern<LI>,
    pub intermediate_info : BroadcastLeafPatternIntermediateInformation<LI>,
    pub relative_y_pos : f32
}

impl <LI : Eq + Hash + Copy + Clone>  CompleteBroadcastLeafPatternDrawInstruction<LI> {
    pub fn new(pattern : DrawableBroadcastLeafPattern<LI>,intermediate_info : BroadcastLeafPatternIntermediateInformation<LI>,relative_y_pos : f32) -> Self {
        Self{pattern,intermediate_info,relative_y_pos}
    }
}



pub struct CompleteOperatorDrawInstruction<LI : Eq + Hash + Copy + Clone> {
    pub drawable_op : DrawableOperator<LI>,
    pub enclosed_lfs_reqs : HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>,
    pub nest_depth : u32,
    pub horizontal_seps_relative_vertical_positions : Vec<f32>
}

impl <LI : Eq + Hash + Copy + Clone>  CompleteOperatorDrawInstruction<LI> {
    pub fn new(
        drawable_op : DrawableOperator<LI>,
        enclosed_lfs_reqs : HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>,
        nest_depth : u32, 
        horizontal_seps_relative_vertical_positions : Vec<f32>
    ) -> Self {
        Self{drawable_op,enclosed_lfs_reqs,nest_depth,horizontal_seps_relative_vertical_positions}
    }
}



/** 
 * This holds all the information required to draw an interaction term as a sequence diagram.
 * **/
 pub struct CompleteInteractionDrawInstruction<LI : Eq + Hash + Copy + Clone> {
    // the total width of the image
    pub width : f32,
    // the total height of the image
    pub height : f32,
    // the value to add to the relative y positions of the leaf pattern and operator instructions to get their absolute position
    // this amounts to the top padding + the height of the headers
    pub y_shift_to_absolute : f32,
    // this gives the horizontal position on the left of which to draw input outside gates
    pub left_side_of_diagram_x_pos : f32,
    // this gives the horizontal position on the right of which to draw output outside gates
    pub right_side_of_diagram_x_pos : f32,
    // for each lifeline, the distance between the left side of the image and the horizontal line corresponding to the lifeline
    pub lifelines_horizontal_positions : HashMap<LI,f32>,
    // ***
    pub lifelines_headers : HashMap<LI,ColoredTextParagraph>,
    // all the instructions to draw all the patterns
    pub patterns_to_draw : Vec<CompleteBroadcastLeafPatternDrawInstruction<LI>>,
    // all the instructions to draw all the operators
    pub operators_to_draw : Vec<CompleteOperatorDrawInstruction<LI>>
}

