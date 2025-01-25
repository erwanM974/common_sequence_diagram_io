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
use std::collections::{HashSet, HashMap};
use ab_glyph::{Font, PxScale};

use image::{Rgb, RgbImage};
use image_colored_text::draw::coord::DrawCoord;
use image_colored_text::draw::multi_line::draw_multiline_colored_text;
use image_colored_text::text::paragraph::ColoredTextParagraph;
use imageproc::drawing::draw_line_segment_mut;

use crate::to_image::drawable::leaf::info::LifelineRequiredHorizontalSpaceInDiagram;


pub(crate) fn draw_combined_fragment_frame<LI : Eq + Hash + Copy + Clone>(    
    image : &mut RgbImage,
    label : &ColoredTextParagraph,
    enclosed_lfs_reqs : &HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>,
    nest_padding : f32,
    horizontal_seps_vertical_positions : &[f32],
    leftmost_lf : LI,
    rightmost_lf : LI,
    lifelines_horizontal_positions : &HashMap<LI,f32>,
    color : Rgb<u8>,
    x_margin_between_items : f32,
    font: &impl Font,
    scale: impl Into<PxScale> + Copy,
) 
{
    let x_left = {
        let leftmost_lf_x_pos = lifelines_horizontal_positions.get(&leftmost_lf).unwrap();
        let leftmost_lf_hor_reqs = enclosed_lfs_reqs.get(&leftmost_lf).unwrap();
        leftmost_lf_x_pos + nest_padding - leftmost_lf_hor_reqs.on_the_left
    };
    let x_right  = {
        let rightmost_lf_x_pos = lifelines_horizontal_positions.get(&rightmost_lf).unwrap();
        let rightmost_lf_hor_reqs = enclosed_lfs_reqs.get(&rightmost_lf).unwrap();
        rightmost_lf_x_pos + rightmost_lf_hor_reqs.on_the_right - nest_padding
    };

    let mut y_coords : Vec<f32> = horizontal_seps_vertical_positions.to_vec();
    let y_start : f32 = y_coords.remove(0);
    let y_end : f32 = y_coords.pop().unwrap();
    draw_line_segment_mut(image,
                            (x_left, y_start),
                            (x_left, y_end),
                            color);
    draw_line_segment_mut(image,
                            (x_right, y_start),
                            (x_right, y_end),
                            color);
    draw_line_segment_mut(image,
                            (x_left, y_start),
                            (x_right, y_start),
                            color);
    draw_line_segment_mut(image,
                            (x_left, y_end),
                            (x_right, y_end),
                            color);
    for y_coord in y_coords {
        draw_line_segment_mut(image,
                                (x_left, y_coord),
                                (x_right, y_coord),
                                color);
    }
    draw_multiline_colored_text(
        image,
        &DrawCoord::StartingAt(x_left + x_margin_between_items),
        &DrawCoord::StartingAt(y_start),
        label,
        font,
        scale
    );
}


pub(crate) fn draw_coregion_frame<LI : Eq + Hash + Copy + Clone>(    
    image : &mut RgbImage,
    framed_lfs : &HashSet<LI>,
    nest_padding : f32,
    horizontal_seps_vertical_positions : &[f32],
    lifelines_horizontal_positions : &HashMap<LI,f32>,
    color : Rgb<u8>,
    margin_between_items : f32
)
{

    let mut y_coords : Vec<f32> = horizontal_seps_vertical_positions.to_vec();
    let y_start : f32 = y_coords.remove(0);
    // ***
    let y_end : f32 = y_coords.pop().unwrap();
    for lf in framed_lfs {
        let lf_x_pos = lifelines_horizontal_positions.get(lf).unwrap();
        let x_left = lf_x_pos + nest_padding - 2.0*margin_between_items ;
        let x_right = lf_x_pos + 2.0*margin_between_items - nest_padding;
        // ***
        draw_line_segment_mut(image,
                              (x_left, y_start),
                              (x_right, y_start),
                              color);
        draw_line_segment_mut(image,
                              (x_left, y_start),
                              (x_left, y_start + margin_between_items),
                              color);
        draw_line_segment_mut(image,
                              (x_right, y_start),
                              (x_right, y_start + margin_between_items),
                              color);
        // ***
        draw_line_segment_mut(image,
                              (x_left, y_end),
                              (x_right, y_end),
                              color);
        draw_line_segment_mut(image,
                              (x_left, y_end),
                              (x_left, y_end - margin_between_items),
                              color);
        draw_line_segment_mut(image,
                              (x_right, y_end),
                              (x_right, y_end - margin_between_items),
                              color);
        // ***
        for y_coord in &y_coords {
            draw_line_segment_mut(image,
                                  (x_left, *y_coord),
                                  (x_right, *y_coord),
                                  color);
            draw_line_segment_mut(image,
                                  (x_left, *y_coord + margin_between_items/2.0),
                                  (x_left, *y_coord - margin_between_items/2.0),
                                  color);
            draw_line_segment_mut(image,
                                  (x_right, *y_coord + margin_between_items/2.0),
                                  (x_right, *y_coord - margin_between_items/2.0),
                                  color);
        }
    }
}





