use crate::lexer::{Token, Type};

pub struct Parser {
    position: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            position: 0,
            tokens
        }
    }

    fn peek(&self, offset: usize) -> Token {
        self.tokens[self.position + offset]
    }

    pub fn parse(&mut self) -> Vec<CollageContext> {
        let mut ctx = Vec::<CollageContext>::new();

        self.parse_ctx(&ctx);

        ctx
    }

    pub fn parse_ctx(&mut self, ctx: &Vec<CollageContext>) {
        while self.position < self.tokens.len() {
            match self.peek(1).token_type {
                Type::DoubleColon => {
                    
                } 
            }
        }
    } 
}

pub enum CollageContext<'a> {
    FunctionDeclaration { function_name: &'a str, argument_types: Vec<&'a str>, return_type: &'a str}
}