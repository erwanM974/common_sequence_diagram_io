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

use super::tool_test_parse_and_convert::tool_test_verify_parsing_and_two_way_conversions;







#[test]
fn test_parse_and_conversions_1() {
    // defines the interaction signature (context) and the interaction textual input
    let ctx = GeneralContext{lf_names:vec!["a".to_string(),"b".to_string()],ms_names:vec!["m".to_string(),"n".to_string()]};
    let input_text = 
r#"seq(
        a -- m -> b,
        alt(
                b -- m -> a,
                0
        )
)"#;

    // expected structure of the parsed internal representation
    let expected_internal_repr : String = r#"
Operator(Seq, 
    [
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern { origin_lf_id: Some(0), msg_id: 0, targets: [1] })), 
        Operator(Alt, 
            [
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern { origin_lf_id: Some(1), msg_id: 0, targets: [0] })), 
                LeafPattern(EMPTY)
            ]
        )
    ]
)
"#.chars().filter(|c| !c.is_whitespace()).collect();

    // expected structure of the interaction term in the concrete interaction language
    let expected_term : String = r#"
Seq(
    Strict(
        Action(MinimalAction { lf_id: 0, ms_id: 0, kind: Emission }), 
        Action(MinimalAction { lf_id: 1, ms_id: 0, kind: Reception })
    ), 
    Alt(
        Strict(
            Action(MinimalAction { lf_id: 1, ms_id: 0, kind: Emission }), 
            Action(MinimalAction { lf_id: 0, ms_id: 0, kind: Reception })
        ), 
        Empty
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
}







#[test]
fn test_parse_and_conversions_2() {
    // defines the interaction signature (context) and the interaction textual input
    let ctx = GeneralContext{lf_names:vec!["a".to_string(),"b".to_string(),"c".to_string()],ms_names:vec!["m".to_string(),"n".to_string(),"p".to_string()]};
    let input_text = 
r#"seq(
        a -- m -> b,
        a -- n -> c,
        loop(
                a -- p -> (b,c)
        )
)"#;

    // expected structure of the parsed internal representation
    let expected_internal_repr : String = r#"
Operator(Seq,
    [
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:0,targets:[1]})),
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:1,targets:[2]})),
        Operator(Loop,
            [
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:2,targets:[1,2]}))
            ]
        )
    ]
)
"#.chars().filter(|c| !c.is_whitespace()).collect();

    // expected structure of the interaction term in the concrete interaction language
    let expected_term : String = r#"
Seq(
    Strict(
        Action(MinimalAction{lf_id:0,ms_id:0,kind:Emission}),
        Action(MinimalAction{lf_id:1,ms_id:0,kind:Reception})
    ),
    Seq(
        Strict(
            Action(MinimalAction{lf_id:0,ms_id:1,kind:Emission}),
            Action(MinimalAction{lf_id:2,ms_id:1,kind:Reception})
        ),
        Loop(
            Strict(
                Action(MinimalAction{lf_id:0,ms_id:2,kind:Emission}),
                Seq(
                    Action(MinimalAction{lf_id:1,ms_id:2,kind:Reception}),
                    Action(MinimalAction{lf_id:2,ms_id:2,kind:Reception})
                )
            )
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
}





#[test]
fn test_parse_and_conversions_3() {
    // defines the interaction signature (context) and the interaction textual input
    let ctx = GeneralContext{
        lf_names:vec!["a".to_string(),"b".to_string(),"c".to_string()],
        ms_names:vec!["m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string()]
    };
    let input_text = 
r#"seq(
        a -- m -> b,
        par(
            a -- n -> c,
            alt(
                c -- o -> c,
                c -- p -> b     
            )
        ),
        loop(
            a -- q -> (a,b,c)
        )
)"#;

    // expected structure of the parsed internal representation
    let expected_internal_repr : String = r#"
Operator(Seq,
    [
        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:0,targets:[1]})),
        Operator(Par,
            [
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:1,targets:[2]})),
                Operator(Alt,
                    [
                        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(2),msg_id:2,targets:[2]})),
                        LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(2),msg_id:3,targets:[1]}))
                    ]
                )
            ]
        ),
        Operator(Loop,
            [
                LeafPattern(BROADCAST(MinimalBroadcastLeafPattern{origin_lf_id:Some(0),msg_id:4,targets:[0,1,2]}))
            ]
        )
    ]
)
"#.chars().filter(|c| !c.is_whitespace()).collect();

    // expected structure of the interaction term in the concrete interaction language
    let expected_term : String = r#"
Seq(
    Strict(
        Action(MinimalAction{lf_id:0,ms_id:0,kind:Emission}),
        Action(MinimalAction{lf_id:1,ms_id:0,kind:Reception})
    ),
    Seq(
        Par(
            Strict(
                Action(MinimalAction{lf_id:0,ms_id:1,kind:Emission}),
                Action(MinimalAction{lf_id:2,ms_id:1,kind:Reception})
            ),
            Alt(
                Strict(
                    Action(MinimalAction{lf_id:2,ms_id:2,kind:Emission}),
                    Action(MinimalAction{lf_id:2,ms_id:2,kind:Reception})
                ),
                Strict(
                    Action(MinimalAction{lf_id:2,ms_id:3,kind:Emission}),
                    Action(MinimalAction{lf_id:1,ms_id:3,kind:Reception})
                )
            )
        ),
        Loop(
            Strict(
                Action(MinimalAction{lf_id:0,ms_id:4,kind:Emission}),
                Seq(
                    Action(MinimalAction{lf_id:0,ms_id:4,kind:Reception}),
                    Seq(
                        Action(MinimalAction{lf_id:1,ms_id:4,kind:Reception}),
                        Action(MinimalAction{lf_id:2,ms_id:4,kind:Reception})
                    )
                )
            )
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
}



