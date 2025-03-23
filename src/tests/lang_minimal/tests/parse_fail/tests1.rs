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

use super::tool_test_parse_fail::tool_test_parse_fail;







#[test]
fn test_fail_unexpected_char() {
    let ctx = GeneralContext{lf_names:vec!["a".to_string(),"b".to_string()],ms_names:vec!["m".to_string(),"n".to_string()]};
    let input_text = 
r#"seq(;
        a -- m -> b,
        alt(
                b -- m -> a,
                0
        )
)"#;
    tool_test_parse_fail(ctx, input_text);
}







#[test]
fn test_fail_arity_loop() {
    let ctx = GeneralContext{lf_names:vec!["a".to_string(),"b".to_string(),"c".to_string()],ms_names:vec!["m".to_string(),"n".to_string(),"p".to_string()]};
    let input_text = 
r#"seq(
        a -- m -> b,
        a -- n -> c,
        loop(
            a -- p -> (b,c),
            b -- m -> c
        )
)"#;
    tool_test_parse_fail(ctx, input_text);
}



