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

    fn get_subinteractions<'a>(&'a self) -> Vec<&'a Self> {
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



    fn identify_pattern_at_interaction_root(&self) -> Option<MinimalLeafPattern> {
        match self {
            MinimalInteraction::Empty => {
                return Some(MinimalLeafPattern::EMPTY);
            },
            MinimalInteraction::Action(act) => {
                return Some(act.to_pattern());
            },
            MinimalInteraction::Strict(i1, i2) => {
                if let MinimalInteraction::Action(ref emission) = **i1 {
                    if emission.kind == MinimalActionKind::Emission {
                        // if on the left of the strict we have an emission "act1 = l1!m1"
                        if let Some(MinimalLeafPattern::BROADCAST(b2)) = i2.identify_pattern_at_interaction_root() {
                            if b2.origin_lf_id.is_none() && b2.msg_id == emission.ms_id {
                                // and on the right of the strict we have identified a pattern of the form "seq(l2?m1,...)" i.e.
                                // a broadcast pattern with no known origin and the same message "m1"
                                // then we return the broadcast, having found the origin as "l1" i.e. "act1.lf_id"

                                let broadcast = MinimalBroadcastLeafPattern::new(
                                    Some(emission.lf_id), 
                                    emission.ms_id,
                                    b2.targets
                                );
                                return Some(MinimalLeafPattern::BROADCAST(broadcast));
                            }
                        }
                    } 
                }
                return None;
            },
            MinimalInteraction::Seq(i1, i2) => {
                if let (
                    Some(MinimalLeafPattern::BROADCAST(b1)),
                    Some(MinimalLeafPattern::BROADCAST(b2))
                ) = (i1.identify_pattern_at_interaction_root(),i2.identify_pattern_at_interaction_root()) {
                    let same_message = b1.msg_id == b2.msg_id;
                    let no_origin_b1 = b1.origin_lf_id.is_none();
                    let no_origin_b2 = b2.origin_lf_id.is_none();
                    if same_message && no_origin_b1 && no_origin_b2 {
                        let mut targets = b1.targets;
                        targets.extend(b2.targets);
                        return Some(
                            MinimalLeafPattern::BROADCAST(
                                MinimalBroadcastLeafPattern::new(
                                    b1.origin_lf_id, 
                                    b1.msg_id, 
                                    targets
                                )
                            )
                        );
                    }
                }
                return None;
            },
            _ => {
                return None;
            }
        };
    }

}







