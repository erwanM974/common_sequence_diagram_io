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



use crate::tests::lang_minimal::core::internal_representation::*;
use crate::tests::lang_minimal::minimal_lang::GeneralContext;
use crate::from_text::parse::parse_interaction;



/**
 * Parses the input text (that contains a textual description of the interaction) with knowledge of the context.
 * Here the parsing is expected to fail.
 * **/
pub fn tool_test_parse_fail(
        ctx : GeneralContext, 
        input_text : &str
    ) {
    let parse_result = parse_interaction::<MinimalLangCioII,GeneralContext>(
        input_text,&ctx
    );
    assert!(parse_result.is_err())
}


