use crate::{
    diagnostic::DiagnosticHolder,
    lexer::Lexer,
    parser::{Parser, Tree},
    runtime::Evaluator,
};
use crate::binder::Binder;
use crate::runtime::Result;

pub struct Compilation {
    source: String,
    pub holder: DiagnosticHolder,
}

impl Compilation {
    pub fn new(source: String) -> Self {
        Self {
            source,
            holder: DiagnosticHolder::new(),
        }
    }

    pub fn eval(&mut self) -> Box<dyn Result> {
        let tree = self.tree();
        let binder = Binder::new();
        let bound_expression = binder.bind_expression(tree.root_expression, &mut self.holder);
        let evaluator = Evaluator::new(bound_expression.unwrap());

        evaluator.eval(&self.holder)
    }

    pub fn tree(&mut self) -> Tree {
        let source = self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex(&mut self.holder);

        let mut parser = Parser::new(tokens);

        parser.parse(&mut self.holder)
    }
}
