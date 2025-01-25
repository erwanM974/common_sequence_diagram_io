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


use image_colored_text::text::paragraph::ColoredTextParagraph;


use crate::core::internal_representation::*;
use crate::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;
use crate::to_image::drawable::leaf::info::LifelineRequiredHorizontalSpaceInDiagram;
use crate::to_image::extract::instructions::*;
use crate::to_image::extract::extract_rec::*;



fn get_all_involved_lifelines_rec<
    CioII : CommonIoInteractionInterface,
    LI : Eq + Hash + Copy + Clone,
    Context : ContextAwareInteractionDrawingInstructionsExtractor<CioII,LI>
    > (
        context : &Context,
        int_repr : &InteractionInternalRepresentation<CioII>
    ) -> HashSet<LI> {
        match int_repr {
            InteractionInternalRepresentation::LeafPattern(leaf) => {
                context.get_involved_lifelines(leaf)
            },
            InteractionInternalRepresentation::Operator(_, sub_ints) => {
                sub_ints.iter().fold(
                    HashSet::new(), 
                    |mut p,x| {
                        p.extend(get_all_involved_lifelines_rec(context,x).iter());
                        p 
                    }
                )
            }
        }
    }

pub(crate) fn extract_drawing_information<CioII,LI,Context> (
    context : &Context,
    int_repr : &InteractionInternalRepresentation<CioII>
) -> CompleteInteractionDrawInstruction<LI> 
where 
    CioII : CommonIoInteractionInterface,
    LI : Eq + Hash + Copy + Clone,
    Context : ContextAwareInteractionDrawingInstructionsExtractor<CioII,LI>
{

let all_lifelines_in_diagram = {
    let mut all_lifelines : Vec<LI> = get_all_involved_lifelines_rec(context,int_repr).into_iter().collect();
    all_lifelines.sort_by(
        |l1,l2| context.lifelines_compare(l1, l2)
    );
    all_lifelines
};

let mut patterns_to_draw = vec![];
let mut operators_to_draw = vec![];
let mut relative_y_pos = context.get_margin_between_items();
let interaction_intermediate_information = extract_intermediate_drawing_information_rec::<CioII,LI,Context>(
    context,
    &all_lifelines_in_diagram,
    int_repr,
    &mut relative_y_pos,
    0,
    &mut patterns_to_draw,
    &mut operators_to_draw
);
// let us calculate the height
// for that we need at first the header height
let mut header_height = 0.0_f32;
// we compute it by iterating the involved lifelines
// as the same time, we update the horizontal requirements of each lifelines with the width of the header
// and store the header texts
let mut lifelines_headers : HashMap<LI,ColoredTextParagraph> = HashMap::new();
let mut lfs_hor_reqs = interaction_intermediate_information.lfs_horizontal_reqs;
for lf in interaction_intermediate_information.involved_lifelines {
    let para = context.get_lifeline_header(&lf);
    let (text_width, text_height,_) = para.paragraph_size(context.get_scale(), context.get_font());
    {
        let mid_hor_space_req = text_width/2.0;
        lfs_hor_reqs.get_mut(&lf).unwrap().update_to_max(
            LifelineRequiredHorizontalSpaceInDiagram::new(mid_hor_space_req, mid_hor_space_req)
        );
    }
    header_height = f32::max(header_height, text_height);
    lifelines_headers.insert(lf, para);
}
// the vertical space occupied by the top of the diagram
let y_shift_to_absolute = context.get_border_padding() + header_height;
// we can now compute the total image height
let height = (2.0_f32)*context.get_border_padding() + header_height + relative_y_pos + context.get_margin_between_items();


// let us now compute the horizontal positions that we need to know
// it consists of the left side of the diagram, on the left of which to draw input gates
let left_side_of_diagram_x_pos : f32;
// the horizontal positions of each involved lifeline
let mut lifelines_horizontal_positions : HashMap<LI,f32> = HashMap::new();
// the right side of the diagram, on the right of which to draw output gates
let right_side_of_diagram_x_pos : f32;
// and finally the total width of the image
let width : f32;

// we proceed from left to right, incrementing a current x position
let mut current_x_pos = 0.0_f32;
{
    // we start by adding the left padding 
    current_x_pos += context.get_border_padding();
    // then need to deal with the input gates
    current_x_pos += interaction_intermediate_information.max_input_gate_width;
    // ***
    current_x_pos += context.get_margin_between_items();
    left_side_of_diagram_x_pos = current_x_pos;
    current_x_pos += context.get_margin_between_items();
    // ***
    for lf in &all_lifelines_in_diagram {
        let lf_req = lfs_hor_reqs.get(lf).unwrap();
        current_x_pos += lf_req.on_the_left;
        lifelines_horizontal_positions.insert(*lf, current_x_pos);
        current_x_pos += lf_req.on_the_right;
        current_x_pos += context.get_margin_between_items();
    }
    //
    right_side_of_diagram_x_pos = current_x_pos;
    current_x_pos += context.get_margin_between_items();
    //
    current_x_pos += interaction_intermediate_information.max_output_gate_width;
    current_x_pos += context.get_border_padding();
    width = current_x_pos;
}
// 
CompleteInteractionDrawInstruction{
    width,
    height,
    y_shift_to_absolute,
    left_side_of_diagram_x_pos,
    right_side_of_diagram_x_pos,
    lifelines_horizontal_positions,
    lifelines_headers,
    patterns_to_draw,
    operators_to_draw
}
}

