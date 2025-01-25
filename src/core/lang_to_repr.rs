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


use crate::core::internal_representation::{InteractionInternalRepresentation,InteractionOperatorRepresentation,CommonIoInteractionInterface};




pub trait FromInteractionTermToInternalRepresentation<CioII : CommonIoInteractionInterface> : Sized + Clone {

    /** 
     * Returns the operator at the root of the interaction term if it is one.
     * **/
    fn get_operator_at_root(&self) -> Option<CioII::InteractionOperatorType>;

    /** 
     * If at the root of the interaction there is the given operator, then returns its sub-interactions.
     * Otherwise returns None.
     * **/
     fn get_subinteractions_under_operator<'a>(&'a self,operator : &CioII::InteractionOperatorType) -> Option<Vec<&'a Self>>;

    /**
     * If at the root of the interaction there is a given leaf patter, then returns it.
     * Otherwose returns None.
     * **/
    fn identify_pattern_at_interaction_root(&self) -> Option<CioII::InteractionLeafPatternType>;

    /** 
     * A tool function to recursively get all the sub-interactions under an associative operator.
     * For instance, if applied to "f(a,f(b,c))" where "f" is associative, it will return "[a,b,c]".
     * **/
    fn get_associative_operands_recursively<'a>(&'a self,operator : &CioII::InteractionOperatorType) -> Vec<&'a Self> {
        //assert!(operator.is_associative());
        // ***
        let mut operands : Vec<&Self> = Vec::new();
        match self.get_subinteractions_under_operator(operator) {
            None => {
                operands.push(self);
            },
            Some(subints) => {
                for sub_int in subints {
                    operands.extend( sub_int.get_associative_operands_recursively(operator) );
                }
            }
        }
        operands
    }

    /** 
     * Conversion from the concrete interaction language to this crate's internal representation.
     * **/
     fn to_io_repr(&self) -> InteractionInternalRepresentation<CioII> {
        match self.identify_pattern_at_interaction_root() {
            Some(pattern) => {
                InteractionInternalRepresentation::LeafPattern(pattern)
            },
            None => {
                // patterns must cover all non-operator symbols (more precisely all operators of arity 0)
                // so here we must be able to identify the root operator
                let op_at_root = self.get_operator_at_root().unwrap();
                let operands = if op_at_root.is_associative() {
                    self.get_associative_operands_recursively(&op_at_root)
                } else {
                    self.get_subinteractions_under_operator(&op_at_root).unwrap()
                };
                let operands_reprs = operands.into_iter().map(
                    |x| x.to_io_repr()
                ).collect();
                InteractionInternalRepresentation::Operator(op_at_root, operands_reprs)
            }
        }
     }

}















