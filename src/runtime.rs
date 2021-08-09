use crate::binder::{BoundExpression, BoundType};
use crate::diagnostic::DiagnosticHolder;
use std::any::Any;
use std::fmt::Display;
use std::ops::Deref;

pub trait Result: Any + Display {
    fn as_any(&self) -> &dyn Any;

    fn as_display(&self) -> &dyn Display;
}

impl Result for String {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_display(&self) -> &dyn Display {
        self
    }
}

impl Result for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_display(&self) -> &dyn Display {
        self
    }
}

impl Result for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_display(&self) -> &dyn Display {
        self
    }
}

impl Result for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_display(&self) -> &dyn Display {
        self
    }
}

pub struct Evaluator {
    root_expression: BoundExpression,
}

impl Evaluator {
    pub fn new(root_expression: BoundExpression) -> Self {
        Self { root_expression }
    }

    pub fn eval(&self, holder: &DiagnosticHolder) -> Box<dyn Result> {
        if holder.success() {
            self.eval_expression(&self.root_expression)
        } else {
            Box::new("<Error>".to_string())
        }
    }

    fn eval_expression(&self, expression: &BoundExpression) -> Box<dyn Result> {
        match expression {
            BoundExpression::Literal(string) => Box::new(string.clone()),
            BoundExpression::Bool(string) => Box::new(string.clone().parse::<bool>().unwrap()),
            BoundExpression::Number(string) => Box::new(string.clone().parse::<f64>().unwrap()),
            BoundExpression::Parenthesis(expression) => {
                self.eval_expression(&expression.clone().unwrap())
            }
            BoundExpression::Identity(expression) => {
                self.eval_expression(&expression.clone().unwrap())
            }
            BoundExpression::Negation(expression) => {
                let evaluated_expression = self.eval_expression(&expression.clone().unwrap());

                if let Some(val) = evaluated_expression.as_any().downcast_ref::<f64>() {
                    Box::new(-val)
                } else {
                    panic!("Cannot apply negative on non numeric types.")
                }
            }
            BoundExpression::LogicalNot(expression) => {
                let evaluated_expression = self.eval_expression(&expression.clone().unwrap());

                if let Some(val) = evaluated_expression.as_any().downcast_ref::<bool>() {
                    Box::new(!val)
                } else {
                    panic!("Cannot apply negative on non numeric types.")
                }
            }
            BoundExpression::LogicalOr(left, right) => {
                let evaluated_binary = self.eval_binary::<bool>(left, right);

                Box::new(evaluated_binary[0] || evaluated_binary[1])
            }
            BoundExpression::LogicalAnd(left, right) => {
                let evaluated_binary = self.eval_binary::<bool>(left, right);

                Box::new(evaluated_binary[0] && evaluated_binary[1])
            }
            BoundExpression::NotEqual(left, right) => {
                let evaluated_left = self.eval_expression(&left.clone().unwrap());
                let evaluated_right = self.eval_expression(&right.clone().unwrap());

                Box::new(evaluated_left.to_string() != evaluated_right.to_string())
            }
            BoundExpression::Equal(left, right) => {
                let evaluated_left = self.eval_expression(&left.clone().unwrap());
                let evaluated_right = self.eval_expression(&right.clone().unwrap());

                Box::new(evaluated_left.to_string() == evaluated_right.to_string())
            }
            BoundExpression::Addition(left, right) => {
                let evaluated_binary = self.eval_binary::<f64>(left, right);

                Box::new(evaluated_binary[0] + evaluated_binary[1])
            }
            BoundExpression::Subtraction(left, right) => {
                let evaluated_binary = self.eval_binary::<f64>(left, right);

                Box::new(evaluated_binary[0] - evaluated_binary[1])
            }
            BoundExpression::Multiplication(left, right) => {
                let evaluated_binary = self.eval_binary::<f64>(left, right);

                Box::new(evaluated_binary[0] * evaluated_binary[1])
            }
            BoundExpression::Division(left, right) => {
                let evaluated_binary = self.eval_binary::<f64>(left, right);

                Box::new(evaluated_binary[0] / evaluated_binary[1])
            }
            BoundExpression::Remainder(left, right) => {
                let evaluated_binary = self.eval_binary::<f64>(left, right);

                Box::new(evaluated_binary[0] % evaluated_binary[1])
            }
            _ => panic!("Evaluation error."),
        }
    }

    fn eval_binary<T>(
        &self,
        left: &Box<Option<BoundExpression>>,
        right: &Box<Option<BoundExpression>>,
    ) -> [T; 2]
    where
        T: Result + Clone,
    {
        [
            (*self.eval_expression(&left.clone().unwrap()))
                .as_any()
                .downcast_ref::<T>()
                .unwrap()
                .to_owned(),
            (*self.eval_expression(&right.clone().unwrap()))
                .as_any()
                .downcast_ref::<T>()
                .unwrap()
                .to_owned(),
        ]
    }
}
