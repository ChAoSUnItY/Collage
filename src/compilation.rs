use crate::{
    diagnostic::DiagnosticHolder,
    lexer::Lexer,
    parser::{Parser, Tree},
    runtime::Evaluator,
};
use crate::runtime::Result;

pub struct Compilation {
    source: String,
}

impl Compilation {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn eval(&self) -> Box<dyn Result> {
        let tree = self.tree();
        let evaluator = Evaluator::new(tree.root_expression.unwrap().clone());

        evaluator.eval()
    }

    pub fn tree(&self) -> Tree {
        let mut diagnostic_holder = DiagnosticHolder::new();
        let source = self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex(&diagnostic_holder);

        let mut parser = Parser::new(tokens);

        parser.parse(&mut diagnostic_holder)
    }
}
