use parser::CollageStatement;

extern crate pest;
extern crate wasm_bindgen;
#[macro_use]
extern crate pest_derive;
extern crate serde;
extern crate serde_json;

pub mod parser;
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn parse(file_location: &str) -> Vec<CollageStatement> {
    let file_source = std::fs::read_to_string(file_location).expect("Unexpected fs error.");

    parser::parse(&file_source).expect("Unexpected parsing error.")
}

pub fn parse_source(file_source: &str) -> Vec<CollageStatement> {
    let result = parser::parse(file_source).expect("Unexpected parsing error.");

    result
}

#[wasm_bindgen(raw_module = "./collage.js")]
pub fn parse_as_json(file_source: &str) -> JsValue {
    let result = parse_source(file_source);
    let array = js_sys::Array::new();

    for statement in result {
        unsafe {
            array.push(&statement.as_obj());
        }
    }

    array.into()
}
