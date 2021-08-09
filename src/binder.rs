use std::string::ToString;

use strum_macros::Display;

use crate::diagnostic::DiagnosticHolder;
use crate::lexer::Token;
use crate::parser::Expression;
use std::any::TypeId;

pub struct Binder {}

impl Binder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind_expression(
        &self,
        expression: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        if !holder.success() {
            return None;
        }

        if let Some(expression) = expression {
            match expression {
                Expression::Literal(token) => self.bind_literal(token.as_ref(), holder),
                Expression::Identifier(token) => self.bind_identifier(token.as_ref(), holder),
                Expression::Bool(token) => self.bind_bool(token.as_ref(), holder),
                Expression::Number(token) => self.bind_number(token.as_ref(), holder),
                Expression::Positive(expression) => self.bind_positive(*expression, holder),
                Expression::Negative(expression) => self.bind_negative(*expression, holder),
                Expression::NOT(expression) => self.bind_not(*expression, holder),
                Expression::OR(left, right) => self.bind_or(*left, *right, holder),
                Expression::AND(left, right) => self.bind_and(*left, *right, holder),
                Expression::Addition(left, right) => self.bind_addition(*left, *right, holder),
                Expression::Subtraction(left, right) => {
                    self.bind_subtraction(*left, *right, holder)
                }
                Expression::Multiplication(left, right) => {
                    self.bind_multiplication(*left, *right, holder)
                }
                Expression::Division(left, right) => self.bind_division(*left, *right, holder),
                Expression::Remainder(left, right) => self.bind_remainder(*left, *right, holder),
                Expression::Parenthesis(expression) => self.bind_parenthesis(*expression, holder),
            }
        } else {
            None
        }
    }

    fn bind_literal(
        &self,
        token: &Token,
        _holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        Some(BoundExpression::Literal(token.literal.to_owned()))
    }

    fn bind_identifier(
        &self,
        token: &Token,
        _holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        Some(BoundExpression::Identifier(token.literal.to_owned()))
    }

    fn bind_bool(&self, token: &Token, _holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Bool(token.literal.to_owned()))
    }

    fn bind_number(
        &self,
        token: &Token,
        _holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        Some(BoundExpression::Number(token.literal.to_owned()))
    }

    fn bind_positive(
        &self,
        expression: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_expression = self.bind_expression(expression, holder);

        if bound_expression.get_type() != BoundType::Number {
            holder.error(&*format!(
                "Cannot apply positive on type \"{:}\"",
                bound_expression.get_type().to_string()
            ))
        }

        Some(BoundExpression::Identity(Box::new(bound_expression)))
    }

    fn bind_negative(
        &self,
        expression: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_expression = self.bind_expression(expression, holder);

        if bound_expression.get_type() != BoundType::Number {
            holder.error(&*format!(
                "Cannot apply negative on type \"{:}\"",
                bound_expression.get_type().to_string()
            ))
        }

        Some(BoundExpression::Negation(Box::new(bound_expression)))
    }

    fn bind_not(
        &self,
        expression: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_expression = self.bind_expression(expression, holder);

        if bound_expression.get_type() != BoundType::Bool {
            holder.error(&*format!(
                "Cannot apply logical NOT on type \"{:}\"",
                bound_expression.get_type().to_string()
            ))
        }

        Some(BoundExpression::LogicalNot(Box::new(bound_expression)))
    }

    fn bind_or(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Bool || bound_right.get_type() != BoundType::Bool {
            holder.error(&*format!(
                "Cannot apply logical OR on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::LogicalOr(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_and(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Bool || bound_right.get_type() != BoundType::Bool {
            holder.error(&*format!(
                "Cannot apply logical AND on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::LogicalAnd(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_addition(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Number || bound_right.get_type() != BoundType::Number
        {
            holder.error(&*format!(
                "Cannot apply addition on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::Addition(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_subtraction(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Number || bound_right.get_type() != BoundType::Number
        {
            holder.error(&*format!(
                "Cannot apply subtraction on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::Addition(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_multiplication(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Number || bound_right.get_type() != BoundType::Number
        {
            holder.error(&*format!(
                "Cannot apply multiplication on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::Addition(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_division(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Number || bound_right.get_type() != BoundType::Number
        {
            holder.error(&*format!(
                "Cannot apply division on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::Addition(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_remainder(
        &self,
        left: Option<Expression>,
        right: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        let bound_left = self.bind_expression(left, holder);
        let bound_right = self.bind_expression(right, holder);

        if bound_left.get_type() != BoundType::Number || bound_right.get_type() != BoundType::Number
        {
            holder.error(&*format!(
                "Cannot apply remainder on type \"{:}\" and \"{:}\"",
                bound_left.get_type().to_string(),
                bound_right.get_type().to_string()
            ))
        }

        Some(BoundExpression::Addition(
            Box::new(bound_left),
            Box::new(bound_right),
        ))
    }

    fn bind_parenthesis(
        &self,
        expression: Option<Expression>,
        holder: &mut DiagnosticHolder,
    ) -> Option<BoundExpression> {
        Some(BoundExpression::Parenthesis(Box::new(
            self.bind_expression(expression, holder),
        )))
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
pub enum BoundType {
    #[strum(serialize = "unidentified")]
    Unidentified,
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "bool")]
    Bool,
    #[strum(serialize = "number")]
    Number,
}

trait TypeDestructable {
    fn get_type(&self) -> BoundType;
}

impl TypeDestructable for Box<Option<BoundExpression>> {
    fn get_type(&self) -> BoundType {
        self.as_ref().get_type()
    }
}

impl TypeDestructable for Option<BoundExpression> {
    fn get_type(&self) -> BoundType {
        self.as_ref()
            .map_or_else(|| BoundType::Unidentified, |e| e.get_type())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoundExpression {
    Literal(String),
    Identifier(String),
    Bool(String),
    Number(String),
    Identity(Box<Option<BoundExpression>>),
    Negation(Box<Option<BoundExpression>>),
    LogicalNot(Box<Option<BoundExpression>>),
    LogicalAnd(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    LogicalOr(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Addition(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Subtraction(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Multiplication(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Division(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Remainder(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Parenthesis(Box<Option<BoundExpression>>),
}

impl BoundExpression {
    pub fn get_type(&self) -> BoundType {
        match self {
            BoundExpression::Literal(_) => BoundType::String,
            BoundExpression::Identifier(_) => BoundType::Unidentified,
            BoundExpression::Bool(_) => BoundType::Bool,
            BoundExpression::Number(_) => BoundType::Number,
            BoundExpression::Identity(expression) => expression.get_type(),
            BoundExpression::Negation(expression) => expression.get_type(),
            BoundExpression::LogicalNot(expression) => expression.get_type(),
            BoundExpression::LogicalOr(_, _) => BoundType::Bool,
            BoundExpression::LogicalAnd(_, _) => BoundType::Bool,
            BoundExpression::Addition(_, _) => BoundType::Number,
            BoundExpression::Subtraction(_, _) => BoundType::Number,
            BoundExpression::Multiplication(_, _) => BoundType::Number,
            BoundExpression::Division(_, _) => BoundType::Number,
            BoundExpression::Remainder(_, _) => BoundType::Number,
            BoundExpression::Parenthesis(expression) => expression.get_type(),
        }
    }
}
