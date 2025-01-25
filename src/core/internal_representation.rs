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



use std::fmt::Debug;


pub trait InteractionOperatorRepresentation {

    fn arity(&self) -> usize;

    /**
    Whether or not a given binary operator *op* is associative.
    It dictates whether or not, in the internal representation, we can represent for instance:
    *op(i1,op(i2,i3))* as *op(i1,i2,i3)*
     **/
    fn is_associative(&self) -> bool;

}


pub trait CommonIoInteractionInterface : Sized {

    /**
     The type of patterns (e.g., message passing, broadcasts etc.) at the leaves
    of the interaction internal representation.
     **/
    type InteractionLeafPatternType : Debug + Clone;

    /**
     The type of the operators of the specific Interaction Language we are considering.
     **/
    type InteractionOperatorType : Debug + InteractionOperatorRepresentation + Clone;

}




/**
 This is how interaction terms are encoded in this crate.
This does not necessarily correspond to the concrete interaction language implementation.
Rather, it is an internal representation that is proper to this present IO crate and facilitates IO operations.
 **/
 #[derive(Debug)]
pub enum InteractionInternalRepresentation<CioII : CommonIoInteractionInterface> {
    LeafPattern(CioII::InteractionLeafPatternType),
    Operator(CioII::InteractionOperatorType, Vec<InteractionInternalRepresentation<CioII>>)
}



