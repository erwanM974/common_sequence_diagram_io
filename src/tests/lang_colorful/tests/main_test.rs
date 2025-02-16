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




use std::path::Path;

use image::Rgb;
use image_colored_text::text::line::ColoredTextLine;
use image_colored_text::text::paragraph::{ColoredTextParagraph, MultiLineTextAlignment};

use crate::tests::lang_colorful::to_image::colorful_colors::*;
use crate::tests::lang_colorful::core::internal_representation::*;
use crate::tests::lang_colorful::colorful_lang::*;
use crate::from_text::parse::parse_interaction;
use crate::tests::lang_colorful::to_image::drawing_context::ColorfulDrawingContext;

use crate::to_image::interface::draw_interaction_as_sequence_diagram;



#[test]
fn draw_test() {
    // defines the interaction signature (context) and the interaction textual input
    let ctx = ColorfulContext{
        lf_names:vec!["bob".to_string(),"alice".to_string(),"carl".to_string()],
        lf_descs:vec![
            ColoredTextParagraph::new(
                vec![
                    ColoredTextLine::new(vec![("bob".to_owned(), Rgb(COLORFUL_LIFELINE))]),
                    ColoredTextLine::new(vec![("x".to_owned(), Rgb(COLORFUL_VARIABLE)),("=".to_owned(), Rgb(COLORFUL_ROUGAIL_TOMATE)),("5".to_owned(), Rgb(COLORFUL_BLACK))]),
                    ColoredTextLine::new(vec![("doesn't drink coffee".to_owned(), Rgb(COLORFUL_LIFELINE))]),
                ], 
                MultiLineTextAlignment::Right, 
                None, 
                Some(Rgb(COLORFUL_BLACK))
            ),
            ColoredTextParagraph::new(
                vec![
                    ColoredTextLine::new(vec![("alice".to_owned(), Rgb(COLORFUL_LIFELINE))]),
                    ColoredTextLine::new(vec![("x".to_owned(), Rgb(COLORFUL_VARIABLE)),("=".to_owned(), Rgb(COLORFUL_ROUGAIL_TOMATE)),("525".to_owned(), Rgb(COLORFUL_BLACK))]),
                    ColoredTextLine::new(vec![("y".to_owned(), Rgb(COLORFUL_VARIABLE)),("=".to_owned(), Rgb(COLORFUL_ROUGAIL_TOMATE)),("boubou".to_owned(), Rgb(COLORFUL_BLACK))]),
                ], 
                MultiLineTextAlignment::Left, 
                None, 
                Some(Rgb(COLORFUL_BLACK))
            ),
            ColoredTextParagraph::new(
                vec![
                    ColoredTextLine::new(vec![("Karl Franz I Holswig Schliestein".to_owned(), Rgb(COLORFUL_LIFELINE))]),
                    ColoredTextLine::new(vec![("drinks tea".to_owned(), Rgb(COLORFUL_LIFELINE))])
                ], 
                MultiLineTextAlignment::Center, 
                Some(Rgb(COLORFUL_PAIN)),
                None
            ),
        ],
        ms_names:vec!["discombobulate".to_string(),"befuddle".to_string(),"flummox".to_string(),"secret".to_string()],
        gt_names:vec!["binturong".to_string(),"quokka".to_string(),"kakapo".to_string()],
    };
    let input_text = 
r#"seq(
    {IgetIt}bob--discombobulate->{why;powerlevel}alice{coffepause},
    brocoli(
        cr{carl}(
            alice{gotIt}--befuddle->carl,
            alice{nah}--flummox->carl{ready}
        ),
        binturong--discombobulate->alice
    ),
    carl--secret->bob,
    rougail{dakatine}(
        bob--befuddle->quokka,
        rougail{tomate}(
            seq(
                bob--flummox->alice,
                alice{a;b;c}--flummox->alice
            ),
            TPC(
                alice--secret->kakapo
            ),
            carl--discombobulate->bob
        )
    )
)"#;
    let name = "bob".to_owned();
    let internal_repr = parse_interaction::<ColorfulLangCioII,ColorfulContext>(
        input_text,&ctx
    ).unwrap_or_else(|x| {eprintln!("{}",x);panic!();});

    let drawing_context = ColorfulDrawingContext::new(ctx);
    draw_interaction_as_sequence_diagram::<ColorfulLangCioII,usize,ColorfulDrawingContext,ColorfulDrawingContext>(
        &internal_repr,
        &drawing_context,
        &drawing_context,
        &Path::new(&format!("{}.png",name))
    );
}