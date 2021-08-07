use crate::diagnostic::DiagnosticHolder;
use crate::parser::SyntaxNode;

pub struct Lexer {
    position: usize,
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            position: 0,
            source,
        }
    }

    fn offset(&self, segmented_source: &Vec<&str>, offset: usize) -> String {
        segmented_source[self.position + offset].to_string()
    }

    pub fn lex(&mut self, diagnostic_holder: &mut DiagnosticHolder) -> Vec<Token> {
        use unicode_segmentation::UnicodeSegmentation;

        let segmented_source =
            UnicodeSegmentation::graphemes(self.source.as_str(), true).collect::<Vec<&str>>();
        let mut tokens = Vec::<Token>::new();

        while self.position < segmented_source.len() {
            let char = &segmented_source[self.position];

            match *char {
                "+" => {
                    tokens.push(Token::new("+", Type::Plus));
                    self.position += 1;
                }
                "-" => {
                    if self.offset(&segmented_source, 1) == ">" {
                        tokens.push(Token::new("->", Type::Arrow));
                        self.position += 2;
                    } else {
                        tokens.push(Token::new("-", Type::Minus));
                        self.position += 1;
                    }
                }
                "*" => {
                    tokens.push(Token::new("*", Type::Star));
                    self.position += 1;
                }
                "/" => {
                    tokens.push(Token::new("/", Type::Slash));
                    self.position += 1;
                }
                "%" => {
                    tokens.push(Token::new("%", Type::Percent));
                    self.position += 1;
                }
                "!" => {
                    tokens.push(Token::new("!", Type::Bang));
                    self.position += 1;
                }
                "(" => {
                    tokens.push(Token::new("(", Type::OpenParenthesis));
                    self.position += 1;
                }
                ")" => {
                    tokens.push(Token::new(")", Type::CloseParenthesis));
                    self.position += 1;
                }
                ":" => {
                    if self.offset(&segmented_source, 1) == ":" {
                        tokens.push(Token::new("::", Type::DoubleColon));
                        self.position += 2;
                    }
                }
                "~" => {
                    tokens.push(Token::new("~", Type::Tilde));
                    self.position += 1;
                }
                "|" => {
                    tokens.push(Token::new("|", Type::VerticalBar));
                    self.position += 1;
                }
                "\"" => {
                    self.position += 1;

                    let start = *&self.position;

                    while self.position < segmented_source.len() && segmented_source[self.position] != "\"" {
                        self.position += 1;
                    }

                    let string_literal = &segmented_source[start..self.position].join("");
                    tokens.push(Token::new(&string_literal, Type::Literal));
                }
                _ if ("0".."9").contains(char) => {
                    let mut float = false;
                    let start = *&self.position;

                    while self.position < segmented_source.len()
                        && (("0".."9").contains(&segmented_source[self.position])
                        || segmented_source[self.position] == ".")
                    {
                        if segmented_source[self.position] == "." {
                            if float {
                                diagnostic_holder.error("Unknown number scheme, only one dot is allowed for float numbers.");
                            } else {
                                float = true;
                            }
                        }

                        self.position += 1;
                    }

                    let number = &segmented_source[start..self.position].join("");
                    tokens.push(Token::new(&number, Type::Number));
                }
                " " | "\t" | "\n" | "\r" => {
                    self.position += 1;
                }
                _ => {
                    let start = *&self.position;

                    while self.position < segmented_source.len()
                        && !segmented_source[self.position].trim().is_empty()
                    {
                        self.position += 1;
                    }

                    let identifier = &segmented_source[start..self.position].join("");
                    tokens.push(Token::new(&identifier, Type::Identifier));
                }
            }
        }

        tokens
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub literal: String,
    pub token_type: Type,
}

impl Token {
    pub fn new(literal: &str, token_type: Type) -> Self {
        Self {
            literal: literal.to_string(),
            token_type,
        }
    }
}

impl SyntaxNode<Token> for Token {
    fn children(&self) -> Vec<Box<Option<Token>>> {
        vec![]
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }

    fn print(&self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Identifier,
    Literal,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    OpenParenthesis,
    CloseParenthesis,
    Arrow,
    Tilde,
    VerticalBar,
    DoubleColon,
}

impl Type {
    pub fn unary_precedence(&self) -> usize {
        match self {
            Type::Plus | Type::Minus | Type::Bang => 3,
            _ => 0,
        }
    }

    pub fn binary_precedence(&self) -> usize {
        match self {
            Type::Star | Type::Slash | Type::Percent => 2,
            Type::Plus | Type::Minus => 1,
            _ => 0,
        }
    }
}
