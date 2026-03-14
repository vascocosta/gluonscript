#[derive(Clone, Debug)]
pub enum Token {
    Number(i64),
    String(String),
    Ident(String),
    Plus,
    Minus,
    Star,
    Greater,
    Smaller,
    Equals,
    If,
    While,
    Fn,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new<'a>(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                // ignore whitespace
                c if c.is_whitespace() => {
                    self.consume();
                }

                // numbers
                c if c.is_ascii_digit() => {
                    tokens.push(self.lex_number());
                }

                // identifiers / keywords
                c if c.is_ascii_alphabetic() => {
                    tokens.push(self.lex_identifier());
                }

                '+' => {
                    self.consume();
                    tokens.push(Token::Plus);
                }

                '-' => {
                    self.consume();
                    tokens.push(Token::Minus);
                }

                '*' => {
                    self.consume();
                    tokens.push(Token::Star);
                }

                '>' => {
                    self.consume();
                    tokens.push(Token::Greater);
                }

                '<' => {
                    self.consume();
                    tokens.push(Token::Smaller);
                }

                '=' => {
                    self.consume();
                    tokens.push(Token::Equals);
                }

                '{' => {
                    self.consume();
                    tokens.push(Token::LBrace);
                }

                '}' => {
                    self.consume();
                    tokens.push(Token::RBrace);
                }

                '(' => {
                    self.consume();
                    tokens.push(Token::LParen);
                }

                ')' => {
                    self.consume();
                    tokens.push(Token::RParen);
                }

                '"' => {
                    self.consume();
                    tokens.push(self.lex_string());
                }

                ',' => {
                    self.consume();
                    tokens.push(Token::Comma);
                }

                _ => panic!("Unexpected character: {}", c),
            }
        }

        tokens
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.consume();
            } else {
                break;
            }
        }

        let num: String = self.chars[start..self.pos].iter().collect();

        Token::Number(num.parse().expect("expected a number"))
    }

    fn lex_string(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else {
                self.consume();
            }
        }

        let string: String = self.chars[start..self.pos].iter().collect();

        match self.consume() {
            Some('"') => {}
            _ => panic!("unterminated string"),
        }

        Token::String(string)
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.consume();
            } else {
                break;
            }
        }

        let ident: String = self.chars[start..self.pos].iter().collect();

        match ident.as_str() {
            "if" => Token::If,
            "while" => Token::While,
            "fn" => Token::Fn,
            _ => Token::Ident(ident),
        }
    }
}
