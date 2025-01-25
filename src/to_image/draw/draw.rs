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
use image::RgbImage;

use image_colored_text::draw::multi_line::draw_multiline_colored_text;
use image_colored_text::draw::coord::DrawCoord;

use crate::to_image::extract::instructions::*;
use crate::to_image::draw::context_aware_drawer::ContextAwareInteractionDrawer;
use crate::to_image::draw::util::draw_lifelines_vertical_spans;

pub(crate) fn make_image_from_display_information<
    LI : Eq + Hash + Copy + Clone,
    Drawer : ContextAwareInteractionDrawer<LI>
  >(
  palette : &Drawer,
  draw_instruction : &CompleteInteractionDrawInstruction<LI>
) -> RgbImage {

  let mut image = RgbImage::new( draw_instruction.width as u32, draw_instruction.height as u32);

  // Draw Background
  palette.draw_background(&mut image, draw_instruction.width, draw_instruction.height);

  // Draw vertical spans for each lifeline
  let involved_lifelines : Vec<LI> = draw_instruction.lifelines_horizontal_positions.keys().cloned().collect();
  draw_lifelines_vertical_spans(
    &mut image, 
    draw_instruction.y_shift_to_absolute, 
    &draw_instruction.lifelines_horizontal_positions,
    draw_instruction.height - palette.get_border_padding(),
    &palette.get_lifelines_colors(&involved_lifelines)
  );

  // Draw static lifelines header
  for (lf,header) in &draw_instruction.lifelines_headers {
    let lf_x_pos = *draw_instruction.lifelines_horizontal_positions.get(lf).unwrap();
    draw_multiline_colored_text(
      &mut image,
      &DrawCoord::CenteredAround(lf_x_pos),
      &DrawCoord::EndingAt(draw_instruction.y_shift_to_absolute),
      header,
      palette.get_font(),
      palette.get_scale()
  );
  }

  // Draw leaf patterns
  for leaf_instruct in &draw_instruction.patterns_to_draw {
    leaf_instruct.pattern.draw(
      &leaf_instruct.intermediate_info,
      &mut image, 
      palette.get_font(),
      palette.get_scale(),
      draw_instruction.y_shift_to_absolute + leaf_instruct.relative_y_pos,
      draw_instruction.left_side_of_diagram_x_pos,
      &draw_instruction.lifelines_horizontal_positions,
      draw_instruction.right_side_of_diagram_x_pos,
      palette.get_margin_between_items(), 
      palette.get_margin_between_items(),
      palette.get_y_margin_between_seq_operands(),
      palette.get_arrow_length()
    );
  }

  // Draw operators
  for operator_instruct in &draw_instruction.operators_to_draw {
    let mut horizontal_seps_absolute_vertical_positions = Vec::new();
    for x in &operator_instruct.horizontal_seps_relative_vertical_positions {
      horizontal_seps_absolute_vertical_positions.push(x + draw_instruction.y_shift_to_absolute);
    }
    operator_instruct.drawable_op.draw(
      &mut image, 
      palette.get_font(),
      palette.get_scale(),
      &horizontal_seps_absolute_vertical_positions,
      &operator_instruct.enclosed_lfs_reqs,
      &draw_instruction.lifelines_horizontal_positions,
       (operator_instruct.nest_depth as f32) * palette.get_nest_padding_unit(),
      palette.get_margin_between_items()
    );
  }

  image
}











