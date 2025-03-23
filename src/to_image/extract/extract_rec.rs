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
use std::collections::{HashMap, HashSet};



use crate::internal_representation::*;
use crate::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;
use crate::to_image::drawable::leaf::info::LifelineRequiredHorizontalSpaceInDiagram;
use crate::to_image::extract::instructions::*;




pub struct InteractionIntermediateInformation<LI : Eq + Hash + Copy + Clone> {
    pub max_input_gate_width : f32,
    pub max_output_gate_width : f32,
    pub involved_lifelines : HashSet<LI>,
    pub lfs_horizontal_reqs : HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>
}

impl<LI : Eq + Hash + Copy + Clone> InteractionIntermediateInformation<LI> {
    pub fn new(
        max_input_gate_width : f32,
        max_output_gate_width : f32,
        involved_lifelines : HashSet<LI>,
        lfs_horizontal_reqs : HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>
    ) -> Self {
        Self{max_input_gate_width,max_output_gate_width,involved_lifelines,lfs_horizontal_reqs}
    }
}





/** 
 * Recursively extracts intermediate drawing information from the structure of the Internal Representation of the interaction.
 * This consists in:
 * - updating the *ypos* vertical position as the term is traversed
 * - filling the *encountered_leaves* with all the patterns that are encountered at the leaves at the correct relative *ypos* position
 * - filling the *encountered_operators* with all the operators that are encountered with the correct information on the *ypos* of their operands
 * - returning information about the lifelines that are encountered in the term structure
 * **/
pub fn extract_intermediate_drawing_information_rec<CioII,LI,Context> 
    (   context : &Context,
        all_lifelines_in_diagram : &Vec<LI>,
        int_repr : &InteractionInternalRepresentation<CioII>,
        ypos : &mut f32,
        nest_shift : u32,
        encountered_leaves : &mut Vec<CompleteBroadcastLeafPatternDrawInstruction<LI>>,
        encountered_operators : &mut Vec<CompleteOperatorDrawInstruction<LI>>
    ) -> InteractionIntermediateInformation<LI>
    where 
        CioII : CommonIoInteractionInterface,
        LI : Eq + Hash + Copy + Clone,
        Context : ContextAwareInteractionDrawingInstructionsExtractor<CioII,LI>
    {
        match int_repr {
            InteractionInternalRepresentation::LeafPattern(leaf) => {
                match context.to_drawable_pattern(leaf) {
                    Some(drawable_pattern) => {
                        let leaf_info = drawable_pattern.get_intermediate_information(
                            context.get_scale(),
                            context.get_font(),
                            all_lifelines_in_diagram,
                            context.get_y_margin_between_seq_operands(),
                            context.get_margin_between_items(),
                            context.get_margin_between_items()
                        );
                        // ***
                        // recursive information gathering
                        let rec_info = InteractionIntermediateInformation::new(
                            leaf_info.input_gate_width, 
                            leaf_info.output_gates_max_width, 
                            leaf_info.involved_lifelines.clone(), 
                            leaf_info.lifelines_horizontal_requirements.clone()
                        );
                        let leaf_height = leaf_info.y_space_top_to_bottom;
                        // keeps track of encountered leaf
                        encountered_leaves.push(CompleteBroadcastLeafPatternDrawInstruction::new(drawable_pattern,leaf_info,*ypos));
                        // increments mutable vertical position reference
                        *ypos += leaf_height;
                        // ***
                        rec_info
                    },
                    None => {
                        InteractionIntermediateInformation::new(
                            0.0, 
                            0.0, 
                            HashSet::new(), 
                            HashMap::new()
                        )
                    }
                }
            },
            InteractionInternalRepresentation::Operator(operator, sub_ints) => {
                let mut op_operands_positions = vec![*ypos];
                let drawable_operator = context.to_drawable_operator(
                    operator,
                    sub_ints
                );
                let operator_info = drawable_operator.get_intermediate_information(
                    context.get_scale(),
                    context.get_font(),
                    context.get_margin_between_items(),
                    context.get_margin_between_items()
                );
                *ypos += operator_info.required_vertical_space_at_the_top;
                let rec_nest_shift = if operator_info.requires_nest_shift {
                    nest_shift + 1
                } else {
                    nest_shift
                };
                // recursive information gathering
                let mut max_input_gate_width = 0.0_f32;
                let mut max_output_gate_width = 0.0_f32;
                let mut op_included_lifelines = HashSet::new();
                let mut lifelines_reqs = HashMap::new();
                // iter operands
                let num_operands = sub_ints.len();
                for (count,sub_int) in sub_ints.iter().enumerate() {
                    let sub_rec_info = extract_intermediate_drawing_information_rec::<CioII,LI,Context>(
                        context,
                        all_lifelines_in_diagram,
                        sub_int,
                        ypos,
                        rec_nest_shift,
                        encountered_leaves,
                        encountered_operators);
                    // ***
                    // recursive information update
                    op_included_lifelines.extend(sub_rec_info.involved_lifelines);
                    LifelineRequiredHorizontalSpaceInDiagram::update_all_to_max(&mut lifelines_reqs, sub_rec_info.lfs_horizontal_reqs);
                    max_input_gate_width = f32::max(max_input_gate_width, sub_rec_info.max_input_gate_width);
                    max_output_gate_width = f32::max(max_output_gate_width, sub_rec_info.max_output_gate_width);
                    // updates the ypos
                    {
                        *ypos += operator_info.required_vertical_space_between_operands/2.0;
                        op_operands_positions.push(*ypos);
                        if count < num_operands - 1 {
                            *ypos += operator_info.required_vertical_space_between_operands/2.0;
                        } 
                    }
                }
                // the left most inner lifeline may require additional space on its left for drawing the operator label
                if let Some(leftmost_lf_id) = op_included_lifelines.iter()
                    .min_by(|l1,l2|
                        {
                            let l1_idx = all_lifelines_in_diagram.iter().position(|l| l == *l1).unwrap();
                            let l2_idx = all_lifelines_in_diagram.iter().position(|l| l == *l2).unwrap();
                            l1_idx.cmp(&l2_idx)
                        }
                    ) {
                    let letftmost_lf_reqs = lifelines_reqs.get_mut(leftmost_lf_id).unwrap();
                    letftmost_lf_reqs.on_the_left = f32::max(
                        letftmost_lf_reqs.on_the_left, 
                        operator_info.required_horizontal_space_at_left_most_lifeline
                    );
                }
                //
                // we make a copy of the internal lifelines horizontal requirements
                let mut enclosed_lfs_reqs = HashMap::new();
                for lf in &op_included_lifelines {
                    enclosed_lfs_reqs.insert(*lf, lifelines_reqs.get(lf).unwrap().clone());
                }
                // finalize recursive information gathering
                let rec_info = InteractionIntermediateInformation::new(
                    max_input_gate_width, 
                    max_output_gate_width, 
                    op_included_lifelines, 
                    lifelines_reqs
                );
                // keepts track of encountered operator instruction
                encountered_operators.push(
                    CompleteOperatorDrawInstruction::new(
                        drawable_operator, 
                        enclosed_lfs_reqs,
                        nest_shift, 
                        op_operands_positions
                    )
                );
                rec_info
            }
        }
}










