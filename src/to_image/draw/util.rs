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

use image::{Rgb, RgbImage};
use imageproc::drawing::{
    draw_filled_rect_mut,
    draw_line_segment_mut
};
use imageproc::rect::Rect;



// **********

pub fn draw_uniform_colored_background(image : &mut RgbImage, img_width : &f32, img_height : &f32, color : Rgb<u8>) {
    draw_filled_rect_mut(image, Rect::at(0,0).of_size(*img_width as u32,*img_height as u32), color);
}

pub fn draw_lifelines_vertical_spans<LI : Eq + Hash + Copy + Clone>(
    image : &mut RgbImage, 
    absolute_top_y_pos : f32,
    lifelines_horizontal_positions : &HashMap<LI,f32>,
    absolute_bottom_y_pos : f32,
    lifelines_colors : &HashMap<LI,Rgb<u8>>,
) {
    for (lf_id,lf_x_middle) in lifelines_horizontal_positions {
        let color = lifelines_colors.get(lf_id).unwrap();
        draw_line_segment_mut(image,
                              (*lf_x_middle, absolute_top_y_pos),
                              (*lf_x_middle, absolute_bottom_y_pos),
                              *color
        );
    }
}


