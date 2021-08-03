use crate::lexer::{Token, Type};

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

    pub fn parse(&mut self) -> Vec<CollageContext> {
        let mut ctx = Vec::<CollageContext>::new();

        self.parse_ctx(&mut ctx);

        ctx
    }

    fn parse_ctx(&mut self, ctx: &mut Vec<CollageContext>) {
        while self.position < self.tokens.len() {
            match self.peek(0).token_type {
                Type::Identifier => {
                    if self.peek(1).token_type == Type::DoubleColon {
                        let function_name = &self.tokens[self.position];
                        self.position += 2;
                        let mut argument_types = Vec::<String>::new();

                        while self.peek(0).token_type == Type::Identifier {
                            let type_literal = &self.tokens[self.position];
                            argument_types.push(type_literal.literal.clone());
                            self.position += 2;

                            if self.position < self.tokens.len()
                                && self.peek(0).token_type == Type::Identifier
                            {
                                continue;
                            } else {
                                break;
                            }
                        }

                        let return_type = argument_types.pop().unwrap();

                        ctx.push(CollageContext::FunctionDeclaration(function_name.literal.clone(),argument_types, return_type));
                    }
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CollageContext {
    FunctionDeclaration(String, Vec<String>, String),
    Data
}
