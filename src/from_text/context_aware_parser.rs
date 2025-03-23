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

use core::num;

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{fail, map};
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::error::{make_error, ParseError};

use crate::internal_representation::{CommonIoInteractionInterface, InteractionInternalRepresentation, InteractionOperatorRepresentation};
use crate::from_text::util::delimited_lang_parser::DelimitedInteractionLanguageParser;






pub trait ContextAwareInteractionParser<CioII : CommonIoInteractionInterface> : DelimitedInteractionLanguageParser {

    fn parse_operator<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> 
            IResult<
                &'a str,
                CioII::InteractionOperatorType,
                E
            >;

    fn parse_explicit_pattern<'a, E: ParseError<&'a str>>(&self, input : &'a str) -> 
            IResult<
                &'a str,
                CioII::InteractionLeafPatternType,
                E
            >;

    fn parse_interaction_with_operator_at_root<'a, E: ParseError<&'a str>>(&self,input : &'a str) -> IResult<&'a str,InteractionInternalRepresentation<CioII>,E> 
    {
        let mut parser = tuple(
            (
                |x| self.parse_operator(x),
                multispace0,
                nom::character::complete::char(self.left_parenthesis_char()),
                multispace0,
                separated_list0(
                    delimited(
                        multispace0,
                        nom::character::complete::char(self.separator_char()), 
                        multispace0
                    ),
                    |x| self.parse_interaction_inner(x)
                ),
                multispace0,
                nom::character::complete::char(self.right_parenthesis_char())
            )
        );
        match parser(input) {
            IResult::Ok((rem, (operator, _, _, _, operands,_,_))) => {
                let arity = operator.arity();
                let num_operands = operands.len();
                if (operator.is_associative() && num_operands >= 2) || (num_operands==arity) {
                    IResult::Ok((rem, InteractionInternalRepresentation::Operator(operator,operands)))
                } else {
                    IResult::Err(nom::Err::Error(make_error::<&'a str, E>(input, nom::error::ErrorKind::Count)))
                }
            },
            IResult::Err(e) => {
                IResult::Err(e)
            }
        }
    }

    fn parse_interaction_inner<'a, E: ParseError<&'a str>>(&self,
        input : &'a str) -> 
            IResult<
                &'a str,
                InteractionInternalRepresentation<CioII>,
                E
            > {

        delimited(
            multispace0,
            alt(
                (
                    // tries to match the (beginning of the) input string to
                    // a leaf pattern of the Interaction Language
                    map(|x| self.parse_explicit_pattern(x),|y| InteractionInternalRepresentation::LeafPattern(y)),
                    // tries to match the (beginning of the) input string to
                    // an interaction with an operator at its root
                    |x| self.parse_interaction_with_operator_at_root(x)
                )
            ),
            multispace0
        )(input)

    }

}

