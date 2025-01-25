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

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;


// **********


pub struct MessageExchangeLineStyle {
    pub bold : bool,
    pub doubled : bool,
    pub color : Rgb<u8>,
    pub arrowhead_length : f32
}


impl MessageExchangeLineStyle {
    pub fn new(bold : bool,doubled : bool,color : Rgb<u8>,arrowhead_length : f32) -> MessageExchangeLineStyle {
        MessageExchangeLineStyle{
            bold,doubled,color,arrowhead_length
        }
    }
}


pub fn draw_message_exchange_horizontal_arrow(image : &mut RgbImage, x_start : f32, x_end : f32, y_pos : f32, style : &MessageExchangeLineStyle) {
    draw_styled_horizontal_line_mut(image,x_start,x_end,y_pos,style);
    if x_start < x_end {
        draw_arrowhead_rightward(image,x_end,y_pos,style.arrowhead_length,style.color);
    } else {
        draw_arrowhead_leftward(image,x_end,y_pos,style.arrowhead_length,style.color);
    }
}



pub fn draw_styled_horizontal_line_mut(image : &mut RgbImage, x_left : f32, x_right : f32, y_pos : f32, style : &MessageExchangeLineStyle) {
    if style.doubled {
        let doubling_y_shift = if style.bold {
            2.5
        } else {
            1.5
        };
        draw_thick_line_segment_mut(image,
            (x_left, y_pos - doubling_y_shift),
            (x_right, y_pos - doubling_y_shift),
            style.color, 
            style.bold);
        draw_thick_line_segment_mut(image,
            (x_left, y_pos + doubling_y_shift),
            (x_right, y_pos + doubling_y_shift),
            style.color, 
            style.bold);
    } else {
        draw_thick_line_segment_mut(image,
            (x_left, y_pos),
            (x_right, y_pos),
            style.color, 
            style.bold);
    }
}


pub fn draw_styled_vertical_line_mut(image : &mut RgbImage, y_top : f32, y_bot : f32, x_pos : f32, style : &MessageExchangeLineStyle) {
    if style.doubled {
        let doubling_x_shift = if style.bold {
            2.5
        } else {
            1.5
        };
        draw_thick_line_segment_mut(image,
            (x_pos - doubling_x_shift, y_top),
            (x_pos - doubling_x_shift, y_bot),
            style.color, 
            style.bold);
        draw_thick_line_segment_mut(image,
            (x_pos + doubling_x_shift, y_top),
            (x_pos + doubling_x_shift, y_bot),
            style.color, 
            style.bold);
    } else {
        draw_thick_line_segment_mut(image,
            (x_pos, y_top),
            (x_pos, y_bot),
            style.color, 
            style.bold);
    }
}




fn draw_thick_line_segment_mut(image : &mut RgbImage, start: (f32, f32), end: (f32, f32), color: Rgb<u8>, is_thick : bool) {
    if is_thick {
        draw_line_segment_mut(image,
            (start.0, start.1 - 0.5),
            (end.0, end.1 - 0.5),
            color);
        draw_line_segment_mut(image,
            (start.0, start.1 + 0.5),
            (end.0, end.1 + 0.5),
            color);
    } else {
        draw_line_segment_mut(image,
            start,
            end,
            color);
    }
}

fn draw_arrowhead_rightward(image : &mut RgbImage, x_pos : f32, y_pos : f32, arrowhead_length : f32, color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - arrowhead_length, y_pos - arrowhead_length),
                          color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - arrowhead_length, y_pos + arrowhead_length),
                          color);
}

fn draw_arrowhead_leftward(image : &mut RgbImage, x_pos : f32, y_pos : f32, arrowhead_length : f32, color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + arrowhead_length, y_pos - arrowhead_length),
                          color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + arrowhead_length, y_pos + arrowhead_length),
                          color);
}