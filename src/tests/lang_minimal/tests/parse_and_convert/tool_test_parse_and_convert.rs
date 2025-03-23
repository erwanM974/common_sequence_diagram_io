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
use crate::tests::lang_minimal::minimal_lang::{GeneralContext, MinimalInteraction};
use crate::from_text::parse::parse_interaction;
use crate::to_text::print::print_interaction;
use crate::conversion::lang_to_repr::FromInteractionTermToInternalRepresentation;
use crate::conversion::repr_to_lang::FromInternalRepresentationToInteractionTerm;



pub struct TestRetranslationParameterization {
    pub merge_patterns : bool,
    pub flatten_operands_under_associative_operators : bool,
    pub expected_retranslated_internal_repr : String,
    pub expected_reprinted : String,
}

impl TestRetranslationParameterization {
    pub fn new(merge_patterns: bool, flatten_operands_under_associative_operators: bool, expected_retranslated_internal_repr: String, expected_reprinted: String) -> Self {
        Self { merge_patterns, flatten_operands_under_associative_operators, expected_retranslated_internal_repr, expected_reprinted }
    }
}


/**
 * Parses the input text (that contains a textual description of the interaction) with knowledge of the context.
 * Then produces the internal representation of that interaction.
 * It then compares it to the expected result.
 * If ok, it converts it into a concrete interaction term.
 * Then compares it to the expected value.
 * Then converts it back to the internal representation.
 * And performs the final verification.
 * **/
pub fn tool_test_verify_parsing_and_two_way_conversions(
        ctx : &GeneralContext, 
        input_text : &str, 
        expected_parsed_internal_repr : &str, 
        expected_term : &str,
        retranslate : Option<TestRetranslationParameterization>,
    ) {
    // we parse the input text and verifify that the obtained internal representation is indeed the expected one
    let internal_repr = parse_interaction::<MinimalLangCioII,GeneralContext>(
        input_text,&ctx
    ).unwrap();
    let got_internal_repr : String = format!("{:?}",internal_repr).chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(expected_parsed_internal_repr, got_internal_repr);

    // we translate the internal representation to a term in the concrete interaction language and verify that it is indeed the expected one
    let term = MinimalInteraction::from_io_repr(&internal_repr);
    let got_term : String = format!("{:?}",term).chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(expected_term, got_term);

    // finally we translate it back to an internal representation and verify that the result is correct
    match retranslate {
        Some(specific_retranslation_param) => {
            let retranslated_internal_repr = term.to_io_repr(
                specific_retranslation_param.merge_patterns,
                specific_retranslation_param.flatten_operands_under_associative_operators
            );
            let got_retranslated_internal_repr : String = format!("{:?}",retranslated_internal_repr).chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(specific_retranslation_param.expected_retranslated_internal_repr, got_retranslated_internal_repr);
            // we also print the internal representation in text form to see if we have the same as the input text
            let reprinted = print_interaction(&retranslated_internal_repr, ctx);
            let reprinted_no_whitespace : String = reprinted.chars().filter(|c| !c.is_whitespace()).collect();
            let expected_reprinted_no_whitespace : String = specific_retranslation_param.expected_reprinted.chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(reprinted_no_whitespace,expected_reprinted_no_whitespace);
        },
        None => {
            let retranslated_internal_repr = term.to_io_repr(
                true,
                true
            );
            let got_retranslated_internal_repr : String = format!("{:?}",retranslated_internal_repr).chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(expected_parsed_internal_repr, got_retranslated_internal_repr);
            // we also print the internal representation in text form to see if we have the same as the input text
            let reprinted = print_interaction(&retranslated_internal_repr, ctx);
            let reprinted_no_whitespace : String = reprinted.chars().filter(|c| !c.is_whitespace()).collect();
            let input_no_whitespace : String = input_text.chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(reprinted_no_whitespace,input_no_whitespace);
        },
    }

}



