use crate::parser::Expression;
use std::fmt::Display;
use std::any::Any;

pub trait Result: Any + Display {
    fn as_any(&self) -> &dyn Any;
}

impl Result for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Result for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Evaluator {
    root_expression: Expression,
}

impl Evaluator {
    pub fn new(root_expression: Expression) -> Self {
        Self { root_expression }
    }

    pub fn eval(&self) -> Box<dyn Result> {
        self.eval_expression(&self.root_expression)
    }

    fn eval_expression(&self, expression: &Expression) -> Box<dyn Result> {
        match expression {
            Expression::Bool(token) => Box::new(token.literal.clone().parse::<bool>().unwrap()),
            Expression::Integer(token) => Box::new(token.literal.clone().parse::<i64>().unwrap()),
            Expression::Parenthesis(expression) => {
                self.eval_expression(&expression.clone().unwrap())
            }
            Expression::Positive(expression) => {
                self.eval_expression(&expression.clone().unwrap())
            }
            Expression::Negative(expression) => {
                let evaluated_expression = self.eval_expression(&expression.clone().unwrap());

                if let Some(val) = evaluated_expression.as_any().downcast_ref::<i64>() {
                    Box::new(-val)
                } else {
                    panic!("Cannot apply negative on non numeric types.")
                }
            }
            Expression::NOT(expression) => {
                let evaluated_expression = self.eval_expression(&expression.clone().unwrap());

                if let Some(val) = evaluated_expression.as_any().downcast_ref::<bool>() {
                    Box::new(!val)
                } else {
                    panic!("Cannot apply negative on non numeric types.")
                }
            }
            Expression::Addition(left, right) => {
                let evaluated_binary = self.eval_binary_i64(left, right);

                Box::new(evaluated_binary[0] + evaluated_binary[1])
            }
            Expression::Subtraction(left, right) => {
                let evaluated_binary = self.eval_binary_i64(left, right);

                Box::new(evaluated_binary[0] - evaluated_binary[1])
            }
            Expression::Multiplication(left, right) => {
                let evaluated_binary = self.eval_binary_i64(left, right);

                Box::new(evaluated_binary[0] * evaluated_binary[1])
            }
            Expression::Division(left, right) => {
                let evaluated_binary = self.eval_binary_i64(left, right);

                Box::new(evaluated_binary[0] / evaluated_binary[1])
            }
            Expression::Remainder(left, right) => {
                let evaluated_binary = self.eval_binary_i64(left, right);

                Box::new(evaluated_binary[0] % evaluated_binary[1])
            }
            _ => panic!("Evaluation error."),
        }
    }

    fn eval_binary_i64(
        &self,
        left: &Box<Option<Expression>>,
        right: &Box<Option<Expression>>,
    ) -> [i64; 2] {
        [
            (*self.eval_expression(&left.clone().unwrap()))
                .as_any()
                .downcast_ref::<i64>()
                .unwrap()
                .to_owned(),
            (*self.eval_expression(&right.clone().unwrap()))
                .as_any()
                .downcast_ref::<i64>()
                .unwrap()
                .to_owned(),
        ]
    }
}
