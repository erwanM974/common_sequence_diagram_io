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
use std::collections::HashSet;


use image_colored_text::text::paragraph::ColoredTextParagraph;

use crate::core::internal_representation::CommonIoInteractionInterface;
use crate::to_image::common_interaction_drawer::CommonInteractionDrawerTrait;
use crate::to_image::drawable::operator::builtin_operator::DrawableOperator;
use crate::to_image::drawable::leaf::broadcast::DrawableBroadcastLeafPattern;

pub trait ContextAwareInteractionDrawingInstructionsExtractor<
        CioII : CommonIoInteractionInterface, 
        LI : Eq + Hash + Copy + Clone
        > : CommonInteractionDrawerTrait {

    fn lifelines_compare(&self, l1 : &LI, l2 : &LI) -> std::cmp::Ordering;

    fn get_involved_lifelines(&self, pattern : &CioII::InteractionLeafPatternType) -> HashSet<LI>;      

    fn get_lifeline_header(&self, l : &LI) -> ColoredTextParagraph;

    /** 
     * Converts a leaf pattern into a drawable broadcast leaf pattern.
     * The *None* case corresponds to the empty interaction.
     * **/
    fn to_drawable_pattern(&self, pattern : &CioII::InteractionLeafPatternType) -> Option<DrawableBroadcastLeafPattern<LI>>;

    fn to_drawable_operator(&self, op : &CioII::InteractionOperatorType) -> DrawableOperator<LI>;

}


