use crate::parser::Expression;

pub struct Evaluator {
    root_expression: Expression,
}

impl Evaluator {
    pub fn new(root_expression: Expression) -> Self {
        Self {
            root_expression
        }
    }

    pub fn eval(&self) -> i64 {
        self.eval_expression(&self.root_expression)
    }

    fn eval_expression(&self, expression: &Expression) -> i64 {
        match expression {
            Expression::Integer(token) =>
                token.literal.clone().parse::<i64>().unwrap(),
            Expression::Addition(left, right) =>
                self.eval_binary(left, right).iter().sum(),
            Expression::Subtraction(left, right) => {
                let evaluated_binary = self.eval_binary(left, right);

                evaluated_binary[0] - evaluated_binary[1]
            }
            _ => panic!("Unexpected expression while evaluating.")
        }
    }

    fn eval_binary(&self, left: &Box<Option<Expression>>, right: &Box<Option<Expression>>) -> [i64; 2] {
        [
            self.eval_expression(&left.clone().unwrap()),
            self.eval_expression(&right.clone().unwrap())
        ]
    }
}