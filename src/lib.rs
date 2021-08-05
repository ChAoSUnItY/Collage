use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod compilation;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod utils;

mod tests;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(raw_module = "./collage.js")]
pub fn evaluate(source: &str) -> js_sys::JsString {
    let compilation = compilation::Compilation::new(source.to_string());
    let result = compilation.eval();

    if let Some(val) = result.downcast_ref::<i64>() {
        val.to_string().into()
    } else if let Some(val) = result.downcast_ref::<bool>() {
        val.to_string().into()
    } else {
        "<ERROR>".into()
    }
}

#[wasm_bindgen(raw_module = "./collage.js")]
pub fn get_syntax_tree(source: &str) -> js_sys::JsString {
    let compilation = compilation::Compilation::new(source.to_string());
    let tree = compilation.tree();

    utils::get_syntax_tree(
        &mut "".to_string(),
        &Box::new(tree.root_expression),
        "".to_string(),
        true,
    )
    .into()
}
