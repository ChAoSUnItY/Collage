use crate::binder::{Binder, BoundExpression};
use crate::runtime::Result;
use crate::{
    diagnostic::DiagnosticHolder,
    lexer::Lexer,
    parser::{Parser, Tree},
    runtime::Evaluator,
};

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
        let tree = self.lex_parse();
        let binder = Binder::new();
        let bound_expression = binder.bind_expression(tree.root_expression, &mut self.holder);
        let evaluator = Evaluator::new(bound_expression.unwrap());

        evaluator.eval(&self.holder)
    }

    pub fn bind_tree(&mut self, tree: Tree) -> Option<BoundExpression> {
        let binder = Binder::new();

        binder.bind_expression(tree.root_expression, &mut self.holder)
    }

    pub fn eval_expression(
        &mut self,
        bound_expression: Option<BoundExpression>,
    ) -> Box<dyn Result> {
        let evaluator = Evaluator::new(bound_expression.unwrap());

        evaluator.eval(&self.holder)
    }

    pub fn lex_parse(&mut self) -> Tree {
        let source = self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex(&mut self.holder);

        let mut parser = Parser::new(tokens);

        parser.parse(&mut self.holder)
    }
}
