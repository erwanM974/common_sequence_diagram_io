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



use nom::bytes::complete::take_while;
use nom::character::complete::alpha1;
use nom::Err::Error;
use nom::IResult;
use nom::sequence::{delimited, tuple};
use nom::character::complete::multispace0;
use nom::error::{ErrorKind, ParseError};







pub fn parse_label_with_underscores<'a,E: ParseError<&'a str>>(input : &'a str) -> IResult<&'a str, String,E> {
    let mut parser = delimited(
        multispace0,
        tuple(
            (
                alpha1,
                take_while(|c: char| c == '_' || c.is_alphanumeric())
            ),
        ),
        multispace0
    );
    parser(input).map(|(rem, (x, y))| {
        (rem, format!("{}{}", x, y))
    })
}




pub fn parse_element_of_preexisting_vec_and_return_index<'a, E: ParseError<&'a str>>(reference_vec : &[String], input : &'a str) -> IResult<&'a str, usize,E> {
    match parse_label_with_underscores(input) {
        Err(e) => {
            Err(e)
        },
        Ok((rem,lab)) => {
            match reference_vec.iter().position(|s| s.eq(&lab)) {
                None => {
                    Err(Error(nom::error::make_error(input, ErrorKind::Fail)))
                }
                Some(index) => {
                    Ok((rem,index))
                }
            }
        }
    }
} 







