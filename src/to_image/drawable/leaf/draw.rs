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

use image::RgbImage;
use ab_glyph::{Font, PxScale};

use image_colored_text::draw::multi_line::draw_multiline_colored_text;
use image_colored_text::draw::coord::DrawCoord;

use crate::to_image::drawable::leaf::broadcast::*;
use crate::to_image::drawable::leaf::info::*;
use crate::to_image::drawable::leaf::util::*;




impl<LifelineIdentifier : Eq + Hash + Copy + Clone> DrawableBroadcastLeafPattern<LifelineIdentifier> {

pub fn draw(
        &self, 
        info : &BroadcastLeafPatternIntermediateInformation<LifelineIdentifier>,
        image : &mut RgbImage,
        font: &impl Font,
        scale: impl Into<PxScale> + Copy,
        // this gives the vertical shift to add to the relative vertical positions given in *info*
        y_pos_top_of_pattern : f32, 
        // this gives the horizontal position on the left of which to draw input outside gates
        left_side_of_diagram_x_pos : f32,
        // this gives the horizontal positions of the lifelines
        lifelines_horizontal_positions : &HashMap<LifelineIdentifier,f32>,
        // this gives the horizontal position on the right of which to draw output outside gates
        right_side_of_diagram_x_pos : f32,
        x_margin_between_items : f32,
        y_margin_between_items : f32,
        y_margin_between_seq_operands : f32, 
        x_arrow_length : f32) 
    {
        // start by drawing the origin
        match &self.origin {
            DrawableBroadcastLeafPatternOrigin::Empty => {
                // ***
            },
            DrawableBroadcastLeafPatternOrigin::InputOutsideGate(ref gate_parag) => {
                draw_multiline_colored_text(
                    image,
                    &DrawCoord::EndingAt(left_side_of_diagram_x_pos),
                    &DrawCoord::CenteredAround(y_pos_top_of_pattern + info.y_space_top_to_midline),
                    gate_parag,
                    font,
                    scale
                );
            },
            DrawableBroadcastLeafPatternOrigin::Lifeline(ref orig_lf,ref orig_act) => {
                let orig_lf_x_pos = *lifelines_horizontal_positions.get(orig_lf).unwrap();
                // ***
                if let Some(preamble) = &orig_act.preamble {
                    draw_multiline_colored_text(
                        image,
                        &DrawCoord::CenteredAround(orig_lf_x_pos),
                        &DrawCoord::EndingAt(y_pos_top_of_pattern + info.y_space_top_to_midline),
                        preamble,
                        font,
                        scale
                    );
                }
                // ***
                if let Some(postamble) = &orig_act.postamble {
                    draw_multiline_colored_text(
                        image,
                        &DrawCoord::CenteredAround(orig_lf_x_pos),
                        &DrawCoord::StartingAt(y_pos_top_of_pattern + info.y_space_top_to_midline),
                        postamble,
                        font,
                        scale
                    );
                }
            }
        }

        // then draw the lifeline targets
        for (tar_lf, tar_act) in &self.lifeline_targets {
            let tar_lf_x_pos = *lifelines_horizontal_positions.get(tar_lf).unwrap();
            // in case the target is the same lifeline as the origin, we need to define a custom midline vertical
            let target_y_midline = match &info.required_space_under_emission {
                Some((orig_lf,req_space)) => {
                    if tar_lf == orig_lf {
                        let (_,pre_h,_,_) = tar_act.get_size_around_midline(scale, font);
                        y_pos_top_of_pattern + info.y_space_top_to_midline + req_space + y_margin_between_seq_operands + pre_h
                    } else {
                        y_pos_top_of_pattern + info.y_space_top_to_midline
                    }
                },
                None => {
                    y_pos_top_of_pattern + info.y_space_top_to_midline
                }
            };
            match &tar_act {
                TargetLifelineBroadcastDrawInstruction::TwoParts(ref lf_act) => {
                    // ***
                    if let Some(preamble) = &lf_act.preamble {
                        draw_multiline_colored_text(
                            image,
                            &DrawCoord::CenteredAround(tar_lf_x_pos),
                            &DrawCoord::EndingAt(target_y_midline),
                            preamble,
                            font,
                            scale
                        );
                    }
                    // ***
                    if let Some(postamble) = &lf_act.postamble {
                        draw_multiline_colored_text(
                            image,
                            &DrawCoord::CenteredAround(tar_lf_x_pos),
                            &DrawCoord::StartingAt(target_y_midline),
                            postamble,
                            font,
                            scale
                        );
                    }
                },
                TargetLifelineBroadcastDrawInstruction::Centered(ref lf_act) => {
                    draw_multiline_colored_text(
                        image,
                        &DrawCoord::CenteredAround(tar_lf_x_pos),
                        &DrawCoord::CenteredAround(target_y_midline),
                        &lf_act.content,
                        font,
                        scale
                    );
                }
            }
        }

        // then draw the output gate targets
        {
            let mut y = y_pos_top_of_pattern + info.y_space_top_to_midline - info.y_shift_above_midline_for_output_gates;
            for target_output_gate in &self.output_outside_gates_targets {
                draw_multiline_colored_text(
                    image,
                    &DrawCoord::StartingAt(right_side_of_diagram_x_pos),
                    &DrawCoord::StartingAt(y),
                    target_output_gate,
                    font,
                    scale
                );
                y += target_output_gate.paragraph_size(scale, font).1;
                y += y_margin_between_items;
            }
        }
        
        // then draw the message
        {
            let anchor_lifeline_x_pos = *lifelines_horizontal_positions.get(&info.message_drawing_location.0.anchor_lifeline).unwrap();
            let message_x_pos = anchor_lifeline_x_pos + info.message_drawing_location.1;
            draw_multiline_colored_text(
                image,
                &DrawCoord::CenteredAround(message_x_pos),
                &DrawCoord::EndingAt(y_pos_top_of_pattern + info.y_space_top_to_midline - y_margin_between_items),
                &self.message,
                font,
                scale
            );
        }
        
        // ***
        let (has_origin ,origin_as_lifeline) = match &self.origin {
            DrawableBroadcastLeafPatternOrigin::Empty => {
                (None,None)
            },
            DrawableBroadcastLeafPatternOrigin::InputOutsideGate(_) => {
                (Some(left_side_of_diagram_x_pos),None)
            },
            DrawableBroadcastLeafPatternOrigin::Lifeline(ref orig_lf,_) => {
                let orig_lf_x_pos = *lifelines_horizontal_positions.get(orig_lf).unwrap();
                (Some(orig_lf_x_pos),Some(*orig_lf))
            }
        };

        // we draw a full line iff:
        // there is an origin
        // and there is a single target 
        // and that target isn't also the origin lifeline
        let single_target;
        let mut sends_message_to_self = false;
        let draw_full_line : Option<(f32, f32)> = {
            // ***
            let output_gates_modifier = if !self.output_outside_gates_targets.is_empty() {
                1
            } else {
                0
            };
            single_target = self.lifeline_targets.len() + output_gates_modifier == 1;
            // ***
            if single_target && has_origin.is_some() {
                let origin_x_pos = has_origin.unwrap();
                let (target_x_pos,target_as_lifeline) = {
                    match self.lifeline_targets.keys().next() {
                        Some(targ_lf) => {
                            let targ_x_pos = *lifelines_horizontal_positions.get(targ_lf).unwrap();
                            (targ_x_pos,Some(*targ_lf))
                        },
                        None => {
                            // then target is an output gate
                            (right_side_of_diagram_x_pos,None)
                        }
                    }
                };
                // origin and target must not be both the same lifeline
                match (origin_as_lifeline,target_as_lifeline) {
                    (Some(orig_lf),Some(targ_lf)) => {
                        if orig_lf == targ_lf {
                            sends_message_to_self = true;
                            None 
                        } else {
                            Some((origin_x_pos,target_x_pos))
                        }
                    },
                    _ => {
                        Some((origin_x_pos,target_x_pos))
                    }
                }
            } else {
                None 
            }
        };

        match draw_full_line {
            Some((x_start,x_end)) => {
                // we draw a single continuous horizontal arrow
                draw_message_exchange_horizontal_arrow(
                    image,
                    x_start,
                    x_end,
                    y_pos_top_of_pattern + info.y_space_top_to_midline,
                    &self.line_style
                );
            },
            None => {
                // otherwise we draw separated smaller arrows on each individual item

                // exiting arrow for the origin 
                if let Some(origin_x_pos) = &has_origin {
                    match &origin_as_lifeline {
                        None => {
                            // the origin is an input gate
                            draw_message_exchange_horizontal_arrow(
                                image,
                                origin_x_pos - x_margin_between_items,
                                origin_x_pos + x_margin_between_items,
                                y_pos_top_of_pattern + info.y_space_top_to_midline,
                                &self.line_style
                            );
                        },
                        Some(_) => {
                            // the origin is a lifeline
                            let end_x_pos = if info.message_drawing_location.0.draw_message_on_left {
                                origin_x_pos - x_arrow_length
                            } else {
                                origin_x_pos + x_arrow_length
                            };
                            if sends_message_to_self && single_target {
                                // here it is a message to self with a single target.
                                // we do not draw the arrowhead on the top midline 
                                draw_styled_horizontal_line_mut(
                                    image,
                                    *origin_x_pos,
                                    end_x_pos,
                                    y_pos_top_of_pattern + info.y_space_top_to_midline,
                                    &self.line_style
                                );
                            } else {
                                draw_message_exchange_horizontal_arrow(
                                    image,
                                    *origin_x_pos,
                                    end_x_pos,
                                    y_pos_top_of_pattern + info.y_space_top_to_midline,
                                    &self.line_style
                                );
                            }
                        }
                    }
                }

                // incoming arrows for the lifeline targets
                for (tar_lf,tar_act) in &self.lifeline_targets {
                    let tar_lf_x_pos = *lifelines_horizontal_positions.get(tar_lf).unwrap();
                    let from_the_left = if let Some(orig_x_pos) = &has_origin {
                        // here there is a specific origin that is specified
                        if let Some(orig_lf) = &origin_as_lifeline {
                            if orig_lf == tar_lf {
                                // if the origin is the same as the target
                                // the message comes from the side on which it is drawn
                                info.message_drawing_location.0.draw_message_on_left
                            } else {
                                // otherwise, if it is a different lifeline, it is on the left if that origin is on the left of the target
                                *orig_x_pos < tar_lf_x_pos
                            }
                        } else {
                            // here the origin is an input gate so the message comes from the left
                            true
                        }
                    } else {
                        // here the origin is the environment so the message comes from the left
                        true
                    };
                    let start_x_pos = if from_the_left {
                        tar_lf_x_pos - x_arrow_length
                    } else {
                        tar_lf_x_pos + x_arrow_length
                    };
                    // in case the target is the same lifeline as the origin, we need to define a custom midline vertical
                    let (target_y_midline,draw_self_link) = match &info.required_space_under_emission {
                        Some((orig_lf,req_space)) => {
                            if tar_lf == orig_lf {
                                let (_,pre_h,_,_) = tar_act.get_size_around_midline(scale, font);
                                ( 
                                    y_pos_top_of_pattern + info.y_space_top_to_midline + req_space + y_margin_between_seq_operands + pre_h,
                                    Some(y_pos_top_of_pattern + info.y_space_top_to_midline)
                                )
                            } else {
                                ( 
                                    y_pos_top_of_pattern + info.y_space_top_to_midline,
                                    None 
                                )
                            }
                        },
                        None => {
                            (
                                y_pos_top_of_pattern + info.y_space_top_to_midline,
                                None 
                            )
                        }
                    };
                    // draw the arrow
                    draw_message_exchange_horizontal_arrow(
                        image,
                        start_x_pos,
                        tar_lf_x_pos,
                        target_y_midline,
                        &self.line_style
                    );
                    // here the target lifeline is the same as the emiting lifeline so we need to add a recurved arrow from top to bottom on the same lifeline
                    if let Some(top_y_midline) = draw_self_link {
                        // the horizontal part of the top midline
                        draw_styled_horizontal_line_mut(
                            image,start_x_pos,tar_lf_x_pos,top_y_midline,&self.line_style
                        );
                        // the vertical part linking the top midline (on the emitting action) to the bottom midline (on the receiving action)
                        draw_styled_vertical_line_mut(
                            image, top_y_midline, target_y_midline, start_x_pos, &self.line_style
                        );
                    }
                }

                // finally incoming arrow for the target output gates if there are any
                if !self.output_outside_gates_targets.is_empty() {
                    draw_message_exchange_horizontal_arrow(
                        image,
                        right_side_of_diagram_x_pos - x_margin_between_items,
                        right_side_of_diagram_x_pos + x_margin_between_items,
                        y_pos_top_of_pattern + info.y_space_top_to_midline,
                        &self.line_style
                    );
                }

            }
        }
        

    }

}