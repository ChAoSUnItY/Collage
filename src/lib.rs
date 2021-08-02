pub mod lexer;
pub mod parser;
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod test {
    use crate::{lexer, parser};

    #[test]
    fn arrow() {
        let mut lexer = lexer::Lexer::new("pow :: a -> a");
        let tokens = lexer.lex();

        println!("{:?}", &tokens);

        let mut parser = parser::Parser::new(tokens);
        let ctx = parser.parse();

        
        println!("{:?}", &ctx);
    }
}
