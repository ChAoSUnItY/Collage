pub mod lexer;
pub mod parser;
mod utils;
mod diagnostic;
mod runtime;

#[cfg(test)]
mod test {
    use crate::{diagnostic::DiagnosticHolder, lexer, parser, utils};
    use test_case::test_case;
    use crate::parser::SyntaxNode;
    use crate::runtime::Evaluator;

    #[test_case("1 + 2 + 3", 6 ; "plus expression")]
    fn expression_parsing_test(source_code: &'static str, result: i64) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = lexer::Lexer::new(source_code);
        let tokens = lexer.lex(&diagnostic_holder);

        let mut parser = parser::Parser::new(tokens);
        let ctx = parser.parse(&mut diagnostic_holder);

        assert_eq!(ctx.len(), 1);
        assert!(ctx[0].is_some());

        for expression in ctx.iter() {
            utils::print_syntax_tree(&Box::new(expression.to_owned()), "".to_string(), true);
        }

        let evaluator = Evaluator::new(ctx.first().unwrap().clone().unwrap());

        assert_eq!(evaluator.eval(), result);
    }
}
