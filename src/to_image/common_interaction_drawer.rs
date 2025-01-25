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


use ab_glyph::{Font, PxScale};

pub trait CommonInteractionDrawerTrait {

    fn get_scale(&self) -> impl Into<PxScale> + Copy;
    
    fn get_font(&self) -> &impl Font;

    fn get_y_margin_between_seq_operands(&self) -> f32;
    fn get_margin_between_items(&self) -> f32;

    fn get_border_padding(&self) -> f32;

}



