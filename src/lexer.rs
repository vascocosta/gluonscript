#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub pos: usize,
}

#[derive(Clone, Debug)]
pub enum Token {
    Int(i64),
    Float(f64),
    String(String),
    Ident(String),
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Star,
    StarEquals,
    Slash,
    SlashEquals,
    Percent,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
    Equals,
    EqualEqual,
    Not,
    NotEqual,
    PipePipe,
    Ampersand,
    If,
    In,
    Else,
    For,
    While,
    Fn,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Dot,
    Return,
    Break,
    Continue,
    True,
    False,
    Pipe,
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ScanError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                c if c.is_whitespace() => {
                    self.consume();
                }

                c if c.is_ascii_digit() => {
                    tokens.push(self.lex_number()?);
                }

                c if c.is_ascii_alphabetic() => {
                    tokens.push(self.lex_identifier());
                }

                '+' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::PlusEquals);
                    } else {
                        tokens.push(Token::Plus);
                    }
                }

                '-' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::MinusEquals);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }

                '*' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::StarEquals);
                    } else {
                        tokens.push(Token::Star);
                    }
                }

                '/' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::SlashEquals);
                    } else {
                        tokens.push(Token::Slash);
                    }
                }

                '%' => {
                    self.consume();
                    tokens.push(Token::Percent);
                }

                '>' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        tokens.push(Token::Greater);
                    }
                }

                '<' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::SmallerEqual);
                    } else {
                        tokens.push(Token::Smaller);
                    }
                }

                '=' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::EqualEqual);
                    } else {
                        tokens.push(Token::Equals)
                    }
                }

                '!' => {
                    self.consume();

                    if self.peek() == Some('=') {
                        self.consume();
                        tokens.push(Token::NotEqual);
                    } else {
                        tokens.push(Token::Not);
                    }
                }

                '|' => {
                    self.consume();

                    if self.peek() == Some('|') {
                        self.consume();
                        tokens.push(Token::PipePipe);
                    } else if self.peek() == Some('>') {
                        self.consume();
                        tokens.push(Token::Pipe);
                    } else {
                        return Err(ScanError {
                            message: format!("unexpected character {}", c),
                            pos: self.pos,
                        });
                    }
                }

                '&' => {
                    self.consume();

                    if self.peek() == Some('&') {
                        self.consume();
                        tokens.push(Token::Ampersand);
                    } else {
                        return Err(ScanError {
                            message: format!("unexpected character {}", c),
                            pos: self.pos,
                        });
                    }
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

                '[' => {
                    self.consume();
                    tokens.push(Token::LBracket);
                }

                ']' => {
                    self.consume();
                    tokens.push(Token::RBracket);
                }

                '"' => {
                    self.consume();
                    tokens.push(self.lex_string()?);
                }

                ',' => {
                    self.consume();
                    tokens.push(Token::Comma);
                }

                ':' => {
                    self.consume();
                    tokens.push(Token::Colon);
                }

                '.' => {
                    self.consume();
                    tokens.push(Token::Dot);
                }

                '#' => {
                    self.consume();

                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }

                        self.consume();
                    }
                }

                _ => {
                    return Err(ScanError {
                        message: format!("unexpected character {}", c),
                        pos: self.pos,
                    });
                }
            }
        }

        Ok(tokens)
    }

    fn lex_number(&mut self) -> Result<Token, ScanError> {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                self.consume();
            } else {
                break;
            }
        }

        let num: String = self.chars[start..self.pos].iter().collect();

        if num.contains(".") {
            Ok(Token::Float(num.parse().map_err(|_| ScanError {
                message: "expected a float number".to_string(),
                pos: self.pos,
            })?))
        } else {
            Ok(Token::Int(num.parse().map_err(|_| ScanError {
                message: "expected an int number".to_string(),
                pos: self.pos,
            })?))
        }
    }

    fn lex_string(&mut self) -> Result<Token, ScanError> {
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
            _ => {
                return Err(ScanError {
                    message: "unterminated string".to_string(),
                    pos: self.pos,
                });
            }
        }

        Ok(Token::String(string))
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
            "in" => Token::In,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        }
    }
}
