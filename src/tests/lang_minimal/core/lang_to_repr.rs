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




use crate::conversion::lang_to_repr::FromInteractionTermToInternalRepresentation;
use crate::tests::lang_minimal::minimal_lang::*;
use crate::tests::lang_minimal::core::internal_representation::*;



impl MinimalAction {

    pub fn to_pattern(&self) -> MinimalLeafPattern {
        let (origin_lf_id,targets) = match &self.kind {
            MinimalActionKind::Emission => {
                (Some(self.lf_id),vec![])
            },
            MinimalActionKind::Reception => {
                (None,vec![self.lf_id])
            }
        };
        let broadcast = MinimalBroadcastLeafPattern::new(origin_lf_id, self.ms_id, targets);
        MinimalLeafPattern::BROADCAST(broadcast)
    }

}







impl FromInteractionTermToInternalRepresentation<MinimalLangCioII> for MinimalInteraction {

    fn get_subinteractions(&self) -> Vec<&Self> {
        match self {
            MinimalInteraction::Strict(i1, i2) => {
                vec![&*i1,&*i2]
            },
            MinimalInteraction::Seq(i1, i2) => {
                vec![&*i1,&*i2]
            },
            MinimalInteraction::Alt(i1, i2) => {
                vec![&*i1,&*i2]
            },
            MinimalInteraction::Par(i1, i2) => {
                vec![&*i1,&*i2]
            },
            MinimalInteraction::Loop(i1)=> {
                vec![&*i1]
            }
            MinimalInteraction::Empty => {
                vec![]
            },
            MinimalInteraction::Action(_) => {
                vec![]
            },
        }
    }
    
    fn get_operator_at_root(&self) -> Option<MinimalOperators> {
        match self {
            MinimalInteraction::Strict(_,_) => {
                Some(MinimalOperators::Strict)
            },
            MinimalInteraction::Seq(_,_) => {
                Some(MinimalOperators::Seq)
            },
            MinimalInteraction::Alt(_,_) => {
                Some(MinimalOperators::Alt)
            },
            MinimalInteraction::Par(_,_) => {
                Some(MinimalOperators::Par)
            },
            MinimalInteraction::Loop(_) => {
                Some(MinimalOperators::Loop)
            },
            _ => {
                None 
            }
        }
    }


    fn identify_pattern_at_interaction_leaf(&self) -> Option<MinimalLeafPattern> {
        match self {
            MinimalInteraction::Empty => {
                Some(MinimalLeafPattern::EMPTY)
            },
            MinimalInteraction::Action(act) => {
                Some(act.to_pattern())
            },
            _ => {
                None
            }
        }
    }
    
    fn merge_patterns_under_operator_if_possible(
        parent_op : &MinimalOperators,
        p1 : &MinimalLeafPattern,
        p2 : &MinimalLeafPattern
    ) -> Option<MinimalLeafPattern> {
        match (p1,p2) {
            (MinimalLeafPattern::BROADCAST(b1),MinimalLeafPattern::BROADCAST(b2)) => {
                match parent_op {
                    MinimalOperators::Strict => {
                        // b1 must be an emission and b2 a reception of the same message
                        if b1.msg_id == b2.msg_id && 
                        b1.origin_lf_id.is_some() && 
                        b2.origin_lf_id.is_none() && 
                        b2.targets.iter().all(|b2_tar| !b1.targets.contains(b2_tar)){
                            let mut new_targs =b1.targets.clone();
                            new_targs.extend(b2.targets.iter().cloned());
                            let new_b = MinimalBroadcastLeafPattern::new(
                                b1.origin_lf_id, 
                                b1.msg_id, 
                                new_targs
                            );
                            Some(MinimalLeafPattern::BROADCAST(new_b))
                        } else {
                            None 
                        }
                    },
                    MinimalOperators::Seq => {
                        // b1 and b2 must involve the same message
                        // either both are receptions
                        // or b1 is an emission occurring on the same lifeline than b2
                        if b1.msg_id == b2.msg_id {
                            match (b1.origin_lf_id, b2.origin_lf_id) {
                                (None,None) => {
                                    let mut new_targs =b1.targets.clone();
                                    new_targs.extend(b2.targets.iter().cloned());
                                    let new_b = MinimalBroadcastLeafPattern::new(
                                        b1.origin_lf_id, 
                                        b1.msg_id, 
                                        new_targs
                                    );
                                    Some(MinimalLeafPattern::BROADCAST(new_b))
                                },
                                (Some(orig_lf),None) => {
                                    if b1.targets.is_empty() && b2.targets == vec![orig_lf] {
                                        let new_b = MinimalBroadcastLeafPattern::new(
                                            b1.origin_lf_id, 
                                            b1.msg_id, 
                                            b2.targets.clone()
                                        );
                                        Some(MinimalLeafPattern::BROADCAST(new_b))
                                    } else {
                                        None 
                                    }
                                },
                                (_,_) => {
                                    None 
                                }
                            }
                        } else {    
                            None 
                        }
                    },
                    _ => {
                        None 
                    }
                }
            },
            (_,_) => {
                None
            }
        }
    }

}







