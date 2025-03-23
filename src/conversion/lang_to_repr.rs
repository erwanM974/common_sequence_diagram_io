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


use crate::internal_representation::{InteractionInternalRepresentation,InteractionOperatorRepresentation,CommonIoInteractionInterface};




pub trait FromInteractionTermToInternalRepresentation<CioII : CommonIoInteractionInterface> : Sized + Clone {

    /** 
     * Returns the operator at the root of the interaction term if it is one.
     * **/
    fn get_operator_at_root(&self) -> Option<CioII::InteractionOperatorType>;

    /** 
     * Returns the sub-interactions of the given interaction.
     * If it is an operator symbol of arity 0, it returns an empty vec![].
     * **/
    fn get_subinteractions(&self) -> Vec<&Self>;

    /**
     * If at the root of the interaction there is a given leaf patter, then returns it.
     * Otherwise returns None.
     * **/
    fn identify_pattern_at_interaction_leaf(&self) -> Option<CioII::InteractionLeafPatternType>;

    /** 
     * If possible, merges two patterns that have been found and that are linked by a certain operator.
     * **/
    fn merge_patterns_under_operator_if_possible(
        parent_op : &CioII::InteractionOperatorType,
        p1 : &CioII::InteractionLeafPatternType,
        p2 : &CioII::InteractionLeafPatternType
    ) -> Option<CioII::InteractionLeafPatternType>;

    /** 
     * A tool function to recursively get all the sub-interactions under an associative operator.
     * For instance, if applied to "f(a,f(b,c))" where "f" is associative, it will return "[a,b,c]".
     * **/
     fn get_associative_operands_recursively<'a>(
        &'a self,
        considered_associative_operator : &CioII::InteractionOperatorType
    ) -> Vec<&'a Self> {
        // ***
        let consider_sub_interactions = match self.get_operator_at_root() {
            None => {
                false
            },
            Some(got_at_root) => {
                &got_at_root == considered_associative_operator
            }
        };
        // ***
        let mut operands : Vec<&Self> = Vec::new();
        // ***
        if consider_sub_interactions {
            for sub_int in self.get_subinteractions() {
                operands.extend( 
                    sub_int.get_associative_operands_recursively(considered_associative_operator) 
                );
            }
        } else {
            operands.push(self);
        }
        // ***
        operands
    }

    /** 
     * Conversion from the concrete interaction language to this crate's internal representation.
     * **/
     fn to_io_repr(
        &self,
        merge_patterns : bool,
        flatten_operands_under_associative_operators : bool
    ) -> InteractionInternalRepresentation<CioII> {
        match self.identify_pattern_at_interaction_leaf() {
            Some(pattern) => {
                let pattern_int_repr = InteractionInternalRepresentation::LeafPattern(pattern);
                pattern_int_repr
            },
            None => {
                // patterns must cover all non-operator symbols (more precisely all operators of arity 0)
                // so here we must be able to identify the root operator
                let op_at_root = self.get_operator_at_root().unwrap();
                let raw_operands = if flatten_operands_under_associative_operators && op_at_root.is_associative() {
                    self.get_associative_operands_recursively(&op_at_root)
                } else {
                    self.get_subinteractions()
                };
                let mut operands = vec![];
                let mut last_pattern : Option<<CioII as CommonIoInteractionInterface>::InteractionLeafPatternType> = None;
                for raw_op in raw_operands {
                    let operand_io_repr = raw_op.to_io_repr(
                        merge_patterns,
                        flatten_operands_under_associative_operators
                    );
                    match operand_io_repr {
                        InteractionInternalRepresentation::LeafPattern(pt) => {
                            match last_pattern {
                                Some(prev_pt) => {
                                    if merge_patterns {
                                        match Self::merge_patterns_under_operator_if_possible(
                                            &op_at_root, 
                                            &prev_pt, 
                                            &pt
                                        ) {
                                            None => {
                                                operands.push(
                                                    InteractionInternalRepresentation::LeafPattern(prev_pt)
                                                );
                                                last_pattern = Some(pt);
                                            },
                                            Some(merged_pt) => {
                                                last_pattern = Some(merged_pt);
                                            }
                                        }
                                    } else {
                                        operands.push(
                                            InteractionInternalRepresentation::LeafPattern(prev_pt)
                                        );
                                        last_pattern = Some(pt);
                                    }
                                },
                                None => {
                                    last_pattern = Some(pt);
                                }
                            }
                        },
                        _ => {
                            if let Some(prev_pt) = last_pattern {
                                operands.push(
                                    InteractionInternalRepresentation::LeafPattern(prev_pt)
                                );
                                last_pattern = None;
                            }
                            operands.push(operand_io_repr);
                        }
                    }
                }
                // ***
                if let Some(pt) = last_pattern {
                    operands.push(
                        InteractionInternalRepresentation::LeafPattern(pt)
                    );
                }
                // ***
                debug_assert!(!operands.is_empty());
                if operands.len() == 1 && op_at_root.arity() >= 2 {
                    // here we have an associative binary operator which subterms amounts to a single pattern
                    // so we only keep the pattern
                    operands.pop().unwrap()
                } else {
                    InteractionInternalRepresentation::Operator(op_at_root, operands)
                }
            }
        }
     }

}















