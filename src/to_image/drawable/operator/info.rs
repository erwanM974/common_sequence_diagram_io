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







/**
 * Intermediate information that is gathered recursively at each operator
 * of the interaction internal representation and is required to draw the interaction.
 * This consists in:
 * - the baseline vertical space required at the top
 * - the vertical space in between each operand
 * - whether or not drawing it required a nested xshit (for drawing nested operators)
 * - the horizontal space required to draw the operator name on the left of the leftmost included lifeline
 **/
 pub struct OperatorIntermediateInformation {
    pub required_vertical_space_at_the_top : f32,
    pub required_vertical_space_between_operands : f32,
    pub requires_nest_shift : bool,
    pub required_horizontal_space_at_left_most_lifeline : f32
}
    
    
