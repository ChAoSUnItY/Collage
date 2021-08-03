pub mod lexer;
pub mod parser;
pub mod emitter;
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};
    use crate::{emitter, lexer, parser};

    #[test]
    fn arrow() {
        let mut lexer = lexer::Lexer::new("pow :: i32 -> i32 -> ()");
        let tokens = lexer.lex();

        println!("{:?}", &tokens);

        let mut parser = parser::Parser::new(tokens);
        let ctx = parser.parse();

        println!("{:?}", &ctx);

        let mut emitter = emitter::Emitter::new(ctx);
        let bytecode = emitter.emit();

        println!("{:#04X?}", &bytecode);

        let mut output = File::create("examples/out/collage.wasm").expect("Unknown fs error.");
        output.write_all(&bytecode);
    }
}
