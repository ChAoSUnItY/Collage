use std::fmt::Debug;

use strum_macros::ToString;

use crate::{
    diagnostic::DiagnosticHolder,
    lexer::{Token, Type},
};

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            position: 0,
            tokens,
        }
    }

    fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.position + offset]
    }

    fn next(&mut self) -> Option<&Token> {
        self.position += 1;
        self.tokens.get(self.position - 1)
    }

    fn assert(&mut self, token_type: Type) -> Option<&Token> {
        if self.peek(0).token_type == token_type {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn parse(&mut self, holder: &mut DiagnosticHolder) -> Vec<Option<Expression>> {
        let mut ctx = Vec::<Option<Expression>>::new();

        ctx.push(self.parse_expressions(holder));

        ctx
    }

    fn parse_expressions(&mut self, holder: &mut DiagnosticHolder) -> Option<Expression> {
        let mut left = self.parse_expression(holder);
        let mut current = self.next();

        while let Some(token) = current {
            if token.token_type == Type::Plus || token.token_type == Type::Minus {
                left = match &token.token_type {
                    &Type::Plus => Some(Expression::Addition(
                        Box::new(left),
                        Box::new(self.parse_expression(holder)),
                    )),
                    &Type::Minus => Some(Expression::Subtraction(
                        Box::new(left),
                        Box::new(self.parse_expression(holder)),
                    )),
                    _ => None,
                };

                current = self.next();
            } else {
                break;
            }
        }

        left
    }

    fn parse_expression(&mut self, holder: &mut DiagnosticHolder) -> Option<Expression> {
        let number_token = self.assert(Type::Number);

        if let Some(token) = number_token {
            Some(Expression::Integer(Box::new(token.to_owned())))
        } else {
            holder.error("Unexpected parsing error: Expected integer.");
            None
        }
    }
}

pub trait SyntaxNode<T>: Debug + PartialEq
where
    T: SyntaxNode<T>,
{
    fn children(&self) -> Vec<Box<Option<T>>>;

    fn as_string(&self) -> String;

    fn print(&self);
}

#[derive(Debug, Clone, PartialEq, ToString)]
pub enum Expression {
    Integer(Box<Token>),
    Float(Box<Token>),
    Addition(Box<Option<Expression>>, Box<Option<Expression>>),
    Subtraction(Box<Option<Expression>>, Box<Option<Expression>>),
}

impl SyntaxNode<Expression> for Expression {
    fn children(&self) -> Vec<Box<Option<Expression>>> {
        match self.clone() {
            Expression::Integer(token) => vec![],
            Expression::Float(token) => vec![],
            Expression::Addition(left, right) => vec![left, right],
            Expression::Subtraction(left, right) => vec![left, right],
        }
    }

    fn as_string(&self) -> String {
        match self {
            Expression::Integer(token) => format!("{}({})", self.to_string(), token.literal),
            _ => self.to_string(),
        }
    }

    fn print(&self) {}
}
