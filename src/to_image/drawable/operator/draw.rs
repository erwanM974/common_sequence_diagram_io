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
use ab_glyph::{Font, PxScale};

use image::RgbImage;

use crate::to_image::drawable::leaf::info::LifelineRequiredHorizontalSpaceInDiagram;
use crate::to_image::drawable::operator::builtin_operator::*;
use crate::to_image::drawable::operator::util::*;

impl<LI : Eq + Hash + Copy + Clone> DrawableOperator<LI> {
        
    /**
     * Draws the operator on the image.
     * **/
     pub fn draw(
        &self, 
        image : &mut RgbImage, 
        font: &impl Font,
        scale: impl Into<PxScale> + Copy,
        horizontal_seps_vertical_positions : &[f32],
        enclosed_lfs_reqs : &HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>,
        lifelines_horizontal_positions : &HashMap<LI,f32>, 
        nest_padding : f32,
        margin_between_items : f32) {

        match &self.kind {
            DrawableOperatorKind::CoRegionLike(framed_lfs) => {
                if framed_lfs.is_empty() {
                    // do nothing
                } else {
                    draw_coregion_frame(
                        image, 
                        framed_lfs, 
                        nest_padding, 
                        horizontal_seps_vertical_positions, 
                        lifelines_horizontal_positions, 
                        self.frame_color, 
                        margin_between_items
                    );
                }
            },
            DrawableOperatorKind::Framed(label) => {
                let leftmost_lf = *enclosed_lfs_reqs.keys()
                    .min_by(|l1,l2|
                        {
                            let l1_x_pos = *lifelines_horizontal_positions.get(l1).unwrap();
                            let l2_x_pos = *lifelines_horizontal_positions.get(l2).unwrap();
                            l1_x_pos.partial_cmp(&l2_x_pos).unwrap()
                        }
                    )
                    .unwrap();
                let rightmost_lf = *enclosed_lfs_reqs.keys()
                    .max_by(|l1,l2|
                        {
                            let l1_x_pos = *lifelines_horizontal_positions.get(l1).unwrap();
                            let l2_x_pos = *lifelines_horizontal_positions.get(l2).unwrap();
                            l1_x_pos.partial_cmp(&l2_x_pos).unwrap()
                        }
                    )
                    .unwrap();
                draw_combined_fragment_frame(
                    image, 
                    label, 
                    enclosed_lfs_reqs, 
                    nest_padding, 
                    horizontal_seps_vertical_positions, 
                    leftmost_lf, 
                    rightmost_lf, 
                    lifelines_horizontal_positions, 
                    self.frame_color, 
                    margin_between_items, 
                    font, 
                    scale
                );
            }
        }
    }
    

}




