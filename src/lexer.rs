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

    pub fn lex(&mut self) -> Vec<Token> {
        use unicode_segmentation::UnicodeSegmentation;

        let segmented_source =
            UnicodeSegmentation::graphemes(self.source, true).collect::<Vec<&str>>();
        let mut tokens = Vec::<Token>::new();

        while self.position < segmented_source.len() {
            let char = &segmented_source[self.position];

            match *char {
                ":" => {
                    if self.offset(&segmented_source, 1) == ":" {
                        tokens.push(Token::new("::", Type::DoubleColon));
                        self.position += 2;
                    }
                }
                "-" => {
                    if self.offset(&segmented_source, 1) == ">" {
                        tokens.push(Token::new("->", Type::Arrow));
                        self.position += 2;
                    }
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

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum Type {
    Identifier,
    Number,
    Arrow,
    DoubleColon,
}
