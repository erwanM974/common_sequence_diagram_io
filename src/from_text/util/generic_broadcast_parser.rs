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
use nom::character::complete::multispace0;
use nom::combinator::{value,map};
use nom::error::ParseError;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};

use crate::from_text::util::delimited_lang_parser::DelimitedInteractionLanguageParser;


pub trait GenericBroadcastParser<GenericBroadcastOrigin,GenericMessage,GenericBroadcastTarget,GenericBroadcastPattern> : DelimitedInteractionLanguageParser {

    fn make_pattern(&self,origin : Option<GenericBroadcastOrigin>, message : GenericMessage, targets : Vec<GenericBroadcastTarget>) -> GenericBroadcastPattern;

    fn get_empty_target_char(&self) -> char;

    fn get_tag_for_message_reception_by_target(&self) -> &'static str;

    fn get_tag_for_message_transmission_from_origin(&self) -> &'static str;

    fn parse_message<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, GenericMessage,E>;

    fn parse_broadcast_origin<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, GenericBroadcastOrigin,E>;

    fn parse_single_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, GenericBroadcastTarget,E>;

    fn parse_multiple_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, Vec<GenericBroadcastTarget>,E> {
        delimited(
            nom::character::complete::char(self.left_parenthesis_char()),
            separated_list1(
                nom::character::complete::char(self.separator_char()), 
                |x| self.parse_single_broadcast_targets(x)),
                nom::character::complete::char(self.right_parenthesis_char())
            )(input)
    }

    fn parse_broadcast_targets<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, Vec<GenericBroadcastTarget>,E> {
        alt(
            (
                map(nom::character::complete::char(self.get_empty_target_char()), |_| vec![]),
                map(|x| self.parse_single_broadcast_targets(x), |y| vec![y]),
                |x| self.parse_multiple_broadcast_targets(x)
            )
        )(input)
    }

    fn parse_broadcast_pattern<'a, E: ParseError<&'a str>>(
        &self, 
        input : &'a str
    ) -> IResult<&'a str, GenericBroadcastPattern,E> {
        alt(
            (
                |x| self.parse_broadcast_with_origin(x),
                |x| self.parse_broadcast_without_origin(x),
            )
        )(input)
    }

    fn parse_broadcast_without_origin<'a, E: ParseError<&'a str>>(
        &self, 
        input : &'a str
    ) -> IResult<&'a str, GenericBroadcastPattern,E> {
        let mut parser = tuple(
            (
                |x| self.parse_message(x), 
                multispace0,
                value((),tag(self.get_tag_for_message_reception_by_target())), 
                multispace0,
                |x| self.parse_broadcast_targets(x)
            )
        );
        parser(input).map(|(rem, (message,_, _, _, targets))| {
            (
                rem, 
                self.make_pattern(None,message,targets)
            )
        })
    }


    fn parse_broadcast_with_origin<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> IResult<&'a str, GenericBroadcastPattern,E> {
        let mut parser = tuple(
            (
                |x| self.parse_broadcast_origin(x), 
                multispace0,
                value((),tag(self.get_tag_for_message_transmission_from_origin())), 
                multispace0,
                |x| self.parse_message(x), 
                multispace0,
                value((),tag(self.get_tag_for_message_reception_by_target())),
                multispace0,
                |x| self.parse_broadcast_targets(x)
            )
        );
        parser(input).map(|(rem, (origin, _, _,_, message,_, _,_,targets))| {
            (
                rem, 
                self.make_pattern(Some(origin),message,targets)
            )
        })
    }



}



