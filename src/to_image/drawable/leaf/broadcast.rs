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

use image_colored_text::text::paragraph::ColoredTextParagraph;
use crate::to_image::drawable::leaf::util::MessageExchangeLineStyle;


/** 
 * Instruction to draw an action with preamble and postamble.
 * **/
pub struct PrePostAmbleDrawableActionItem {
    pub preamble : Option<ColoredTextParagraph>,
    pub postamble : Option<ColoredTextParagraph>
}


impl PrePostAmbleDrawableActionItem {

    pub fn new(
        preamble : Option<ColoredTextParagraph>,
        postamble : Option<ColoredTextParagraph>
    ) -> Self {
        Self{preamble,postamble}
    }

    pub fn get_size_around_midline(
        &self, 
        scale: impl Into<PxScale> + Copy,
        font: &impl Font) -> (f32,f32,f32,f32) 
    {
        let (pre_w,pre_h) = if let Some(preamble) = &self.preamble {
            let (preamble_width, preamble_height, _) = preamble.paragraph_size(scale, font);
            (preamble_width,preamble_height)
        } else {
            (0.0,0.0)
        };
        let (post_w,post_h) = if let Some(postamble) = &self.postamble {
            let (postamble_width, postamble_height, _) = postamble.paragraph_size(scale, font);
            (postamble_width,postamble_height)
        } else {
            (0.0,0.0)
        };
        (pre_w,pre_h,post_w,post_h)
    }

}


/** 
 * Instruction to draw an action centered around the midline.
 * **/
 pub struct CenteredDrawableActionItem {
    pub content : ColoredTextParagraph,
}

impl CenteredDrawableActionItem {

    pub fn new(content : ColoredTextParagraph) -> Self {
        Self{content}
    }

    pub fn get_size_around_midline(
        &self, 
        scale: impl Into<PxScale> + Copy,
        font: &impl Font) -> (f32,f32,f32,f32) 
    {
        let (width, height, _) = self.content.paragraph_size(scale, font);
        (width,height/2.0,width,height/2.0)
    }

}

/** 
 * How to draw the reception of a message on a specific lifeline.
 * **/
pub enum TargetLifelineBroadcastDrawInstruction {
    TwoParts(PrePostAmbleDrawableActionItem),
    Centered(CenteredDrawableActionItem),
}

impl TargetLifelineBroadcastDrawInstruction {

    pub fn get_size_around_midline(
        &self, 
        scale: impl Into<PxScale> + Copy,
        font: &impl Font) -> (f32,f32,f32,f32) 
    {
        match &self {
            TargetLifelineBroadcastDrawInstruction::TwoParts(ref act) => {
                act.get_size_around_midline(scale, font)
            },
            TargetLifelineBroadcastDrawInstruction::Centered(ref act) => {
                act.get_size_around_midline(scale, font)
            }
        }
    }

}




pub enum DrawableBroadcastLeafPatternOrigin<LifelineIdentifier : Eq + Hash + Copy + Clone> {
    Empty,
    Lifeline(LifelineIdentifier,PrePostAmbleDrawableActionItem),
    InputOutsideGate(ColoredTextParagraph)
}



/** 
 * Instructions to draw a broadcast pattern.
 * **/
pub struct DrawableBroadcastLeafPattern<LifelineIdentifier : Eq + Hash + Copy + Clone> {
    // how to draw the message on top of the horizontal line
    pub message : ColoredTextParagraph,
    // how to draw the horizontal line
    pub line_style : MessageExchangeLineStyle,
    // instructions to draw the origin
    pub origin : DrawableBroadcastLeafPatternOrigin<LifelineIdentifier>,
    // instructions to draw the lifeline targets of the horizontal line
    pub lifeline_targets : HashMap<LifelineIdentifier,TargetLifelineBroadcastDrawInstruction>,
    // instructions to draw the targets that are not lifelines but output gates
    pub output_outside_gates_targets : Vec<ColoredTextParagraph>
}

impl<LI : Eq + Hash + Copy + Clone>  DrawableBroadcastLeafPattern<LI> {

    pub fn new(
        message : ColoredTextParagraph,
        line_style : MessageExchangeLineStyle,
        origin : DrawableBroadcastLeafPatternOrigin<LI>,
        lifeline_targets : HashMap<LI,TargetLifelineBroadcastDrawInstruction>,
        output_outside_gates_targets : Vec<ColoredTextParagraph>) -> Self {
        Self { message, line_style, origin, lifeline_targets, output_outside_gates_targets }
    }

}




