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



use std::path::Path;
use std::hash::Hash;


use crate::internal_representation::*;
use crate::to_image::extract::context_aware_extractor::ContextAwareInteractionDrawingInstructionsExtractor;
use crate::to_image::draw::context_aware_drawer::ContextAwareInteractionDrawer;

use crate::to_image::extract::extract::extract_drawing_information;
use crate::to_image::draw::draw::make_image_from_display_information;

pub fn draw_interaction_as_sequence_diagram<CioII,LI,Extractor,Drawer> (
    int_repr : &InteractionInternalRepresentation<CioII>,
    extractor : &Extractor,
    palette : &Drawer,
    file_path : &Path,
)
where 
    CioII : CommonIoInteractionInterface,
    LI : Eq + Hash + Copy + Clone,
    Extractor : ContextAwareInteractionDrawingInstructionsExtractor<CioII,LI>,
    Drawer : ContextAwareInteractionDrawer<LI>
{

    let display_info = extract_drawing_information::<CioII,LI,Extractor>(
        extractor,
        int_repr
    );
    let image = make_image_from_display_information::<LI,Drawer>(
        palette,
        &display_info
    );
    let _ = image.save(file_path);
}




