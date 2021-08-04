use crate::diagnostic::DiagnosticHolder;
use crate::parser::SyntaxNode;

pub struct Lexer {
    position: usize,
    source: &'static str,
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Self {
            position: 0,
            source,
        }
    }

    fn offset(&self, segmented_source: &Vec<&str>, offset: usize) -> String {
        segmented_source[self.position + offset].to_string()
    }

    pub fn lex(&mut self, diagnostic_holder: &DiagnosticHolder) -> Vec<Token> {
        use unicode_segmentation::UnicodeSegmentation;

        let segmented_source =
            UnicodeSegmentation::graphemes(self.source, true).collect::<Vec<&str>>();
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
                _ if ("0".."9").contains(char) => {
                    let start = *&self.position;

                    while self.position < segmented_source.len()
                        && ("0".."9").contains(&segmented_source[self.position])
                    {
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

    fn print(&self) {
        print!(" ");
        print!("{}", self.literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Identifier,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Arrow,
    Tilde,
    VerticalBar,
    DoubleColon,
}
