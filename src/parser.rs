use crate::lexer::{Token, Type};

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
    scope: Scope,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            position: 0,
            tokens,
            scope: Scope::new(vec![], vec![]),
        }
    }

    fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.position + offset]
    }

    pub fn parse(&mut self) -> Vec<CollageContext> {
        let mut ctx = Vec::<CollageContext>::new();

        self.parse_ctx(&mut ctx);

        ctx
    }

    fn parse_ctx(&mut self, ctx: &mut Vec<CollageContext>) {
        while self.position < self.tokens.len() {
            match self.peek(0).token_type {
                Type::Identifier => match self.peek(1).token_type {
                    Type::DoubleColon => {
                        let function_name = &self.tokens[self.position].literal;
                        self.position += 2;
                        let mut argument_types = Vec::<String>::new();

                        while self.peek(0).token_type == Type::Identifier {
                            let type_literal = self.peek(0);
                            argument_types.push(type_literal.literal.clone());
                            self.position += 1;

                            if self.position < self.tokens.len()
                                && self.peek(0).token_type == Type::Arrow
                            {
                                self.position += 1;
                                continue;
                            } else {
                                break;
                            }
                        }

                        let return_type = argument_types.pop().unwrap();

                        if self.scope.has_same_function(
                            &function_name,
                            &argument_types,
                            &return_type,
                        ) {
                            panic!("Dupicated function declarations: {:?}", &function_name);
                        } else {
                            self.scope.add_function(FunctionSignature {
                                name: function_name.clone(),
                                types: argument_types.to_vec(),
                                return_type: return_type.clone(),
                            })
                        }

                        ctx.push(CollageContext::FunctionDeclaration(
                            function_name.clone(),
                            argument_types,
                            return_type,
                        ));
                    }
                    Type::Identifier => {
                        let function_name = &self.tokens[self.position].literal;
                        self.position += 1;
                        let mut argument_names = Vec::<String>::new();

                        while self.peek(0).token_type == Type::Identifier {
                            let argument_name = self.peek(0);
                            argument_names.push(argument_name.literal.clone());
                            self.position += 1;
                        }

                        self.position += 1;

                        let expression = self.parse_expression();

                        if !self.scope.has_function(function_name, argument_names.len()) {
                            panic!("Unknown function implementation: {}", function_name)
                        }

                        ctx.push(CollageContext::TildeFunctionImplementation(
                            function_name.clone(),
                            argument_names,
                            expression,
                        ))
                    }
                    _ => {}
                },
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    fn parse_expression(&self) -> CollageExpression {
        match self.peek(0).token_type {
            Type::Number => {
                CollageExpression::Integer(self.peek(0).literal.clone().parse::<i64>().unwrap())
            }
            _ => {
                panic!("Unknown expression")
            }
        }
    }
}

struct Scope {
    signatures: Vec<FunctionSignature>,
    variables: Vec<Variable>,
}

impl Scope {
    pub fn new(signatures: Vec<FunctionSignature>, variables: Vec<Variable>) -> Self {
        Self {
            signatures,
            variables,
        }
    }

    pub fn inherit(scope: &Scope) -> Self {
        Self {
            signatures: scope.signatures.to_vec(),
            variables: scope.variables.to_vec(),
        }
    }

    pub fn add_function(&mut self, signature: FunctionSignature) {
        self.signatures.push(signature)
    }

    pub fn has_function(&self, function_name: &str, len: usize) -> bool {
        self.signatures
            .iter()
            .any(|f| f.name == function_name && f.types.len() == len)
    }

    pub fn has_same_function(
        &self,
        function_name: &str,
        types: &Vec<String>,
        return_type: &str,
    ) -> bool {
        self.signatures.iter().any(|f| {
            f.name == function_name
                && types.iter().enumerate().all(|(i, t)| &f.types[i] == t)
                && return_type == f.return_type
        })
    }
}

#[derive(Debug, Clone)]
struct FunctionSignature {
    name: String,
    types: Vec<String>,
    return_type: String,
}

impl FunctionSignature {
    pub fn eq(&self, name: &String, types: &Vec<String>, return_type: &String) -> bool {
        &self.name == name
            && self.types.iter().zip(types).all(|(a, b)| a == b)
            && &self.return_type == return_type
    }
}

impl PartialEq for FunctionSignature {
    fn eq(&self, other: &Self) -> bool {
        let FunctionSignature {
            name,
            types,
            return_type,
        } = other;

        self.eq(name, types, return_type)
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    variable_type: String,
}

#[derive(Debug)]
pub enum CollageContext {
    FunctionDeclaration(String, Vec<String>, String),
    TildeFunctionImplementation(String, Vec<String>, CollageExpression),
}

#[derive(Debug)]
pub enum CollageExpression {
    Integer(i64),
    Float(f64),
}
