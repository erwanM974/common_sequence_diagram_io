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



use crate::conversion::repr_to_lang::FromInternalRepresentationToInteractionTerm;
use crate::tests::lang_minimal::core::internal_representation::*;
use crate::tests::lang_minimal::minimal_lang::*;



impl FromInternalRepresentationToInteractionTerm<MinimalLangCioII> for MinimalInteraction {

    fn instantiate_interaction_under_operator(operator : &MinimalOperators, sub_ints : &mut Vec<Self>) -> Option<Self> {
        if operator == &MinimalOperators::Loop {
            let i1 = sub_ints.pop().unwrap();
            Some(MinimalInteraction::Loop(Box::new(i1)))
        } else {
            let i2 = sub_ints.pop().unwrap();
            let i1 = sub_ints.pop().unwrap();
            match operator {
                MinimalOperators::Strict => {
                    Some(MinimalInteraction::Strict(Box::new(i1), Box::new(i2)))
                },
                MinimalOperators::Seq => {
                    Some(MinimalInteraction::Seq(Box::new(i1), Box::new(i2)))
                },
                MinimalOperators::Par => {
                    Some(MinimalInteraction::Par(Box::new(i1), Box::new(i2)))
                },
                MinimalOperators::Alt => {
                    Some(MinimalInteraction::Alt(Box::new(i1), Box::new(i2)))
                },
                MinimalOperators::Loop => {
                    None
                }
            }
        }
        
    }

    fn get_empty_interaction() -> Self {
        MinimalInteraction::Empty
    }
    
    fn transform_pattern_to_term(pattern : &MinimalLeafPattern) -> MinimalInteraction {
        match pattern {
            MinimalLeafPattern::EMPTY => {
                MinimalInteraction::Empty
            },
            MinimalLeafPattern::BROADCAST(broadcast) => {
                if broadcast.targets.is_empty() {
                    let origin_lf_id = broadcast.origin_lf_id.unwrap();
                    let emission = MinimalAction::new(origin_lf_id, broadcast.msg_id,MinimalActionKind::Emission);
                    MinimalInteraction::Action(emission)
                } else {
                    let mut receptions = broadcast.targets.iter().map(
                        |lf_id| MinimalInteraction::Action(MinimalAction::new(*lf_id, broadcast.msg_id,MinimalActionKind::Reception))
                    ).collect();
                    let recs_int = Self::fold_associative_operands_recursively(&MinimalOperators::Seq, &mut receptions);
                    match broadcast.origin_lf_id {
                        None => {
                            recs_int
                        },
                        Some(origin_lf_id) => {
                            let em_int = MinimalInteraction::Action(
                                MinimalAction::new(origin_lf_id, broadcast.msg_id,MinimalActionKind::Emission)
                            );
                            MinimalInteraction::Strict(Box::new(em_int), Box::new(recs_int))
                        }
                    }
                }
            }
        }

    }
}







