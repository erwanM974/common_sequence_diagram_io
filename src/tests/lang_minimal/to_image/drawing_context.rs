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


use std::collections::{HashMap,HashSet};

use ab_glyph::{Font, FontRef, PxScale};
use image::Rgb;
use image_colored_text::text::paragraph::*;
use image_colored_text::text::line::ColoredTextLine;


use crate::tests::common::*;
use crate::tests::lang_minimal::minimal_lang::GeneralContext;
use crate::to_image::common_interaction_drawer::CommonInteractionDrawerTrait;
use crate::to_image::draw::context_aware_drawer::ContextAwareInteractionDrawer;
use crate::to_image::drawable::leaf::util::MessageExchangeLineStyle;
use crate::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;

use crate::tests::lang_minimal::core::internal_representation::{MinimalOperators, MinimalLangCioII, MinimalLeafPattern};
use crate::to_image::drawable::leaf::broadcast::*;
use crate::to_image::drawable::operator::builtin_operator::{DrawableOperator, DrawableOperatorKind};


use crate::to_image::draw::util::draw_uniform_colored_background;

pub const MY_COLOR_WHITE : [u8;3] = [255u8,  255u8,  255u8];
pub const MY_COLOR_BLACK : [u8;3] = [0u8, 0u8, 0u8];
pub const MY_COLOR_LIFELINE : [u8;3] = [22u8, 22u8, 130u8];
pub const MY_COLOR_MESSAGE : [u8;3] = [15u8, 86u8, 15u8];


pub struct MinimalDrawingContext {
    pub general_context : GeneralContext,
    pub font : FontRef<'static>,
    pub y_margin_between_seq_operands : f32,
    pub margin_between_items : f32,
    pub border_padding : f32,
    pub arrowhead_length : f32
}

impl MinimalDrawingContext {

    pub fn new(general_context : GeneralContext) -> MinimalDrawingContext {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        let y_margin_between_seq_operands = 11.0;
        let margin_between_items = 6.0;
        let border_padding = 10.0;
        let arrowhead_length = 10.0;
        MinimalDrawingContext{
            general_context,
            font,
            y_margin_between_seq_operands,
            margin_between_items,
            border_padding,
            arrowhead_length
        }
    }
}


impl CommonInteractionDrawerTrait for MinimalDrawingContext {

    fn get_scale(&self) -> impl Into<PxScale> + Copy {
        SCALE
    }

    fn get_font(&self) -> &impl Font {
        &self.font
    }

    fn get_y_margin_between_seq_operands(&self) -> f32 {
        self.y_margin_between_seq_operands
    }

    fn get_margin_between_items(&self) -> f32 {
        self.margin_between_items
    }

    fn get_border_padding(&self) -> f32 {
        self.border_padding
    }

}

impl ContextAwareInteractionDrawingInstructionsExtractor<MinimalLangCioII,usize> for MinimalDrawingContext {
    fn lifelines_compare(&self, l1 : &usize, l2 : &usize) -> std::cmp::Ordering {
        l1.cmp(l2)
    }

    fn get_involved_lifelines(&self, pattern : &MinimalLeafPattern) -> HashSet<usize> {
        match pattern{
            MinimalLeafPattern::BROADCAST(ref brd) => {
                let mut lfs = HashSet::new();
                if let Some(orig_lf) = &brd.origin_lf_id {
                    lfs.insert(*orig_lf);
                }
                for lf in &brd.targets {
                    lfs.insert(*lf);
                }
                lfs 
            },
            MinimalLeafPattern::EMPTY => {
                HashSet::new()
            }
        }
    }

    fn get_lifeline_header(&self, l : &usize) -> ColoredTextParagraph {
        let lf_name = self.general_context.lf_names.get(*l).unwrap();
        ColoredTextParagraph::new(
            vec![ColoredTextLine::new(vec![(lf_name.to_owned(),Rgb(MY_COLOR_LIFELINE))])],
            MultiLineTextAlignment::Center, 
            Some(Rgb(MY_COLOR_WHITE)), 
            Some(Rgb(MY_COLOR_BLACK))
        )
    }

    fn to_drawable_pattern(&self, pattern : &MinimalLeafPattern) -> Option<DrawableBroadcastLeafPattern<usize>> {
        match pattern {
            MinimalLeafPattern::BROADCAST(ref brd) => {
                // retrieve the message label
                let ms_name = self.general_context.ms_names.get(brd.msg_id).unwrap();
                let message = ColoredTextParagraph::new(
                    vec![ColoredTextLine::new(vec![(ms_name.to_owned(),Rgb(MY_COLOR_MESSAGE))])],
                    MultiLineTextAlignment::Center, 
                    None, 
                    None
                );
                let line_style = MessageExchangeLineStyle::new(
                    false, 
                    false, 
                    Rgb(MY_COLOR_BLACK), 
                    self.arrowhead_length
                );
                let origin = match brd.origin_lf_id {
                    None => {
                        DrawableBroadcastLeafPatternOrigin::Empty
                    },
                    Some(lf_id) => {
                        DrawableBroadcastLeafPatternOrigin::Lifeline(lf_id, PrePostAmbleDrawableActionItem::new(None,None))
                    }
                };
                let mut targets : HashMap<usize,TargetLifelineBroadcastDrawInstruction> = HashMap::new();
                for lf in &brd.targets {
                    let empty_paragraph = ColoredTextParagraph::new(
                        vec![], 
                        MultiLineTextAlignment::Center,
                        None,
                        None
                    );
                    targets.insert(
                        *lf, 
                        TargetLifelineBroadcastDrawInstruction::Centered(CenteredDrawableActionItem::new(empty_paragraph))
                    );
                }
                Some(DrawableBroadcastLeafPattern::new(message,line_style,origin,targets,vec![]))
            },
            MinimalLeafPattern::EMPTY => {
                None
            }
        }
    }

    fn to_drawable_operator(&self, op : &MinimalOperators) -> DrawableOperator<usize> {
        if op == &MinimalOperators::Seq {
            DrawableOperator::new(Rgb(MY_COLOR_BLACK),DrawableOperatorKind::CoRegionLike(HashSet::new()))
        } else {
            let op_label = ColoredTextParagraph::new(
                vec![ColoredTextLine::new(vec![(op.as_lowercase_string(),Rgb(MY_COLOR_BLACK))])], 
                MultiLineTextAlignment::Center,
                None,
                None
            );
            DrawableOperator::new(Rgb(MY_COLOR_BLACK),DrawableOperatorKind::Framed(op_label))
        }
    }

}




impl ContextAwareInteractionDrawer<usize> for MinimalDrawingContext {
    fn draw_background(&self, image : &mut image::RgbImage, img_width : f32, img_height : f32) {
        draw_uniform_colored_background(image,&img_width,&img_height,Rgb(MY_COLOR_WHITE));
    }
    
    fn get_lifelines_colors(&self, involved_lifelines : &[usize]) -> HashMap<usize,Rgb<u8>> {
        let mut lifelines_colors = HashMap::new();
        for lf in involved_lifelines {
            lifelines_colors.insert(*lf,Rgb(MY_COLOR_BLACK));
        }
        lifelines_colors
    }

    fn get_arrow_length(&self) -> f32 {
        20.0
    }

    fn get_nest_padding_unit(&self) -> f32 {
        3.0
    }
}