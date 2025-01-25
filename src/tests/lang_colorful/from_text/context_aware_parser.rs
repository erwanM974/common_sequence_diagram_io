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

use image::Rgb;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value, opt};
use nom::error::ParseError;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};

use image_colored_text::text::paragraph::*;
use image_colored_text::text::line::*;

use crate::from_text::context_aware_parser::ContextAwareInteractionParser;
use crate::tests::lang_colorful::colorful_lang::*;
use crate::tests::lang_colorful::core::internal_representation::*;
use crate::core::internal_representation::*;



use crate::from_text::util::delimited_lang_parser::DelimitedInteractionLanguageParser;
use crate::from_text::util::generic_broadcast_parser::GenericBroadcastParser;
use crate::from_text::util::parse_utils::*;
use crate::tests::lang_colorful::to_image::colorful_colors::*;

impl DelimitedInteractionLanguageParser for ColorfulContext {

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


impl GenericBroadcastParser<ColorfulAction,usize,ColorfulAction,ColorfulLeafPattern> for ColorfulContext {
    fn make_pattern(&self,origin : Option<ColorfulAction>, message : usize, targets : Vec<ColorfulAction>) -> ColorfulLeafPattern {
        let mut targets = targets;
        ColorfulLeafPattern::new(origin.unwrap(), message, targets.remove(0))
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

    fn parse_broadcast_origin<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, ColorfulAction,E> {
        map(
            tuple(
                (
                    opt(|x| parse_note(x)),
                    |x| self.parse_gate_or_lifeline(x),
                    opt(|x| parse_note(x)),
                )
            ),
            |(n1,gol,n2)| ColorfulAction::new(gol,(n1,n2))
        )
        (input)
    }

    fn parse_single_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, ColorfulAction,E> {
        self.parse_broadcast_origin(input)
    }
}

impl ColorfulContext {

    fn parse_gate_or_lifeline<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str,GateOrLifeline,E> {
        alt(
            (
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(&self.lf_names,x),
                    |l| GateOrLifeline::Lifeline(l)
                ),
                map(
                    |x| parse_element_of_preexisting_vec_and_return_index(&self.gt_names,x),
                    |g| GateOrLifeline::Gate(g)
                )
            )
        )(input)
    }

}

fn parse_note<'a, E: ParseError<&'a str>>(input : &'a str) -> IResult<&'a str, ColoredTextParagraph,E> {
    let mut parser = delimited(
        nom::character::complete::char('{'),
        separated_list1(
            nom::character::complete::char(';'),
            |x| parse_label_with_underscores(x)
        ),
        nom::character::complete::char('}')
    );
    parser(input).map(|(rem, x)| {
        let mut lines = vec![];
        for line in x {
            lines.push(ColoredTextLine::new(vec![(line,Rgb(COLORFUL_NOTE_COLOR))]));
        }
        let para = ColoredTextParagraph::new(
            lines,
            MultiLineTextAlignment::Center,
            Some(Rgb(COLORFUL_NOTE_BACKGROUND)),
            Some(Rgb(COLORFUL_NOTE_COLOR))
        );
        (rem, para)
    })
}



impl ContextAwareInteractionParser<ColorfulLangCioII> for ColorfulContext {

    fn parse_operator<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        <ColorfulLangCioII as CommonIoInteractionInterface>::InteractionOperatorType, 
        E> {
        alt(
            (
            value(ColorfulOperators::TPC, tag("TPC")),
            value(ColorfulOperators::Brocoli, tag("brocoli")),
            value(ColorfulOperators::Coreg(None), tag("seq")),
            map(
                tuple(
                    (
                        value((),tag("cr{")),
                        |x| parse_element_of_preexisting_vec_and_return_index(&self.lf_names,x),
                        value((),tag("}")),
                    )
                ),
                |(_,c,_)| ColorfulOperators::Coreg(Some(c))
            ),
            map(
                tuple(
                    (
                        value((),tag("rougail{")),
                        |x| parse_label_with_underscores(x),
                        value((),tag("}")),
                    )
                ),
                |(_,x,_)| ColorfulOperators::Rougail(x)
            )
            )
        )
        (input)
    }

    fn parse_explicit_pattern<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<
        &'a str,
        <ColorfulLangCioII as CommonIoInteractionInterface>::InteractionLeafPatternType,
        E> {
        self.parse_broadcast_pattern(input)
    }



}








