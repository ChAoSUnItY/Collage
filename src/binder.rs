use crate::diagnostic::DiagnosticHolder;
use crate::lexer::Token;
use crate::parser::Expression;

pub struct Binder {}

impl Binder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind_expression(&self, expression: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        if !holder.success() {
            return None;
        }

        if let Some(expression) = expression {
            match expression {
                Expression::Literal(token) => self.bind_literal(token.as_ref(), holder),
                Expression::Bool(token) => self.bind_bool(token.as_ref(), holder),
                Expression::Integer(token) => self.bind_integer(token.as_ref(), holder),
                Expression::Float(token) => self.bind_float(token.as_ref(), holder),
                Expression::Positive(expression) => self.bind_positive(*expression, holder),
                Expression::Negative(expression) => self.bind_negative(*expression, holder),
                Expression::NOT(expression) => None,
                Expression::Addition(_, _) => None,
                Expression::Subtraction(_, _) => None,
                Expression::Multiplication(_, _) => None,
                Expression::Division(_, _) => None,
                Expression::Remainder(_, _) => None,
                Expression::Parenthesis(expression) => None,
            }
        } else {
            None
        }
    }

    fn bind_literal(&self, token: &Token, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Literal(token.literal.to_owned()))
    }

    fn bind_bool(&self, token: &Token, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Bool(token.literal.to_owned()))
    }

    fn bind_integer(&self, token: &Token, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Integer(token.literal.to_owned()))
    }

    fn bind_float(&self, token: &Token, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Float(token.literal.to_owned()))
    }

    fn bind_positive(&self, expression: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Identity(Box::new(self.bind_expression(expression, holder))))
    }

    fn bind_negative(&self, expression: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Negation(Box::new(self.bind_expression(expression, holder))))
    }

    fn bind_not(&self, expression: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::NOT(Box::new(self.bind_expression(expression, holder))))
    }

    fn bind_addition(&self, left: Option<Expression>, right: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Addition(
            Box::new(self.bind_expression(left, holder)),
            Box::new(self.bind_expression(right, holder)),
        ))
    }

    fn bind_subtraction(&self, left: Option<Expression>, right: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Subtraction(
            Box::new(self.bind_expression(left, holder)),
            Box::new(self.bind_expression(right, holder)),
        ))
    }

    fn bind_multiplication(&self, left: Option<Expression>, right: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Multiplication(
            Box::new(self.bind_expression(left, holder)),
            Box::new(self.bind_expression(right, holder)),
        ))
    }

    fn bind_division(&self, left: Option<Expression>, right: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Division(
            Box::new(self.bind_expression(left, holder)),
            Box::new(self.bind_expression(right, holder)),
        ))
    }

    fn bind_remainder(&self, left: Option<Expression>, right: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Remainder(
            Box::new(self.bind_expression(left, holder)),
            Box::new(self.bind_expression(right, holder)),
        ))
    }

    fn bind_parenthesis(&self, expression: Option<Expression>, holder: &mut DiagnosticHolder) -> Option<BoundExpression> {
        Some(BoundExpression::Parenthesis(Box::new(self.bind_expression(expression, holder))))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoundType {
    Unidentified,
    String,
    Bool,
    I64,
    F64
}

impl BoundType {
    pub fn is_numeric(&self) -> bool {
        match self {
            BoundType::I64 | BoundType::F64 => true,
            _ => false
        }
    }
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
        self.as_ref().map_or_else(|| BoundType::Unidentified, |e| e.get_type())
    }
}

pub enum BoundExpression {
    Literal(String),
    Bool(String),
    Integer(String),
    Float(String),
    Identity(Box<Option<BoundExpression>>),
    Negation(Box<Option<BoundExpression>>),
    NOT(Box<Option<BoundExpression>>),
    Addition(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Subtraction(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Multiplication(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Division(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Remainder(Box<Option<BoundExpression>>, Box<Option<BoundExpression>>),
    Parenthesis(Box<Option<BoundExpression>>),
}

impl BoundExpression {
    pub fn get_type(&self) -> BoundType {
        fn get_binary_type(left: BoundType, right: BoundType, default: BoundType) -> BoundType {
            if left.is_numeric() && right.is_numeric() {
                if left == BoundType::F64 || right == BoundType::F64 {
                    BoundType::F64
                } else {
                    BoundType::I64
                }
            } else {
                default
            }
        }

        match self {
            BoundExpression::Literal(_) => BoundType::String,
            BoundExpression::Bool(_) => BoundType::Bool,
            BoundExpression::Integer(_) => BoundType::I64,
            BoundExpression::Float(_) => BoundType::F64,
            BoundExpression::Identity(expression) => expression.get_type(),
            BoundExpression::Negation(expression) => expression.get_type(),
            BoundExpression::NOT(expression) => expression.get_type(),
            BoundExpression::Addition(left, right) =>
                get_binary_type(left.get_type(), right.get_type(), BoundType::I64),
            BoundExpression::Subtraction(left, right) =>
                get_binary_type(left.get_type(), right.get_type(), BoundType::I64),
            BoundExpression::Multiplication(left, right) =>
                get_binary_type(left.get_type(), right.get_type(), BoundType::I64),
            BoundExpression::Division(left, right) =>
                get_binary_type(left.get_type(), right.get_type(), BoundType::F64),
            BoundExpression::Remainder(left, right) =>
                get_binary_type(left.get_type(), right.get_type(), BoundType::I64),
            BoundExpression::Parenthesis(expression) => expression.get_type()
        }
    }
}