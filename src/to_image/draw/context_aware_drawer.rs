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

use image::{Rgb, RgbImage};

use crate::to_image::common_interaction_drawer::CommonInteractionDrawerTrait;

pub trait ContextAwareInteractionDrawer<
        LI : Eq + Hash + Copy + Clone
    > : CommonInteractionDrawerTrait {  

    fn draw_background(&self, image : &mut RgbImage, img_width : f32, img_height : f32);    

    fn get_lifelines_colors(&self, involved_lifelines : &[LI]) -> HashMap<LI,Rgb<u8>>;

    fn get_arrow_length(&self) -> f32;

    fn get_nest_padding_unit(&self) -> f32;

}


