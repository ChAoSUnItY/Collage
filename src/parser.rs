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

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position.clone() + offset)
    }

    fn assert(&mut self, token_type: Type) -> Option<&Token> {
        let current = self.peek(0);

        if let Some(token) = current {
            if token.token_type == token_type {
                let token = &self.tokens[self.position];
                self.position += 1;
                return Some(token);
            }
        }

        None
    }

    pub fn parse(&mut self, holder: &mut DiagnosticHolder) -> Tree {
        Tree {
            root_expression: self.parse_expression(0, holder),
        }
    }

    fn parse_expression(
        &mut self,
        parent_precedence: usize,
        holder: &mut DiagnosticHolder,
    ) -> Option<Expression> {
        let mut left: Option<Expression> = None;
        if let Some(precedence_token) = self.tokens.get(self.position) {
            let precedence = precedence_token.token_type.unary_precedence();

            left = if precedence != 0 && precedence >= parent_precedence {
                self.position += 1;

                match precedence_token.token_type {
                    Type::Plus => Some(Expression::Positive(Box::new(
                        self.parse_expression(precedence, holder),
                    ))),
                    Type::Minus => Some(Expression::Negative(Box::new(
                        self.parse_expression(precedence, holder),
                    ))),
                    Type::Bang => Some(Expression::NOT(Box::new(
                        self.parse_expression(precedence, holder),
                    ))),
                    _ => None,
                }
            } else {
                self.parse_literal_expression(holder)
            }
        }

        loop {
            if let Some(precedence_token) = self.tokens.get(self.position) {
                let precedence = precedence_token.token_type.binary_precedence();

                if precedence == 0 || precedence <= parent_precedence {
                    break;
                }
                self.position += 1;

                left = match precedence_token.token_type {
                    Type::DoubleAmpersand => Some(Expression::AND(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::DoublePipe => Some(Expression::OR(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::Plus => Some(Expression::Addition(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::Minus => Some(Expression::Subtraction(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::Star => Some(Expression::Multiplication(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::Slash => Some(Expression::Division(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    Type::Percent => Some(Expression::Remainder(
                        Box::new(left),
                        Box::new(self.parse_expression(precedence, holder)),
                    )),
                    _ => None,
                };
            } else {
                break;
            }
        }

        left
    }

    fn parse_literal_expression(&mut self, holder: &mut DiagnosticHolder) -> Option<Expression> {
        let current = self.tokens.get(self.position);

        if let Some(token) = current {
            match token.token_type {
                Type::OpenParenthesis => {
                    std::mem::drop(self.assert(Type::OpenParenthesis));
                    let expression = self.parse_expression(0, holder);
                    std::mem::drop(self.assert(Type::CloseParenthesis));

                    Some(Expression::Parenthesis(Box::new(expression)))
                }
                Type::Number => {
                    let number_token = self.assert(Type::Number);

                    if let Some(token) = number_token {
                        Some(Expression::Number(Box::new(token.to_owned())))
                    } else {
                        holder.error("Unexpected parsing error: Expected integer.");
                        None
                    }
                }
                Type::Literal => {
                    let string_literal = self.assert(Type::Literal);

                    if let Some(token) = string_literal {
                        Some(Expression::Literal(Box::new(token.to_owned())))
                    } else {
                        holder.error("Unexpected parsing error: Expected string literal.");
                        None
                    }
                }
                Type::Identifier => {
                    let identifier_token = self.assert(Type::Identifier);

                    if let Some(token) = identifier_token {
                        match token.literal.as_str() {
                            "true" | "false" => Some(Expression::Bool(Box::new(token.to_owned()))),
                            _ => Some(Expression::Identifier(Box::new(token.to_owned()))),
                        }
                    } else {
                        holder
                            .error("Unexpected parsing error: Expected identifier / type literal.");
                        None
                    }
                }
                _ => panic!("Unknown expression: {:?}", token),
            }
        } else {
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

#[derive(Debug, Clone)]
pub struct Tree {
    pub root_expression: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, ToString)]
pub enum Expression {
    Identifier(Box<Token>),
    Literal(Box<Token>),
    Bool(Box<Token>),
    Number(Box<Token>),
    Positive(Box<Option<Expression>>),
    Negative(Box<Option<Expression>>),
    NOT(Box<Option<Expression>>),
    AND(Box<Option<Expression>>, Box<Option<Expression>>),
    OR(Box<Option<Expression>>, Box<Option<Expression>>),
    Addition(Box<Option<Expression>>, Box<Option<Expression>>),
    Subtraction(Box<Option<Expression>>, Box<Option<Expression>>),
    Multiplication(Box<Option<Expression>>, Box<Option<Expression>>),
    Division(Box<Option<Expression>>, Box<Option<Expression>>),
    Remainder(Box<Option<Expression>>, Box<Option<Expression>>),
    Parenthesis(Box<Option<Expression>>),
}

impl SyntaxNode<Expression> for Expression {
    fn children(&self) -> Vec<Box<Option<Expression>>> {
        match self.clone() {
            Expression::Positive(expression) => vec![expression],
            Expression::Negative(expression) => vec![expression],
            Expression::NOT(expression) => vec![expression],
            Expression::OR(left, right) => vec![left, right],
            Expression::AND(left, right) => vec![left, right],
            Expression::Addition(left, right) => vec![left, right],
            Expression::Subtraction(left, right) => vec![left, right],
            Expression::Multiplication(left, right) => vec![left, right],
            Expression::Division(left, right) => vec![left, right],
            Expression::Remainder(left, right) => vec![left, right],
            Expression::Parenthesis(expression) => vec![expression],
            _ => vec![],
        }
    }

    fn as_string(&self) -> String {
        match self {
            Expression::Literal(token) => format!("{}({})", self.to_string(), token.literal),
            Expression::Bool(token) => format!("{}({})", self.to_string(), token.literal),
            Expression::Number(token) => format!("{}({})", self.to_string(), token.literal),
            _ => self.to_string(),
        }
    }

    fn print(&self) {}
}
