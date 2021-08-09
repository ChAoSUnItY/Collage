#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::binder::Binder;
    use crate::{
        diagnostic::DiagnosticHolder,
        lexer::Lexer,
        parser::Parser,
        runtime::Evaluator,
        utils::{print_syntax_tree, to_string},
    };
    use std::fmt::Display;

    #[test_case("\"Hi\"", "Hi" ; "string literal test")]
    #[test_case("true", true ; "bool literal test")]
    #[test_case("1", 1. ; "integer literal expression")]
    #[test_case("1.", 1. ; "float literal expression A")]
    #[test_case("1.0", 1. ; "float literal expression B")]
    #[test_case("1 + 2 + 3", 6. ; "addition expression")]
    #[test_case("1 - 2 - 3", -4. ; "subtraction expression")]
    #[test_case("1 * 2 * 3", 6. ; "multiplication expression")]
    #[test_case("10 / 2 / 5", 1. ; "division expression")]
    #[test_case("10 % 4 % 2", 0. ; "remainder expression")]
    #[test_case("(20 + 1) * 5", 105. ; "parenthesis expression")]
    #[test_case("10 + 1 * 5", 15. ; "precedence test")]
    #[test_case("+1 + 1", 2. ; "positive expression test")]
    #[test_case("-1 + 1", 0. ; "negative expression test")]
    #[test_case("!true", false ; "NOT expression test")]
    fn expression_parsing_test<T: Display + 'static>(source_code: &'static str, result: T) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = Lexer::new(source_code.trim().to_string());
        let tokens = lexer.lex(&mut diagnostic_holder);

        let mut parser = Parser::new(tokens);
        let tree = parser.parse(&mut diagnostic_holder);

        print_syntax_tree(
            &Box::new(tree.root_expression.clone()),
            "".to_string(),
            true,
        );

        let binder = Binder::new();
        let bound_expression = binder.bind_expression(tree.root_expression, &mut diagnostic_holder);

        let evaluator = Evaluator::new(bound_expression.unwrap());
        let result = evaluator.eval(&diagnostic_holder);

        assert!(diagnostic_holder.success());

        assert_eq!(
            to_string::<T>(result.as_ref().as_any()).unwrap(),
            result.to_string()
        );
    }
}
