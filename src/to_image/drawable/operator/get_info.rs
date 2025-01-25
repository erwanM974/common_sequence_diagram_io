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
use ab_glyph::{Font, PxScale};

use crate::to_image::drawable::operator::info::OperatorIntermediateInformation;
use crate::to_image::drawable::operator::builtin_operator::*;


impl<LI : Eq + Hash + Copy + Clone> DrawableOperator<LI> {
    
    
    pub fn get_intermediate_information(
        &self,
        scale: impl Into<PxScale> + Copy,
        font: &impl Font,
        y_margin_between_items : f32,
        x_margin_between_items : f32
    ) -> OperatorIntermediateInformation {
        let required_vertical_space_at_the_top : f32;
        let required_vertical_space_between_operands : f32;
        let requires_nest_shift : bool;
        let required_horizontal_space_at_left_most_lifeline : f32;
        match &self.kind {
            DrawableOperatorKind::CoRegionLike(lfs) => {
                if lfs.is_empty() {
                    required_vertical_space_at_the_top = y_margin_between_items;
                    required_vertical_space_between_operands = y_margin_between_items;
                    requires_nest_shift = false;
                    required_horizontal_space_at_left_most_lifeline = 0.0;
                } else {
                    required_vertical_space_at_the_top = 2.0*y_margin_between_items;
                    required_vertical_space_between_operands = 2.0*y_margin_between_items;
                    requires_nest_shift = false;
                    required_horizontal_space_at_left_most_lifeline = 0.0;
                }
            },
            DrawableOperatorKind::Framed(top_left_text) => {
                let (text_width, text_height,_) = top_left_text.paragraph_size(scale, font);
                required_vertical_space_at_the_top = text_height;
                required_vertical_space_between_operands = 2.0*y_margin_between_items;
                requires_nest_shift = true;
                required_horizontal_space_at_left_most_lifeline = text_width + 2.0*x_margin_between_items;
            }
        }
        OperatorIntermediateInformation{
            required_vertical_space_at_the_top,
            required_vertical_space_between_operands,
            requires_nest_shift,
            required_horizontal_space_at_left_most_lifeline
        }
    }

}

