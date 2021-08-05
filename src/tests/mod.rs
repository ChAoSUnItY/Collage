#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{
        diagnostic::DiagnosticHolder,
        lexer::Lexer,
        parser::Parser,
        runtime::Evaluator,
        utils::{print_syntax_tree, to_string},
    };
    use std::any::Any;
    use std::fmt::Display;

    #[test_case("1 + 2 + 3", 6 as i64 ; "addition expression")]
    #[test_case("1 - 2 - 3", -4 as i64 ; "subtraction expression")]
    #[test_case("1 * 2 * 3", 6 as i64 ; "multiplication expression")]
    #[test_case("10 / 2 / 5", 1 as i64 ; "division expression")]
    #[test_case("10 % 4 % 2", 0 as i64 ; "remainder expression")]
    #[test_case("(20 + 1) * 5", 105 as i64 ; "parenthesis expression")]
    #[test_case("10 + 1 * 5", 15 as i64 ; "precedence test")]
    #[test_case("true", true ; "true bool literal test")]
    fn expression_parsing_test<T: Display + 'static>(source_code: &'static str, result: T) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.lex(&diagnostic_holder);

        let mut parser = Parser::new(tokens);
        let tree = parser.parse(&mut diagnostic_holder);

        print_syntax_tree(
            &Box::new(tree.root_expression.clone()),
            "".to_string(),
            true,
        );

        let evaluator = Evaluator::new(tree.root_expression.unwrap().clone());

        assert_eq!(
            to_string::<T>(evaluator.eval().as_ref()).unwrap(),
            result.to_string()
        );
    }
}
