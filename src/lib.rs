pub mod lexer;
pub mod parser;
mod utils;
mod diagnostic;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod test {
    use crate::{diagnostic::DiagnosticHolder, lexer, parser};
    use test_case::test_case;
    use crate::parser::SyntaxNode;

    #[test_case("1 + 2 + 3" ; "plus expression")]
    fn expression_parsing_test(source_code: &'static str) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = lexer::Lexer::new(source_code);
        let tokens = lexer.lex(&diagnostic_holder);

        let mut parser = parser::Parser::new(tokens);
        let ctx = parser.parse(&mut diagnostic_holder);

        assert_eq!(ctx.len(), 1);
        assert!(ctx[0].is_some());

        println!("{:?}", ctx[0].as_ref().unwrap().children());
    }
}
