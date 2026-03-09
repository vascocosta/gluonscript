use crate::eval::{Expr, Operator, Program, Stmt};
use crate::lexer::Token;

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

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            statements.push(self.parse_stmt());
        }

        Program { statements }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Ident(_)) => {
                if matches!(self.peek_next(), Some(Token::Equals)) {
                    self.parse_assign()
                } else {
                    Stmt::Expr(self.parse_expr(0))
                }
            }
            _ => Stmt::Expr(self.parse_expr(0)),
        }
    }

    fn parse_assign(&mut self) -> Stmt {
        let name = match self.consume() {
            Some(Token::Ident(n)) => n,
            _ => panic!("expected identifier"),
        };

        self.consume(); // Discard Token::Equals

        let value = self.parse_expr(0);

        Stmt::Assign { name, value }
    }

    fn parse_if(&mut self) -> Stmt {
        self.consume(); // Discard Token::If

        let condition = self.parse_expr(0);

        // Discard Token::LBrace
        match self.consume() {
            Some(Token::LBrace) => {}
            _ => panic!("expected {{"),
        }

        let mut then_branch = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            then_branch.push(self.parse_stmt());
        }

        self.consume(); // Discard Token::RBrace

        Stmt::If {
            condition,
            then_branch,
        }
    }

    fn parse_while(&mut self) -> Stmt {
        self.consume(); // while

        let condition = self.parse_expr(0);

        match self.consume() {
            Some(Token::LBrace) => {}
            _ => panic!("expected {{"),
        }

        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.parse_stmt());
        }

        self.consume(); // }

        Stmt::While { condition, body }
    }

    fn parse_expr(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_primary();

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

            let right = self.parse_expr(prec + 1);

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Some(Token::Number(n)) => Expr::Number(n),
            Some(Token::String(s)) => Expr::String(s),
            Some(Token::Ident(name)) => {
                // check if this is a function call
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.consume(); // consume '('

                    let mut args = Vec::new();

                    // parse arguments if any
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expr(0));

                            if matches!(self.peek(), Some(Token::Comma)) {
                                self.consume(); // consume comma
                            } else {
                                break;
                            }
                        }
                    }

                    match self.consume() {
                        Some(Token::RParen) => {}
                        _ => panic!("expected ')'"),
                    }

                    Expr::Call { name, args }
                } else {
                    Expr::Variable(name)
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr(0);

                match self.consume() {
                    Some(Token::RParen) => expr,
                    _ => panic!("expected ')'"),
                }
            }

            other => panic!("unexpected token: {:?}", other),
        }
    }

    fn token_to_operator(token: &Token) -> Option<Operator> {
        match token {
            Token::Plus => Some(Operator::Add),
            Token::Minus => Some(Operator::Sub),
            Token::Star => Some(Operator::Mul),
            Token::Greater => Some(Operator::Greater),
            Token::Smaller => Some(Operator::Smaller),
            _ => None,
        }
    }
}
