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
use std::collections::{HashSet,HashMap};
use ab_glyph::{Font, PxScale};

use crate::to_image::drawable::leaf::broadcast::*;
use crate::to_image::drawable::leaf::info::*;



impl<LifelineIdentifier : Eq + Hash + Copy + Clone> DrawableBroadcastLeafPattern<LifelineIdentifier> {

    pub fn get_intermediate_information(
        &self, 
        scale: impl Into<PxScale> + Copy,
        font: &impl Font,
        all_lifelines_in_diagram : &[LifelineIdentifier],
        y_margin_between_seq_operands : f32,
        x_margin_between_items : f32,
        y_margin_between_items : f32,
    ) -> BroadcastLeafPatternIntermediateInformation<LifelineIdentifier> {
        let minimum_horizontal_space_for_involved_lifeliens = LifelineRequiredHorizontalSpaceInDiagram::new(
            2.5*x_margin_between_items, 
            2.5*x_margin_between_items
        );
        // ***
        let mut input_gate_width = 0.0_f32;
        let mut output_gates_max_width = 0.0_f32;
        // ***
        let mut involved_lifelines : HashSet<LifelineIdentifier> = HashSet::new();
        // ***
        let mut lifelines_horizontal_requirements : HashMap<LifelineIdentifier, LifelineRequiredHorizontalSpaceInDiagram> = HashMap::new();
        let (msg_txt_width, msg_txt_height, _) = self.message.paragraph_size(scale, font);
        // required vertical space between the top of the broadcast pattern and the midline on which we draw the horizontal line
        let mut y_space_top_to_midline = msg_txt_height + y_margin_between_items;
        let mut y_space_midline_to_bottom = y_margin_between_items;
        // ***
        // ***
        let mut required_space_under_emission : Option<(LifelineIdentifier, f32)> = None;
        // if the origin is a lifeline, the message will be drawn close to it. Otherwise it will be drawn close to the the target lifeline
        // that is the most on the left
        let message_drawing_location : MessageDrawingLocation<LifelineIdentifier>;
        let mut message_drawing_location_lf_horizontal_reqs_if_emission : Option<LifelineRequiredHorizontalSpaceInDiagram> = None;
        // we start by dealing with the origin
        match &self.origin {
            DrawableBroadcastLeafPatternOrigin::Empty => {
                let leftmost_lf_id = *self.lifeline_targets.keys()
                    .min_by(|l1,l2|
                        {
                            let l1_idx = all_lifelines_in_diagram.iter().position(|l| l == *l1).unwrap();
                            let l2_idx = all_lifelines_in_diagram.iter().position(|l| l == *l2).unwrap();
                            l1_idx.cmp(&l2_idx)
                        }
                    )
                    .unwrap();
                message_drawing_location = MessageDrawingLocation::new(leftmost_lf_id,true);
            },
            DrawableBroadcastLeafPatternOrigin::InputOutsideGate(ref gate_parag) => {
                let leftmost_lf_id = *self.lifeline_targets.keys()
                    .min_by(|l1,l2|
                        {
                            let l1_idx = all_lifelines_in_diagram.iter().position(|l| l == *l1).unwrap();
                            let l2_idx = all_lifelines_in_diagram.iter().position(|l| l == *l2).unwrap();
                            l1_idx.cmp(&l2_idx)
                        }
                    )
                    .unwrap();
                message_drawing_location = MessageDrawingLocation::new(leftmost_lf_id,true);
                // ***
                let (gate_width, gate_height, _) = gate_parag.paragraph_size(scale, font);
                input_gate_width = gate_width;
                // ***
                y_space_top_to_midline = f32::max(y_space_top_to_midline, gate_height/2.0);
                y_space_midline_to_bottom = f32::max(y_space_midline_to_bottom, gate_height/2.0);
            },
            DrawableBroadcastLeafPatternOrigin::Lifeline(ref orig_lf,ref orig_act) => {
                let (pre_w,pre_h,post_w,post_h) = orig_act.get_size_around_midline(scale, font);
                let half_max_act_width = (f32::max(pre_w,post_w) + x_margin_between_items) /2.0;
                // memorize that for later
                required_space_under_emission = Some((*orig_lf,f32::max(post_h,2.0*y_margin_between_items)));
                // ***
                y_space_top_to_midline = f32::max(y_space_top_to_midline, pre_h);
                y_space_midline_to_bottom = f32::max(y_space_midline_to_bottom, post_h);
                // by default the message is written on the right of the emitting lifeline
                // except if there are lifeline targets and these targets are on its left
                // and there is no output gate
                let draw_message_on_left = 
                    (!self.lifeline_targets.is_empty())
                    &&
                    self.lifeline_targets.iter()
                    .all(
                        |(tar_lf,_)| {
                            let orig_lf_idx = all_lifelines_in_diagram.iter().position(|l| l == orig_lf).unwrap();
                            let tar_lf_idx = all_lifelines_in_diagram.iter().position(|l| l == tar_lf).unwrap();
                            tar_lf_idx <= orig_lf_idx
                         }
                    )
                    &&
                    self.output_outside_gates_targets.is_empty();
                // ***
                message_drawing_location = MessageDrawingLocation::new(*orig_lf, draw_message_on_left);
                message_drawing_location_lf_horizontal_reqs_if_emission = Some(
                    LifelineRequiredHorizontalSpaceInDiagram::new(half_max_act_width, half_max_act_width)
                );
                // ***
                let mut reqs = LifelineRequiredHorizontalSpaceInDiagram::new(half_max_act_width, half_max_act_width);
                reqs.update_to_max(minimum_horizontal_space_for_involved_lifeliens.clone());
                lifelines_horizontal_requirements.insert(*orig_lf, reqs);
                // ***
                involved_lifelines.insert(*orig_lf);
            }
        }
        // ***
        // now let us deal with the lifeline targets
        for (tar_lf, tar_act) in &self.lifeline_targets {
            let (pre_w,pre_h,post_w,post_h) = tar_act.get_size_around_midline(scale, font);
            let half_max_act_width = (f32::max(pre_w,post_w) + x_margin_between_items) /2.0;
            let mut new_reqs = LifelineRequiredHorizontalSpaceInDiagram::new(half_max_act_width, half_max_act_width);
            // ***
            let mut default_upd_y_space_top_to_midline = pre_h;
            let mut default_upd_y_space_midline_to_bottom = post_h;
            if let Some((orig_lf,req_space_under_emission)) = &required_space_under_emission {
                if orig_lf == tar_lf {
                    // target lifeline is the same as origin lifeline so action should be drawn underneath the emission with added margins
                    default_upd_y_space_top_to_midline = 0.0;
                    default_upd_y_space_midline_to_bottom = req_space_under_emission + y_margin_between_seq_operands + pre_h + post_h + y_margin_between_items;
                    // also this means that we have already added a horizontal requirements for that lifeline so we need to retrieve it
                    // and keep the maximum horizontal space requirement
                    new_reqs.update_to_max(lifelines_horizontal_requirements.remove(orig_lf).unwrap());
                }
            }
            // ***
            y_space_top_to_midline = f32::max(y_space_top_to_midline, default_upd_y_space_top_to_midline);
            y_space_midline_to_bottom = f32::max(y_space_midline_to_bottom, default_upd_y_space_midline_to_bottom);
            // ***
            new_reqs.update_to_max(minimum_horizontal_space_for_involved_lifeliens.clone());
            lifelines_horizontal_requirements.insert(*tar_lf, new_reqs);
            // ***
            involved_lifelines.insert(*tar_lf);
        }
        // ***
        // we also need to update the horizontal requirements on lifelines due to the drawing of the message
        let precise_message_drawing_location_x_shift_wrt_anchor_lifeline : f32;
        {
            // the lifeline close to which to draw the message
            let msg_anchor_lf = message_drawing_location.anchor_lifeline;
            let msg_anchor_lf_idx = all_lifelines_in_diagram.iter().position(|l| *l == msg_anchor_lf).unwrap();
            let mut msg_anchor_lf_previous_horizontal_reqs = if let Some(x) = message_drawing_location_lf_horizontal_reqs_if_emission {
                x 
            } else {
                lifelines_horizontal_requirements.get(&msg_anchor_lf).unwrap().clone()
            };
            // ***
            if message_drawing_location.draw_message_on_left {
                precise_message_drawing_location_x_shift_wrt_anchor_lifeline = 0.0 - (msg_anchor_lf_previous_horizontal_reqs.on_the_left + msg_txt_width/2.0 + x_margin_between_items);
                // we draw the message on the left of that lifeline
                // ***
                if msg_anchor_lf_idx > 0 {
                    // if there is a neighbor to its left then the width of the message may be shared between the two
                    let left_neighbor_lf = all_lifelines_in_diagram.get(msg_anchor_lf_idx - 1).unwrap();
                    let mut left_neighbor_req = if let Some(x) = lifelines_horizontal_requirements.remove(left_neighbor_lf) {
                        x
                    } else {
                        LifelineRequiredHorizontalSpaceInDiagram::new_empty()
                    };
                    // half of the width is for the left side of the anchor lifeline
                    msg_anchor_lf_previous_horizontal_reqs.on_the_left += x_margin_between_items + msg_txt_width/2.0;
                    lifelines_horizontal_requirements.get_mut(&msg_anchor_lf).unwrap().update_to_max(
                        msg_anchor_lf_previous_horizontal_reqs 
                    );
                    // the other half of the width is for the right side of the left neighbor of the anchor lifeline
                    // with added margin
                    left_neighbor_req.on_the_right += x_margin_between_items + msg_txt_width/2.0;
                    lifelines_horizontal_requirements.insert(*left_neighbor_lf, left_neighbor_req);
                } else {
                    // otherwise the width of the message must be added to the required space on the left
                    msg_anchor_lf_previous_horizontal_reqs.on_the_left += x_margin_between_items + msg_txt_width;
                    lifelines_horizontal_requirements.get_mut(&msg_anchor_lf).unwrap().update_to_max(
                        msg_anchor_lf_previous_horizontal_reqs 
                    );
                }
            } else {
                precise_message_drawing_location_x_shift_wrt_anchor_lifeline = msg_anchor_lf_previous_horizontal_reqs.on_the_right + msg_txt_width/2.0 + x_margin_between_items;
                // we draw the message on the right of that lifeline
                // this also means that it is necessarily the emitting lifeline
                // ***
                if msg_anchor_lf_idx < (all_lifelines_in_diagram.len() - 1) {
                    // if there is a neighbor to its right then the width of the message may be shared between the two
                    let right_neighbor_lf = all_lifelines_in_diagram.get(msg_anchor_lf_idx + 1).unwrap();
                    let mut right_neighbor_req = if let Some(x) = lifelines_horizontal_requirements.remove(right_neighbor_lf) {
                        x
                    } else {
                        LifelineRequiredHorizontalSpaceInDiagram::new_empty()
                    };
                    // half of the width is for the right side of the anchor lifeline
                    msg_anchor_lf_previous_horizontal_reqs.on_the_right += x_margin_between_items + msg_txt_width/2.0;
                    lifelines_horizontal_requirements.get_mut(&msg_anchor_lf).unwrap().update_to_max(
                        msg_anchor_lf_previous_horizontal_reqs 
                    );
                    // the other half of the width is for the left side of the right neighbor of the anchor lifeline
                    // with added margin
                    right_neighbor_req.on_the_left += x_margin_between_items + msg_txt_width/2.0;
                    lifelines_horizontal_requirements.insert(*right_neighbor_lf, right_neighbor_req);
                } else {
                    // otherwise the width of the message must be added to the required space on the right
                    msg_anchor_lf_previous_horizontal_reqs.on_the_right += x_margin_between_items + msg_txt_width;
                    lifelines_horizontal_requirements.get_mut(&msg_anchor_lf).unwrap().update_to_max(
                        msg_anchor_lf_previous_horizontal_reqs 
                    );
                }
            }
        }
        // ***
        // finally we deal with the target output gates
        // if there are several output gates, they will be drawn on top of one another at the right of the diagram
        let y_shift_above_midline_for_output_gates = if !self.output_outside_gates_targets.is_empty() {
            let mut stacked_output_gates_heights = y_margin_between_items;
            for target_output_gate in &self.output_outside_gates_targets {
                let (gate_width, gate_height, _) = target_output_gate.paragraph_size(scale, font);
                output_gates_max_width = f32::max(output_gates_max_width, gate_width);
                stacked_output_gates_heights += gate_height + y_margin_between_items;
            }
            let half_stacked_output_gates_heights = stacked_output_gates_heights/2.0;
            y_space_top_to_midline = f32::max(y_space_top_to_midline, half_stacked_output_gates_heights);
            y_space_midline_to_bottom = f32::max(y_space_midline_to_bottom, half_stacked_output_gates_heights);
            half_stacked_output_gates_heights
        } else {
            0.0
        };
        // ***
        let y_space_top_to_bottom = y_space_top_to_midline + y_space_midline_to_bottom;
        BroadcastLeafPatternIntermediateInformation::new(
            y_space_top_to_bottom, 
            y_space_top_to_midline,
            (message_drawing_location,precise_message_drawing_location_x_shift_wrt_anchor_lifeline),
            involved_lifelines,
            lifelines_horizontal_requirements,
            required_space_under_emission,
            input_gate_width,
            output_gates_max_width,
            y_shift_above_midline_for_output_gates
        )
    }
}






