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


use crate::internal_representation::CommonIoInteractionInterface;
use crate::internal_representation::InteractionInternalRepresentation;


pub trait ContextAwareInteractionPrinter<CioII : CommonIoInteractionInterface> {

    fn left_parenthesis(&self) -> &str;
    fn right_parenthesis(&self) -> &str;
    fn operand_separator(&self) -> &str;

    /** 
     * Returns how to print the given operator, also taking into account the sub-interactions underneath.
     * **/
    fn print_operator(
        &self, 
        operator : &CioII::InteractionOperatorType,
        sub_ints : &[InteractionInternalRepresentation<CioII>]
    ) -> String;

    fn print_explicit_pattern(&self, leaf_pattern : &CioII::InteractionLeafPatternType) -> String;

    fn print_interaction_inner(&self,
        depth : usize,
        int : &InteractionInternalRepresentation<CioII>
    ) -> String {
        match int {
            InteractionInternalRepresentation::LeafPattern(leaf) => {
                format!("{}{}", "\t".repeat(depth), self.print_explicit_pattern(leaf))
            },
            InteractionInternalRepresentation::Operator(op, sub_ints ) => {
                let first_line : &str = &format!("{}{}{}", "\t".repeat(depth), self.print_operator(op,sub_ints), self.left_parenthesis());
                let inner_lines : Vec<String> = sub_ints.iter()
                .map(|sub_int| self.print_interaction_inner(depth + 1, sub_int))
                .collect();
                let inner_lines_sep = format!("{}\n", self.operand_separator());
                let last_line : &str = &format!("{}{}", "\t".repeat(depth), self.right_parenthesis());
                format!("{}\n{}\n{}", first_line.to_owned(), inner_lines.join(&inner_lines_sep), last_line)
            }
            
        }
    }

}













