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



use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::error::ParseError;
use nom::IResult;


use crate::from_text::context_aware_parser::ContextAwareInteractionParser;
use crate::tests::lang_minimal::minimal_lang::*;
use crate::tests::lang_minimal::core::internal_representation::*;
use crate::internal_representation::*;



use crate::from_text::util::delimited_lang_parser::DelimitedInteractionLanguageParser;
use crate::from_text::util::generic_broadcast_parser::GenericBroadcastParser;
use crate::from_text::util::parse_utils::parse_element_of_preexisting_vec_and_return_index;


impl DelimitedInteractionLanguageParser for GeneralContext {

    fn left_parenthesis_char(&self) -> char {
        '('
    }

    fn right_parenthesis_char(&self) -> char {
        ')'
    }

    fn separator_char(&self) -> char {
        ','
    }
}


impl GenericBroadcastParser<usize,usize,usize,MinimalBroadcastLeafPattern> for GeneralContext {
    fn make_pattern(&self,origin : Option<usize>, message : usize, targets : Vec<usize>) -> MinimalBroadcastLeafPattern {
        MinimalBroadcastLeafPattern::new(origin, message, targets)
    }

    fn get_empty_target_char(&self) -> char {
        '|'
    }

    fn get_tag_for_message_reception_by_target(&self) -> &'static str {
        "->"
    }

    fn get_tag_for_message_transmission_from_origin(&self) -> &'static str {
        "--"
    }

    fn parse_message<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, usize,E> {
        parse_element_of_preexisting_vec_and_return_index(&self.ms_names,input)
    }

    fn parse_broadcast_origin<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, usize,E> {
        parse_element_of_preexisting_vec_and_return_index(&self.lf_names,input)
    }

    fn parse_single_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, usize,E> {
        parse_element_of_preexisting_vec_and_return_index(&self.lf_names,input)
    }
}



impl ContextAwareInteractionParser<MinimalLangCioII> for GeneralContext {

    fn parse_operator<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        <MinimalLangCioII as CommonIoInteractionInterface>::InteractionOperatorType, 
        E> {
        alt(
            (
            value(MinimalOperators::Strict, tag("strict")),
            value(MinimalOperators::Seq, tag("seq")),
            value(MinimalOperators::Alt, tag("alt")),
            value(MinimalOperators::Par, tag("par")),
            value(MinimalOperators::Loop, tag("loop"))
            )
        )
        (input)
    }

    fn parse_explicit_pattern<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        <MinimalLangCioII as CommonIoInteractionInterface>::InteractionLeafPatternType,
        E> {
        // we have two kinds of patterns :
        // *m -> l* for the reception of *m* by *l*
        // *l -- m -> X* for the emission of *m* by *l* to *X*, with *X* itself being either of three patterns:
        //     *|* for the empty target
        //     *l2* for another lifeline
        //     *(l2,l3)* for two or more lifelines
        alt(
            (
                map(|x| self.parse_broadcast_pattern(x), |y| MinimalLeafPattern::BROADCAST(y)),
                value(MinimalLeafPattern::EMPTY,alt((tag("0"),tag("o"))))
            )
        )
        (input)
    }



}








