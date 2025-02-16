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
use maplit::hashset;


use crate::internal_representation::InteractionInternalRepresentation;
use crate::tests::common::*;
use crate::to_image::common_interaction_drawer::CommonInteractionDrawerTrait;
use crate::to_image::draw::context_aware_drawer::ContextAwareInteractionDrawer;
use crate::to_image::drawable::leaf::util::MessageExchangeLineStyle;
use crate::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;

use crate::tests::lang_colorful::to_image::colorful_colors::*;
use crate::tests::lang_colorful::colorful_lang::*;
use crate::tests::lang_colorful::core::internal_representation::{ColorfulOperators, ColorfulLangCioII, ColorfulLeafPattern};
use crate::to_image::drawable::leaf::broadcast::*;
use crate::to_image::drawable::operator::builtin_operator::{DrawableOperator, DrawableOperatorKind};


use crate::to_image::draw::util::draw_uniform_colored_background;




pub struct ColorfulDrawingContext {
    pub color_context : ColorfulContext,
    pub font : FontRef<'static>,
    pub y_margin_between_seq_operands : f32,
    pub margin_between_items : f32,
    pub border_padding : f32,
    pub arrowhead_length : f32
}

impl ColorfulDrawingContext {

    pub fn new(color_context : ColorfulContext) -> ColorfulDrawingContext {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        let y_margin_between_seq_operands = 11.0;
        let margin_between_items = 6.0;
        let border_padding = 10.0;
        let arrowhead_length = 10.0;
        ColorfulDrawingContext{
            color_context,
            font,
            y_margin_between_seq_operands,
            margin_between_items,
            border_padding,
            arrowhead_length
        }
    }
}


impl CommonInteractionDrawerTrait for ColorfulDrawingContext {

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

impl ContextAwareInteractionDrawingInstructionsExtractor<ColorfulLangCioII,usize> for ColorfulDrawingContext {
    fn lifelines_compare(&self, l1 : &usize, l2 : &usize) -> std::cmp::Ordering {
        l1.cmp(l2)
    }

    fn get_involved_lifelines(&self, pattern : &ColorfulLeafPattern) -> HashSet<usize> {
        let mut lfs = HashSet::new();
        if let GateOrLifeline::Lifeline(l1) = pattern.origin.item {
            lfs.insert(l1);
        }
        if let GateOrLifeline::Lifeline(l2) = pattern.target.item {
            lfs.insert(l2);
        }
        lfs 
    }

    fn get_lifeline_header(&self, l : &usize) -> ColoredTextParagraph {
        self.color_context.lf_descs.get(*l).unwrap().clone()
    }

    fn to_drawable_pattern(&self, pattern : &ColorfulLeafPattern) -> Option<DrawableBroadcastLeafPattern<usize>> {
        // retrieve the message label
        let ms_name = self.color_context.ms_names.get(pattern.msg_id).unwrap();
        let message = if ms_name == "secret" {
            ColoredTextParagraph::new(
                vec![
                    ColoredTextLine::new(vec![("This is the".to_owned(),Rgb(COLORFUL_SECRET_MESSAGE_COLOR))]),
                    ColoredTextLine::new(vec![("secret message".to_owned(),Rgb(COLORFUL_SECRET_MESSAGE_COLOR))])
                    ],
                MultiLineTextAlignment::Center, 
                Some(Rgb(COLORFUL_SECRET_MESSAGE_BACKGROUND)), 
                Some(Rgb(COLORFUL_SECRET_MESSAGE_BORDER))
            )
        } else {
            ColoredTextParagraph::new(
                vec![ColoredTextLine::new(vec![(ms_name.to_owned(),Rgb(COLORFUL_MESSAGE))])],
                MultiLineTextAlignment::Center, 
                None, 
                None
            )
        };
        let mut involves_gate = false;
        let origin = match pattern.origin.item {
            GateOrLifeline::Gate(gt_id) => {
                involves_gate = true;
                let gt_name = self.color_context.gt_names.get(gt_id).unwrap();
                let para = ColoredTextParagraph::new(
                    vec![ColoredTextLine::new(vec![(gt_name.to_owned(),Rgb(COLORFUL_GATE))])],
                    MultiLineTextAlignment::Center, 
                    None, 
                    Some(Rgb(COLORFUL_BLACK))
                );
                DrawableBroadcastLeafPatternOrigin::InputOutsideGate(para)
            },
            GateOrLifeline::Lifeline(lf_id) => {
                DrawableBroadcastLeafPatternOrigin::Lifeline(
                    lf_id, 
                    PrePostAmbleDrawableActionItem::new(
                        pattern.origin.notes.0.clone(),
                        pattern.origin.notes.1.clone()
                    )
                )
            }
        };
        let mut lifeline_targets : HashMap<usize,TargetLifelineBroadcastDrawInstruction> = HashMap::new();
        let mut gate_targets = vec![];
        match pattern.target.item {
            GateOrLifeline::Gate(gt_id) => {
                involves_gate = true;
                let gt_name = self.color_context.gt_names.get(gt_id).unwrap();
                let para = ColoredTextParagraph::new(
                    vec![ColoredTextLine::new(vec![(gt_name.to_owned(),Rgb(COLORFUL_GATE))])],
                    MultiLineTextAlignment::Center, 
                    None, 
                    Some(Rgb(COLORFUL_BLACK))
                );
                gate_targets.push(para);
            },
            GateOrLifeline::Lifeline(lf_id) => {
                let pre_post = PrePostAmbleDrawableActionItem::new(
                    pattern.target.notes.0.clone(),
                    pattern.target.notes.1.clone()
                );
                lifeline_targets.insert(
                    lf_id,
                    TargetLifelineBroadcastDrawInstruction::TwoParts(pre_post)
                );
            }
        };
        let line_style = MessageExchangeLineStyle::new(
            ms_name.len() > 5, 
            involves_gate, 
            Rgb(COLORFUL_BLACK), 
            self.arrowhead_length
        );
        Some(DrawableBroadcastLeafPattern::new(message,line_style,origin,lifeline_targets,gate_targets))
    }

    fn to_drawable_operator(&self, op : &ColorfulOperators, _sub_ints : &[InteractionInternalRepresentation<ColorfulLangCioII>]) -> DrawableOperator<usize> {
        match op {
            &ColorfulOperators::Coreg(None) => {
                DrawableOperator::new(Rgb(COLORFUL_BLACK),DrawableOperatorKind::CoRegionLike(HashSet::new()))
            },
            &ColorfulOperators::Coreg(Some(lf_id)) => {
                DrawableOperator::new(Rgb(COLORFUL_BLACK),DrawableOperatorKind::CoRegionLike(hashset!{lf_id}))
            },
            ColorfulOperators::TPC => {
                let op_label = ColoredTextParagraph::new(
                    vec![
                        ColoredTextLine::new(vec![("TEAM PAIN".to_owned(),Rgb(COLORFUL_BLACK))]),
                        ColoredTextLine::new(vec![("AU CHOCOLAT".to_owned(),Rgb(COLORFUL_CHOCOLAT))]),
                        ], 
                    MultiLineTextAlignment::Center,
                    Some(Rgb(COLORFUL_PAIN)),
                    Some(Rgb(COLORFUL_CHOCOLAT))
                );
                DrawableOperator::new(Rgb(COLORFUL_BLACK),DrawableOperatorKind::Framed(op_label))
            },
            ColorfulOperators::Rougail(kind) => {
                let rougail_color = if kind == "tomate" {
                    COLORFUL_ROUGAIL_TOMATE
                } else {
                    COLORFUL_DEFAULT_ROUGAIL
                };
                let op_label = ColoredTextParagraph::new(
                    vec![
                        ColoredTextLine::new(vec![("rougail".to_owned(),Rgb(COLORFUL_BLACK))]),
                        ColoredTextLine::new(vec![(kind.to_owned(),Rgb(rougail_color))]),
                        ], 
                    MultiLineTextAlignment::Left,
                    None,
                    None
                );
                DrawableOperator::new(Rgb(rougail_color),DrawableOperatorKind::Framed(op_label))
            },
            ColorfulOperators::Brocoli => {
                let op_label = ColoredTextParagraph::new(
                    vec![
                        ColoredTextLine::new(vec![("brocoli".to_owned(),Rgb(COLORFUL_BLACK))])
                        ], 
                    MultiLineTextAlignment::Center,
                    Some(Rgb(COLORFUL_BROCOLI)),
                    None
                );
                DrawableOperator::new(Rgb(COLORFUL_BROCOLI),DrawableOperatorKind::Framed(op_label))
            }
        }
    }

}




impl ContextAwareInteractionDrawer<usize> for ColorfulDrawingContext {
    fn draw_background(&self, image : &mut image::RgbImage, img_width : f32, img_height : f32) {
        draw_uniform_colored_background(image,&img_width,&img_height,Rgb(COLORFUL_WHITE));
    }
    
    fn get_lifelines_colors(&self, involved_lifelines : &[usize]) -> HashMap<usize,Rgb<u8>> {
        let mut lifelines_colors = HashMap::new();
        for lf in involved_lifelines {
            lifelines_colors.insert(*lf,Rgb(COLORFUL_BLACK));
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