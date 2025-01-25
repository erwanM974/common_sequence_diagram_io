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
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::error::ParseError;

use crate::core::internal_representation::{InteractionInternalRepresentation,CommonIoInteractionInterface};
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
        parser(input).map(|(rem, (operator, _, _, _, operands,_,_))| {
            (rem, InteractionInternalRepresentation::Operator(operator,operands))
        })
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

        /*

        
        if let Ok((r,op_kind)) = self.parse_operator(input_str) {
            let first_char_after_operator = r.chars().next().unwrap();
            if first_char_after_operator.eq(&self.left_parenthesis_char()) {
                let mut remainder = "";
                let mut current_r = &r[1..];
                let mut operands = vec![];
                'iter_operands : loop {
                    if let Ok((r2,inner_int)) = self.parse_interaction_inner::<E>(current_r) {
                        operands.push(inner_int);
                        let first_char_after_operand = r2.chars().next().unwrap();
                        if first_char_after_operand.eq(&self.right_parenthesis_char()) {
                            remainder = &r2[1..];
                            break 'iter_operands;
                        } else if first_char_after_operand.eq(&self.separator_char()) {
                            current_r = &r2[1..];
                        }
                    } else {
                        break 'iter_operands;
                    }
                }
                // ***
                let number_of_operands = operands.len();
                let is_arity_ok = if op_kind.is_associative() {
                    // if associative, any number >=2 is ok
                    number_of_operands >= 2
                } else {
                    number_of_operands == op_kind.arity()
                };
                if is_arity_ok {
                    let i = InteractionInternalRepresentation::Operator(
                        op_kind,
                        operands
                    );
                    return Ok((remainder,i));
                }
            } 
        }


        // otherwise could not parse the interaction, hence returns the sequence of encountered failures
        return Err(Error(nom::error::make_error(input_str, ErrorKind::Fail)));*/
    }

}

