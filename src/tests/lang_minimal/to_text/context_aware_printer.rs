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




use crate::to_text::context_aware_printer::ContextAwareInteractionPrinter;

use crate::tests::lang_minimal::minimal_lang::*;
use crate::tests::lang_minimal::core::internal_representation::*;


impl ContextAwareInteractionPrinter<MinimalLangCioII> for GeneralContext {

    fn left_parenthesis(&self) -> &str {
        "("
    }

    fn right_parenthesis(&self) -> &str {
        ")"
    }
    
    fn operand_separator(&self) -> &str {
        ","
    }

    fn print_operator(&self, operator : &MinimalOperators) -> String {
        operator.as_lowercase_string()
    }

    fn print_explicit_pattern(&self, leaf_pattern : &MinimalLeafPattern) -> String {
        match leaf_pattern {
            MinimalLeafPattern::EMPTY => {
                "0".to_owned()
            },
            MinimalLeafPattern::BROADCAST(brd) => {
               let start = match brd.origin_lf_id {
                None => {
                    "".to_owned()
                },
                Some(lf_id) => {
                    format!("{} -- ", self.lf_names.get(lf_id).unwrap())
                }
               };
                let targs_num = brd.targets.len();
                let end : String = match targs_num {
                    0 => {"|".to_owned()},
                    1 => {
                        let targ_lf_id = brd.targets.get(0).unwrap();
                        self.lf_names.get(*targ_lf_id).unwrap().clone()
                    },
                    _ => {
                        let targs : Vec<String> = brd.targets.iter().map(|targ| self.lf_names.get(*targ).unwrap().clone()).collect();
                        format!("({})", targs.join(","))
                    }
                };
                format!("{}{} -> {}",start, self.ms_names.get(brd.msg_id).unwrap(), end)
            }
        }
    }

}




