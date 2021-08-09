#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::binder::Binder;
    use crate::diagnostic::Unit;
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
    #[test_case("true || false", true ; "OR expression test")]
    #[test_case("true && true", true ; "AND expression test")]
    #[test_case("1 == 1", true ; "equal expression test")]
    #[test_case("1 != 1", false ; "not equal expression test")]
    #[test_case("2 > 1", true ; "greater than expression test")]
    #[test_case("2 >= 1", true ; "greater equal than expression test")]
    #[test_case("2 < 1", false ; "less than expression test")]
    #[test_case("2 <= 1", false ; "less equal than expression test")]
    fn eval_test<T: Display + 'static>(source_code: &'static str, expected_result: T) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = Lexer::new(source_code.trim().to_string());
        let tokens = lexer.lex(&mut diagnostic_holder);

        assert!(diagnostic_holder.success());

        let mut parser = Parser::new(tokens);
        let tree = parser.parse(&mut diagnostic_holder);

        assert!(diagnostic_holder.success());

        let binder = Binder::new();
        let bound_expression = binder.bind_expression(tree.root_expression, &mut diagnostic_holder);

        assert!(diagnostic_holder.success());

        let evaluator = Evaluator::new(bound_expression.unwrap());
        let result = evaluator.eval(&diagnostic_holder);

        assert!(diagnostic_holder.success());

        assert_eq!(
            to_string::<T>(result.as_ref().as_any()).unwrap(),
            expected_result.to_string()
        );
    }

    #[test_case("1 || true", &["Cannot apply logical OR on type \"number\" and \"bool\""] ; "type check test A")]
    fn parsing_error_test(source_code: &'static str, expected_messages: &[&'static str]) {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let mut lexer = Lexer::new(source_code.trim().to_string());
        let tokens = lexer.lex(&mut diagnostic_holder);

        assert!(diagnostic_holder.success());

        let mut parser = Parser::new(tokens);
        let tree = parser.parse(&mut diagnostic_holder);

        let mut binder = Binder::new();
        std::mem::drop(binder.bind_expression(tree.root_expression, &mut diagnostic_holder));

        assert!(!diagnostic_holder.success());

        assert_eq!(
            diagnostic_holder.diagonistic_units.len(),
            expected_messages.len()
        );

        for message in diagnostic_holder
            .diagonistic_units
            .iter()
            .zip(expected_messages)
        {
            match message {
                (unit, expected) => {
                    if let Unit::Error(string) = unit {
                        assert_eq!(string, expected);
                    }
                }
            }
        }
    }
}
