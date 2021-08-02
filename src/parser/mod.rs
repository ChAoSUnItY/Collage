use js_sys::Reflect;
use pest::{error::Error, Parser};
use wasm_bindgen::JsValue;

#[derive(Parser)]
#[grammar = "parser/collage.pest"]
struct CollageParser;

#[derive(Debug, PartialEq)]
pub enum CollageStatement {
    Assignment(String, Box<CollageExpression>),
}

impl CollageStatement {
    pub unsafe fn as_obj(&self) -> JsValue {
        let obj = js_sys::Object::new();

        match self {
            Self::Assignment(identifier, expression) => {
                Reflect::set(&obj, &"identifier".into(), &JsValue::from_str(&*identifier));
                Reflect::set(&obj, &"expression".into(), &expression.as_obj());
            }
        }

        obj.into()
    }
}

#[derive(Debug, PartialEq)]
pub enum CollageExpression {
    BinaryExpression(Box<CollageExpression>, String, Box<CollageExpression>),
    WrappedExpression(Box<CollageExpression>),
    Number(f64),
    String(String),
}

impl CollageExpression {
    pub fn as_obj(&self) -> JsValue {
        let obj = js_sys::Object::new();

        unsafe {
            match self {
                Self::BinaryExpression(left, operator, right) => {
                    Reflect::set(&obj, &"left".into(), &right.as_obj());
                }
                Self::WrappedExpression(expression) => {
                    Reflect::set(&obj, &"expression".into(), &expression.as_obj());
                }
                Self::Number(number) => {
                    Reflect::set(&obj, &"expression".into(), &JsValue::from_f64(*number));
                }
                Self::String(string) => {
                    Reflect::set(&obj, &"expression".into(), &JsValue::from_str(&*string));
                }
            }
        }

        obj.into()
    }
}

pub fn parse(file_source: &str) -> Result<Vec<CollageStatement>, Error<Rule>> {
    let collage_rules = CollageParser::parse(Rule::collageContext, file_source)?;
    let mut statements = Vec::<CollageStatement>::new();

    use pest::iterators::Pair;

    fn parse_expression(pair: Pair<Rule>) -> CollageExpression {
        match pair.as_rule() {
            Rule::binaryExpression => {
                let mut inner = pair.into_inner();
                let left = parse_expression(inner.next().unwrap());
                let operator = inner.next().unwrap().as_str().to_string();
                let right = parse_expression(inner.next().unwrap());

                CollageExpression::BinaryExpression(Box::new(left), operator, Box::new(right))
            }
            Rule::wrappedExpression => CollageExpression::WrappedExpression(Box::new(
                parse_expression(pair.into_inner().next().unwrap()),
            )),
            Rule::integer | Rule::decimal => CollageExpression::Number(
                pair.as_str().parse::<f64>().expect("Invalid number format"),
            ),
            Rule::string => CollageExpression::String(pair.as_str().to_string()),
            _ => panic!("{:?}", pair.as_rule()),
        }
    }

    for pair in collage_rules {
        match pair.as_rule() {
            Rule::assignment => {
                let mut inner = pair.into_inner();
                let identifier = inner.next().unwrap().as_str();
                let expression = parse_expression(inner.next().unwrap());

                statements.push(CollageStatement::Assignment(
                    identifier.to_string(),
                    Box::new(expression),
                ))
            }
            _ => {}
        }
    }

    Ok(statements)
}
