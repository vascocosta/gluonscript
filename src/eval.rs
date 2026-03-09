use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

impl Expr {
    fn eval_expr(expr: &Expr, env: &Env) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::Variable(name) => env.vars.get(name).expect("undefined variable").clone(),
            Expr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, env);
                let r = Self::eval_expr(right, env);

                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Operator::Add) => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), Operator::Sub) => Value::Number(a - b),
                    (Value::Number(a), Value::Number(b), Operator::Mul) => Value::Number(a * b),
                    (Value::Number(a), Value::Number(b), Operator::Greater) => Value::Bool(a > b),
                    (Value::Number(a), Value::Number(b), Operator::Smaller) => Value::Bool(a < b),
                    _ => panic!("type error"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
    Sub,
    Greater,
    Smaller,
}

impl Operator {
    pub fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Greater => 5,
            Operator::Smaller => 5,
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
}

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
        match stmt {
            Stmt::Assign { name, value } => {
                let v = Expr::eval_expr(value, env);
                env.vars.insert(name.clone(), v);
                None
            }

            Stmt::Expr(expr) => Some(Expr::eval_expr(expr, env)),

            Stmt::If {
                condition,
                then_branch,
            } => {
                let cond = Expr::eval_expr(condition, env);

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        Self::exec_stmt(stmt, env);
                    }
                }

                None
            }

            Stmt::While { condition, body } => {
                loop {
                    let cond = Expr::eval_expr(condition, env);

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body {
                                Self::exec_stmt(stmt, env);
                            }
                        }

                        Value::Bool(false) => break,

                        _ => panic!("while condition must be bool"),
                    }
                }

                None
            }
        }
    }

    pub fn run(&self) -> Option<Value> {
        let mut env = Env::new();
        let mut last = None;

        for stmt in &self.statements {
            if let Some(v) = Self::exec_stmt(stmt, &mut env) {
                last = Some(v);
            }
        }

        last
    }
}

struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}
