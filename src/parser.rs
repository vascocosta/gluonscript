use crate::ast::{Expr, Stmt};
use crate::lexer::Token;
use crate::operators::Operator;
use crate::program::Program;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub pos: usize,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn consume(&mut self) -> Option<Token> {
        let t = self.peek().cloned();
        self.pos += 1;

        t
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            statements.push(self.parse_stmt()?);
        }

        Ok(Program { statements })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::For) => self.parse_for(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::Break) => self.parse_break(),
            Some(Token::Continue) => self.parse_continue(),

            Some(Token::Ident(_)) => match self.peek_next() {
                Some(Token::Equals) => self.parse_assign(),
                Some(Token::QuestionEquals) => self.parse_question_assign(),
                Some(Token::PlusEquals) => self.parse_op_assign(Operator::Add),
                Some(Token::MinusEquals) => self.parse_op_assign(Operator::Sub),
                Some(Token::StarEquals) => self.parse_op_assign(Operator::Mul),
                Some(Token::SlashEquals) => self.parse_op_assign(Operator::Div),
                _ => Ok(Stmt::Expr(self.parse_expr(0)?)),
            },

            Some(Token::Fn) => self.parse_function(),
            _ => Ok(Stmt::Expr(self.parse_expr(0)?)),
        }
    }

    fn parse_assign(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.consume() {
            Some(Token::Ident(n)) => n,

            _ => {
                return Err(ParseError {
                    message: "expected identifier".to_string(),
                    pos: self.pos,
                });
            }
        };

        self.consume(); // Discard Token::Equals

        let value = self.parse_expr(0)?;

        Ok(Stmt::Assign { name, value })
    }

    fn parse_question_assign(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.consume() {
            Some(Token::Ident(n)) => n,

            _ => {
                return Err(ParseError {
                    message: "expected identifier".to_string(),
                    pos: self.pos,
                });
            }
        };

        self.consume();

        let value = self.parse_expr(0)?;

        Ok(Stmt::TryAssign { name, value })
    }

    fn parse_op_assign(&mut self, op: Operator) -> Result<Stmt, ParseError> {
        let name = match self.consume() {
            Some(Token::Ident(n)) => n,

            _ => {
                return Err(ParseError {
                    message: "expected identifier".to_string(),
                    pos: self.pos,
                });
            }
        };

        self.consume();

        let value = self.parse_expr(0)?;

        Ok(Stmt::Assign {
            name: name.clone(),
            value: Expr::Binary {
                left: Box::new(Expr::Variable(name)),
                op,
                right: Box::new(value),
            },
        })
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.consume(); // Discard Token::If

        let condition = self.parse_expr(0)?;

        // Discard Token::LBrace
        match self.consume() {
            Some(Token::LBrace) => {}

            _ => {
                return Err(ParseError {
                    message: "expected {{".to_string(),
                    pos: self.pos,
                });
            }
        }

        let mut then_branch = Vec::new();
        let mut else_branch = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            then_branch.push(self.parse_stmt()?);
        }

        self.consume();

        if matches!(self.peek(), Some(Token::Else)) {
            self.consume();

            if matches!(self.peek(), Some(Token::If)) {
                let else_if_stmt = self.parse_if()?;

                else_branch.push(else_if_stmt);
            } else {
                match self.consume() {
                    Some(Token::LBrace) => {}

                    _ => {
                        return Err(ParseError {
                            message: "expected {{".to_string(),
                            pos: self.pos,
                        });
                    }
                }

                while !matches!(self.peek(), Some(Token::RBrace)) {
                    else_branch.push(self.parse_stmt()?);
                }

                self.consume();
            }
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        let condition = self.parse_expr(0)?;

        match self.consume() {
            Some(Token::LBrace) => {}

            _ => {
                return Err(ParseError {
                    message: "expected {{".to_string(),
                    pos: self.pos,
                });
            }
        }

        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.parse_stmt()?);
        }

        self.consume(); // }

        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        let var = match self.consume() {
            Some(Token::Ident(n)) => n,

            _ => {
                return Err(ParseError {
                    message: "expected variable name".to_string(),
                    pos: self.pos,
                });
            }
        };

        match self.consume() {
            Some(Token::In) => {}

            _ => {
                return Err(ParseError {
                    message: "expected 'in'".to_string(),
                    pos: self.pos,
                });
            }
        }

        let iterable = self.parse_expr(0)?;

        match self.consume() {
            Some(Token::LBrace) => {}

            _ => {
                return Err(ParseError {
                    message: "expected {{identifier}}".to_string(),
                    pos: self.pos,
                });
            }
        }

        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.parse_stmt()?);
        }

        self.consume();

        Ok(Stmt::For {
            var,
            iterable,
            body,
        })
    }

    fn parse_function(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        let name = match self.consume() {
            Some(Token::Ident(n)) => n,

            _ => {
                return Err(ParseError {
                    message: "expected function name".to_string(),
                    pos: self.pos,
                });
            }
        };

        match self.consume() {
            Some(Token::LParen) => {}

            _ => {
                return Err(ParseError {
                    message: "expected '('".to_string(),
                    pos: self.pos,
                });
            }
        }

        let mut params = Vec::new();

        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                match self.consume() {
                    Some(Token::Ident(p)) => params.push(p),

                    _ => {
                        return Err(ParseError {
                            message: "expected parameter name".to_string(),
                            pos: self.pos,
                        });
                    }
                }

                if matches!(self.peek(), Some(Token::Comma)) {
                    self.consume();
                } else {
                    break;
                }
            }
        }

        match self.consume() {
            Some(Token::RParen) => {}

            _ => {
                return Err(ParseError {
                    message: "expected ')'".to_string(),
                    pos: self.pos,
                });
            }
        }

        match self.consume() {
            Some(Token::LBrace) => {}

            _ => {
                return Err(ParseError {
                    message: "expected {{".to_string(),
                    pos: self.pos,
                });
            }
        }

        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.parse_stmt()?);
        }

        self.consume();

        Ok(Stmt::Function { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        let expr = self.parse_expr(0)?;

        Ok(Stmt::Return(expr))
    }

    fn parse_break(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> Result<Stmt, ParseError> {
        self.consume();

        Ok(Stmt::Continue)
    }

    fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary()?;

        loop {
            match self.peek() {
                Some(Token::LBracket) => {
                    self.consume(); // [

                    let index = self.parse_expr(0)?;

                    match self.consume() {
                        Some(Token::RBracket) => {}

                        _ => {
                            return Err(ParseError {
                                message: "expected ']'".to_string(),
                                pos: self.pos,
                            });
                        }
                    }

                    left = Expr::Index {
                        target: Box::new(left),
                        index: Box::new(index),
                    };
                }

                Some(Token::Dot) => {
                    self.consume();

                    let name = match self.consume() {
                        Some(Token::Ident(n)) => n,

                        _ => {
                            return Err(ParseError {
                                message: "expected property name".to_string(),
                                pos: self.pos,
                            });
                        }
                    };

                    left = Expr::Property {
                        target: Box::new(left),
                        name,
                    };
                }

                Some(Token::LParen) => {
                    self.consume(); // (

                    let mut args = Vec::new();

                    if !matches!(self.peek(), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expr(0)?);

                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.consume();
                            } else {
                                break;
                            }
                        }
                    }

                    match self.consume() {
                        Some(Token::RParen) => {}

                        _ => {
                            return Err(ParseError {
                                message: "expected ')'".to_string(),
                                pos: self.pos,
                            });
                        }
                    }

                    left = Expr::Call {
                        callee: Box::new(left),
                        args,
                    };
                }

                _ => break,
            }
        }

        loop {
            let op = match self.peek().and_then(Self::token_to_operator) {
                Some(op) => op,
                None => break,
            };

            let prec = Operator::precedence(&op);

            if prec < min_prec {
                break;
            }

            self.consume();

            let right = self.parse_expr(prec + 1)?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.peek(), Some(Token::Minus)) {
            self.consume();

            let expr = self.parse_primary()?;

            return Ok(Expr::Binary {
                left: Box::new(Expr::Int(0)),
                op: Operator::Sub,
                right: Box::new(expr),
            });
        }

        match self.consume() {
            Some(Token::Int(n)) => Ok(Expr::Int(n)),
            Some(Token::Float(n)) => Ok(Expr::Float(n)),
            Some(Token::String(s)) => Ok(Expr::String(s)),
            Some(Token::True) => Ok(Expr::Bool(true)),
            Some(Token::False) => Ok(Expr::Bool(false)),

            Some(Token::Ident(name)) => {
                // check if this is a function call
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.consume(); // consume '('

                    let mut args = Vec::new();

                    // parse arguments if any
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expr(0)?);

                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.consume(); // consume comma
                            } else {
                                break;
                            }
                        }
                    }

                    match self.consume() {
                        Some(Token::RParen) => {}

                        _ => {
                            return Err(ParseError {
                                message: "expected ')'".to_string(),
                                pos: self.pos,
                            });
                        }
                    }

                    Ok(Expr::Call {
                        callee: Box::new(Expr::Variable(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Variable(name))
                }
            }

            Some(Token::LParen) => {
                let expr = self.parse_expr(0);

                match self.consume() {
                    Some(Token::RParen) => expr,
                    _ => {
                        return Err(ParseError {
                            message: "expected ')'".to_string(),
                            pos: self.pos,
                        });
                    }
                }
            }

            Some(Token::LBracket) => {
                let mut elements = Vec::new();

                if !matches!(self.peek(), Some(Token::RBracket)) {
                    loop {
                        elements.push(self.parse_expr(0)?);

                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                }

                match self.consume() {
                    Some(Token::RBracket) => {}

                    _ => {
                        return Err(ParseError {
                            message: "expected ']'".to_string(),
                            pos: self.pos,
                        });
                    }
                }

                Ok(Expr::ListLiteral(elements))
            }

            Some(Token::LBrace) => {
                let mut fields = Vec::new();

                if !matches!(self.peek(), Some(Token::RBrace)) {
                    loop {
                        let key = match self.consume() {
                            Some(Token::Ident(name)) => name,

                            _ => {
                                return Err(ParseError {
                                    message: "expected key".to_string(),
                                    pos: self.pos,
                                });
                            }
                        };

                        match self.consume() {
                            Some(Token::Colon) => {}

                            _ => {
                                return Err(ParseError {
                                    message: "expected ':'".to_string(),
                                    pos: self.pos,
                                });
                            }
                        }

                        let value = self.parse_expr(0)?;

                        fields.push((key, value));

                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                }

                match self.consume() {
                    Some(Token::RBrace) => {}

                    _ => {
                        return Err(ParseError {
                            message: "expected }}".to_string(),
                            pos: self.pos,
                        });
                    }
                }

                Ok(Expr::RecordLiteral(fields))
            }

            Some(Token::Fn) => {
                match self.consume() {
                    Some(Token::LParen) => {}
                    _ => panic!("expected '('"),
                }

                let mut params = Vec::new();

                if !matches!(self.peek(), Some(Token::RParen)) {
                    loop {
                        match self.consume() {
                            Some(Token::Ident(p)) => params.push(p),
                            _ => panic!("expected parameter name"),
                        }

                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                }

                match self.consume() {
                    Some(Token::RParen) => {}
                    _ => panic!("expected ')'"),
                }

                match self.consume() {
                    Some(Token::LBrace) => {}
                    _ => panic!("expected {{"),
                }

                let mut body = Vec::new();

                while !matches!(self.peek(), Some(Token::RBrace)) {
                    body.push(self.parse_stmt()?);
                }

                self.consume();

                Ok(Expr::Lambda { params, body })
            }

            other => {
                let message = if let Some(token) = other {
                    format!("unexpected token {:?}", token)
                } else {
                    "unexpected token".to_string()
                };

                return Err(ParseError {
                    message,
                    pos: self.pos,
                });
            }
        }
    }

    fn token_to_operator(token: &Token) -> Option<Operator> {
        match token {
            Token::Plus => Some(Operator::Add),
            Token::Minus => Some(Operator::Sub),
            Token::Star => Some(Operator::Mul),
            Token::Slash => Some(Operator::Div),
            Token::Greater => Some(Operator::Greater),
            Token::GreaterEqual => Some(Operator::GreaterEqual),
            Token::Smaller => Some(Operator::Smaller),
            Token::SmallerEqual => Some(Operator::SmallerEqual),
            Token::Percent => Some(Operator::Percent),
            Token::EqualEqual => Some(Operator::EqualEqual),
            Token::NotEqual => Some(Operator::NotEqual),
            Token::PipePipe => Some(Operator::Or),
            Token::Ampersand => Some(Operator::And),
            Token::Pipe => Some(Operator::Pipe),
            _ => None,
        }
    }
}
