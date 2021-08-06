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
        let mut holder = DiagnosticHolder::new();
        let tree = self.tree(&mut holder);
        let evaluator = Evaluator::new(tree.root_expression.unwrap().clone());

        evaluator.eval(&holder)
    }

    pub fn tree(&self, holder: &mut DiagnosticHolder) -> Tree {
        let source = self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex(holder);

        let mut parser = Parser::new(tokens);

        parser.parse(holder)
    }
}
