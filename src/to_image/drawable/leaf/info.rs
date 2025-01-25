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



/**
 * Intermediate information that is gathered recursively at each leaf (e.g., message passing, broadcast etc.)
 * of the interaction internal representation and is required to draw the interaction.
 **/
 pub struct BroadcastLeafPatternIntermediateInformation<LifelineIdentifier : Eq + Hash + Copy + Clone> {
    // the total height (required vertical space from top to bottom)
    pub y_space_top_to_bottom : f32,
    // the required vertical space from top to the midline (i.e. the y position at which to draw the horizontal line)
    pub y_space_top_to_midline : f32,
    // where to draw the message
    pub message_drawing_location : (MessageDrawingLocation<LifelineIdentifier>,f32),
    // keeps track of the lifelines that are directly involved in the broadcast pattern
    pub involved_lifelines : HashSet<LifelineIdentifier>,
    // for each lifeline the horizontal requirements (space required on its left and right)
    // this may include more lifelines than only those that are directly involved because the width of 
    // the message to display may be shared between neighboring lifelines
    pub lifelines_horizontal_requirements : HashMap<LifelineIdentifier,LifelineRequiredHorizontalSpaceInDiagram>,
    // in case there is a target on the same lifeline as the emission, 
    // we need to keep track of the required space under the midline to draw the emission
    pub required_space_under_emission : Option<(LifelineIdentifier, f32)>,
    // width of the input gate (0.0 if there is no input gate)
    pub input_gate_width : f32,
    // max width of the output gates (0.0 if there are none)
    pub output_gates_max_width : f32,
    // vertical space above the midline on which to start drawing the output gates from top to bottom
    pub y_shift_above_midline_for_output_gates : f32
}

impl<LifelineIdentifier : Eq + Hash + Copy + Clone> BroadcastLeafPatternIntermediateInformation<LifelineIdentifier> {

    pub fn new(
        y_space_top_to_bottom : f32,
        y_space_top_to_midline : f32,
        message_drawing_location : (MessageDrawingLocation<LifelineIdentifier>,f32),
        involved_lifelines : HashSet<LifelineIdentifier>,
        lifelines_horizontal_requirements : HashMap<LifelineIdentifier,LifelineRequiredHorizontalSpaceInDiagram>,
        required_space_under_emission : Option<(LifelineIdentifier, f32)>,
        input_gate_width : f32,
        output_gates_max_width : f32,
        y_shift_above_midline_for_output_gates : f32
    ) -> BroadcastLeafPatternIntermediateInformation<LifelineIdentifier> {
        BroadcastLeafPatternIntermediateInformation{
            y_space_top_to_bottom,
            y_space_top_to_midline,
            message_drawing_location,
            involved_lifelines,
            lifelines_horizontal_requirements,
            required_space_under_emission,
            input_gate_width,
            output_gates_max_width,
            y_shift_above_midline_for_output_gates
        }
        }
}


pub struct MessageDrawingLocation<LifelineIdentifier : Eq + Hash + Copy + Clone> {
    pub anchor_lifeline : LifelineIdentifier,
    // if *draw_message_on_left* is true draw on the left of the lifeline, otherwise on the right
    pub draw_message_on_left : bool
}

impl<LifelineIdentifier : Eq + Hash + Copy + Clone> MessageDrawingLocation<LifelineIdentifier> {
    pub fn new(anchor_lifeline : LifelineIdentifier,draw_message_on_left : bool) -> MessageDrawingLocation<LifelineIdentifier> {
        MessageDrawingLocation{anchor_lifeline,draw_message_on_left}
    }
}

/** 
 * Intermediate information propagated and updated during the extraction.
 * It concerns the minimum horizontal space that is required on the left and the right side of a lifeline.
 * **/
 #[derive(Clone)]
pub struct LifelineRequiredHorizontalSpaceInDiagram {
    pub on_the_left : f32,
    pub on_the_right : f32
}

impl LifelineRequiredHorizontalSpaceInDiagram {

    pub fn new(on_the_left : f32,on_the_right : f32) -> LifelineRequiredHorizontalSpaceInDiagram {
        LifelineRequiredHorizontalSpaceInDiagram{on_the_left,on_the_right}
    }

    pub fn new_empty() -> LifelineRequiredHorizontalSpaceInDiagram {
        LifelineRequiredHorizontalSpaceInDiagram::new(0.0,0.0)
    }

    pub fn update_to_max(&mut self, other : Self) {
        self.on_the_left = f32::max(self.on_the_left, other.on_the_left);
        self.on_the_right = f32::max(self.on_the_right, other.on_the_right);
    }

    pub fn update_all_to_max<LI : Eq + Hash>(
        req1 : &mut HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>,
        req2 : HashMap<LI,LifelineRequiredHorizontalSpaceInDiagram>) {
        for (lf,r2) in req2 {
            match req1.get_mut(&lf) {
                None => {
                    req1.insert(lf,r2);
                },
                Some(r1) => {
                    r1.update_to_max(r2);
                }
            }
        }
    }

}

