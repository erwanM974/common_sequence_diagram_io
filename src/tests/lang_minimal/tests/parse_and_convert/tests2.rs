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

use crate::tests::lang_minimal::minimal_lang::GeneralContext;

use super::tool_test_parse_and_convert::{tool_test_verify_parsing_and_two_way_conversions, TestRetranslationParameterization};







#[test]
fn test_parse_and_conversions_retranslating_variants() {
    // defines the interaction signature (context) and the interaction textual input
    let ctx = GeneralContext{lf_names:vec!["a".to_string(),"b".to_string()],ms_names:vec!["m".to_string(),"n".to_string()]};
    let input_text = 
r#"seq(
        a -- m -> b,
        alt(
                b -- m -> a,
                0,
                a -- n -> b
        ),
        b -- n -> a
)"#;

    // expected structure of the parsed internal representation
    let expected_internal_repr : String = r#"
Operator(Seq,[
    LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:0,targets:[1]})),
    Operator(Alt,[
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:0,targets:[0]})),
        LeafPattern(EMPTY),
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:1,targets:[1]}))
    ]),
    LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:1,targets:[0]}))
])
"#.chars().filter(|c| !c.is_whitespace()).collect();

    // expected structure of the interaction term in the concrete interaction language
    let expected_term : String = r#"
Seq(
    Strict(
        Action(MinimalAction{lf_id:0,ms_id:0,kind:Emission}),
        Action(MinimalAction{lf_id:1,ms_id:0,kind:Reception})
    ),
    Seq(
        Alt(
            Strict(
                Action(MinimalAction{lf_id:1,ms_id:0,kind:Emission}),
                Action(MinimalAction{lf_id:0,ms_id:0,kind:Reception})
            ),
            Alt(
                Empty,
                Strict(
                    Action(MinimalAction{lf_id:0,ms_id:1,kind:Emission}),
                    Action(MinimalAction{lf_id:1,ms_id:1,kind:Reception})
                )
            )
        ),
        Strict(
            Action(MinimalAction{lf_id:1,ms_id:1,kind:Emission}),
            Action(MinimalAction{lf_id:0,ms_id:1,kind:Reception})
        )
    )
)
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    tool_test_verify_parsing_and_two_way_conversions(
        &ctx, 
        input_text, 
        &expected_internal_repr,
        &expected_term, 
        None
    );


    let expected_retranslated : String = r#"
    Operator(Seq,[
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:0,targets:[1]})),
        Operator(Seq,[
            Operator(Alt,[
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:0,targets:[0]})),
                Operator(Alt,[
                    LeafPattern(EMPTY),
                    LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:1,targets:[1]}))
                ])
            ]),
            LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:1,targets:[0]}))
        ])
    ])
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_reprinted =
r#"seq(
    a -- m -> b,
    seq(
        alt(
            b -- m -> a,
            alt(
                0,
                a -- n -> b
            )
        ),
        b -- n -> a
    )
)"#.to_string();

    tool_test_verify_parsing_and_two_way_conversions(
        &ctx, 
        input_text, 
        &expected_internal_repr,
        &expected_term, 
        Some(
            TestRetranslationParameterization::new(
                true, 
                false,
                expected_retranslated, 
                expected_reprinted
            )
        )
    );

    

    let expected_retranslated2 : String = r#"
Operator(Seq,[
    Operator(Strict,[
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:0,targets:[]})),
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:None,msg_id:0,targets:[1]}))
    ]),
    Operator(Seq,[
        Operator(Alt,[
            Operator(Strict,[
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:0,targets:[]})),
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:None,msg_id:0,targets:[0]}))
            ]),
            Operator(Alt,[
                LeafPattern(EMPTY),
                Operator(Strict,[
                    LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:1,targets:[]})),
                    LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:None,msg_id:1,targets:[1]}))
                ])
            ])
        ]),
        Operator(Strict,[
            LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(1),msg_id:1,targets:[]})),
            LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:None,msg_id:1,targets:[0]}))
        ])
    ])
])
    "#.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_reprinted2 =
r#"seq(
    strict(
        a -- m ->|,
        m -> b
    ),
    seq(
        alt(
            strict(
                b -- m ->|,
                m -> a
            ),
            alt(
                0,
                strict(
                    a -- n ->|,
                    n -> b
                )
            )
        ),
        strict(
            b -- n ->|,
            n -> a
        )
    )
)"#.to_string();

    tool_test_verify_parsing_and_two_way_conversions(
        &ctx, 
        input_text, 
        &expected_internal_repr,
        &expected_term, 
        Some(
            TestRetranslationParameterization::new(
                false, 
                false,
                expected_retranslated2, 
                expected_reprinted2
            )
        )
    );
}






