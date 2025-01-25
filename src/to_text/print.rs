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



use crate::core::internal_representation::{CommonIoInteractionInterface, InteractionInternalRepresentation};
use crate::to_text::context_aware_printer::ContextAwareInteractionPrinter;


pub fn print_interaction<CioII,Printer>
    (
        int : &InteractionInternalRepresentation<CioII>,
        printer : &Printer
    ) -> 
        String
where 
    CioII : CommonIoInteractionInterface,
    Printer : ContextAwareInteractionPrinter<CioII>
{
    printer.print_interaction_inner(0, int)
}

