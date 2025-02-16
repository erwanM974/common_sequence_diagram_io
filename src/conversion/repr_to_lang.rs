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


use crate::internal_representation::{InteractionInternalRepresentation,InteractionOperatorRepresentation,CommonIoInteractionInterface};



pub trait FromInternalRepresentationToInteractionTerm<CioII : CommonIoInteractionInterface> : Sized + Clone {

    /**
     * Returns the empty interaction of the given interaction language.
     * We suppose that it exists.
     * **/
    fn get_empty_interaction() -> Self;

     /**
      * A tool function to help construct interaction terms.
      * For instance, given operator "f" of arity 2, this function applied to "f,vec![i1,i2]" should return "Some(f(i1,i2))"
      * Should return None when the arity of the operator does not match the size of the input Vec.
      * **/
     fn instantiate_interaction_under_operator(operator : &CioII::InteractionOperatorType, sub_ints : &mut Vec<Self>) -> Option<Self>;

    /**
     * Obtain an interaction term from a description of a pattern of communication.
     * **/
    fn transform_pattern_to_term(pattern : &CioII::InteractionLeafPatternType) -> Self;

    /** 
     * A tool function to recursively fold a number of interactions under a binary associative operator.
     * For instance, if applied to "(f,vec![a,b,c,d])" where "f" is a binary associative operator, it will return "f(a,f(b,f(c,d)))".
     * **/
    fn fold_associative_operands_recursively(operator : &CioII::InteractionOperatorType, operands : &mut Vec<Self>) -> Self {
        //assert!(operator.is_associative());
        let ops_num = operands.len();
        if ops_num == 2 {
            let i2 = operands.pop().unwrap();
            let i1 = operands.pop().unwrap();
            Self::instantiate_interaction_under_operator(operator,&mut vec![i1,i2]).unwrap()
        } else if ops_num == 1 {
            operands.pop().unwrap().clone()
        } else if ops_num == 0 {
            Self::get_empty_interaction()
        } else {
            let i1 = operands.remove(0);
            let i2 = Self::fold_associative_operands_recursively(operator,operands);
            Self::instantiate_interaction_under_operator(operator,&mut vec![i1,i2]).unwrap()
        }
    }

     /** 
      * Conversion from this crate's internal representation to the concrete interaction language.
      * **/
     fn from_io_repr(io_int_repr : &InteractionInternalRepresentation<CioII>) -> Self {
        match io_int_repr {
            InteractionInternalRepresentation::LeafPattern(leaf_pattern) => {
                Self::transform_pattern_to_term(leaf_pattern)
            },
            InteractionInternalRepresentation::Operator(operator, sub_ints_reprs) => {
                let mut sub_ints : Vec<Self> = sub_ints_reprs.iter().map(Self::from_io_repr).collect();
                if operator.is_associative() {
                    Self::fold_associative_operands_recursively(operator, &mut sub_ints)
                } else {
                    Self::instantiate_interaction_under_operator(operator,&mut sub_ints).unwrap()
                }
            }

        }
     }




}








