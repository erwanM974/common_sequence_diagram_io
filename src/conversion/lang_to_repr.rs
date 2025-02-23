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
     * If found, we may also return a "remainder".
     * For instance, if we have a term of the form OP(x,OP(y,z)) where OP(x,y) can be identifier as a pattern P, then
     * it will return Some(P,Some(OP,z))
     * **/
    fn identify_pattern_at_interaction_root<'a>(&'a self) -> Option<(CioII::InteractionLeafPatternType,Option<(CioII::InteractionOperatorType,&'a Self)>)>;

    /** 
     * A tool function to recursively get all the sub-interactions under an associative operator.
     * For instance, if applied to "f(a,f(b,c))" where "f" is associative, it will return "[a,b,c]".
     * **/
     fn get_associative_operands_recursively(
        &self,
        considered_associative_operator : &CioII::InteractionOperatorType
    ) -> Vec<InteractionInternalRepresentation<CioII>> {
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
        let mut operands : Vec<InteractionInternalRepresentation<CioII>> = Vec::new();
        // ***
        if consider_sub_interactions {
            for sub_int in self.get_subinteractions() {
                match sub_int.identify_pattern_at_interaction_root() {
                    Some((pattern,may_remain)) => {
                        match may_remain {
                            Some((x_op,x)) => {
                                if &x_op == considered_associative_operator {
                                    operands.push(InteractionInternalRepresentation::LeafPattern(pattern));
                                    operands.extend(
                                        x.get_associative_operands_recursively(considered_associative_operator) 
                                    );
                                } else {
                                    operands.extend( 
                                        sub_int.get_associative_operands_recursively(considered_associative_operator) 
                                    );
                                }
                            },
                            None => {
                                operands.push(InteractionInternalRepresentation::LeafPattern(pattern));
                            },
                        }
                    },
                    None => {
                        operands.extend( 
                            sub_int.get_associative_operands_recursively(considered_associative_operator) 
                        );
                    }
                }
            }
        } else {
            operands.push(self.to_io_repr());
        }
        // ***
        operands
    }

    /** 
     * Conversion from the concrete interaction language to this crate's internal representation.
     * **/
     fn to_io_repr(&self) -> InteractionInternalRepresentation<CioII> {
        match self.identify_pattern_at_interaction_root() {
            Some((pattern,may_remain)) => {
                let pattern_int_repr = InteractionInternalRepresentation::LeafPattern(pattern);
                if let Some((remainder_op,remainder)) = may_remain {
                    InteractionInternalRepresentation::Operator(
                        remainder_op, 
                        vec![
                            pattern_int_repr,
                            remainder.to_io_repr()
                        ]
                    )
                } else {
                    pattern_int_repr
                }
            },
            None => {
                // patterns must cover all non-operator symbols (more precisely all operators of arity 0)
                // so here we must be able to identify the root operator
                let op_at_root = self.get_operator_at_root().unwrap();
                let operands = if op_at_root.is_associative() {
                    self.get_associative_operands_recursively(&op_at_root)
                } else {
                    self.get_subinteractions().iter().map(|x| x.to_io_repr()).collect()
                };
                InteractionInternalRepresentation::Operator(op_at_root, operands)
            }
        }
     }

}















