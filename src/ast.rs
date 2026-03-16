use std::collections::HashMap;

use crate::{builtin, program::Program};

#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    ListLiteral(Vec<Expr>),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}

impl Expr {
    pub fn eval_expr(expr: &Expr, env: &Env) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s) => Value::String(s.to_owned()),
            Expr::Variable(name) => env.get(name),
            Expr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, env);
                let r = Self::eval_expr(right, env);

                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Operator::Add) => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), Operator::Sub) => Value::Number(a - b),
                    (Value::Number(a), Value::Number(b), Operator::Mul) => Value::Number(a * b),
                    (Value::Number(a), Value::Number(b), Operator::Greater) => Value::Bool(a > b),
                    (Value::Number(a), Value::Number(b), Operator::Smaller) => Value::Bool(a < b),
                    (Value::Number(a), Value::Number(b), Operator::GreaterEqual) => {
                        Value::Bool(a >= b)
                    }
                    (Value::Number(a), Value::Number(b), Operator::SmallerEqual) => {
                        Value::Bool(a <= b)
                    }
                    (Value::Number(a), Value::Number(b), Operator::Percent) => Value::Number(a % b),
                    (Value::Number(a), Value::Number(b), Operator::Equal) => Value::Bool(a == b),
                    (Value::String(a), Value::String(b), Operator::Add) => {
                        Value::String(format!("{}{}", a, b))
                    }
                    _ => panic!("type error"),
                }
            }
            Expr::Call { name, args } => {
                let values: Vec<Value> = args.iter().map(|a| Self::eval_expr(a, env)).collect();

                if let Some(func) = env.functions.get(name) {
                    let mut local_env = env.child();

                    for (param, value) in func.params.iter().zip(values) {
                        local_env.set(param.clone(), value);
                    }

                    let mut result = Value::Bool(false);

                    for stmt in &func.body {
                        if let Some(v) = Program::exec_stmt(stmt, &mut local_env) {
                            result = v;
                        }
                    }

                    result
                } else {
                    builtin::call_builtin(name, &values)
                }
            }
            Expr::ListLiteral(elements) => {
                let values = elements.iter().map(|e| Self::eval_expr(e, env)).collect();

                Value::List(values)
            }
            Expr::Index { target, index } => {
                let list = Self::eval_expr(target, env);
                let idx = Self::eval_expr(index, env);

                match (list, idx) {
                    (Value::List(v), Value::Number(i)) => {
                        v.get(i as usize).cloned().expect("index out of bounds")
                    }

                    _ => panic!("invalid indexing"),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
    Sub,
    Percent,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
    Equal,
}

impl Operator {
    pub fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Greater => 5,
            Operator::GreaterEqual => 5,
            Operator::Smaller => 5,
            Operator::SmallerEqual => 5,
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
            Operator::Percent => 10,
            Operator::Equal => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct Env {
    pub vars: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    fn child(&self) -> Self {
        Self {
            vars: HashMap::new(),
            functions: self.functions.clone(),
            parent: Some(Box::new(self.clone())),
        }
    }

    fn get(&self, name: &str) -> Value {
        if let Some(v) = self.vars.get(name) {
            return v.clone();
        }

        if let Some(parent) = &self.parent {
            if let Some(v) = parent.vars.get(name) {
                return v.clone();
            }
        }

        panic!("undefined variable: {}", name)
    }

    fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}
