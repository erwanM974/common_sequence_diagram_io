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




use nom::error::VerboseError;

use crate::from_text::context_aware_parser::ContextAwareInteractionParser;
use crate::internal_representation::{CommonIoInteractionInterface, InteractionInternalRepresentation};



pub fn parse_interaction<CioII,Parser>
    (
        input_str : &str,
        parser : &Parser
    ) -> 
        Result<
            InteractionInternalRepresentation<CioII>, 
            String
        > 
where 
    CioII : CommonIoInteractionInterface,
    Parser : ContextAwareInteractionParser<CioII>
{
    //let input_str_no_white_space : String = input_str.chars().filter(|c| !c.is_whitespace()).collect();
    match parser.parse_interaction_inner::<VerboseError<&str>>(input_str) {
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            Err(nom::error::convert_error(input_str, e))
        },
        Ok( (_,int_repr)) => {
            Ok(int_repr)
        },
        _ => {
            panic!()
        }
    }
}

