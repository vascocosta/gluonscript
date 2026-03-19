use std::collections::HashMap;

use crate::operators::Operator;
use crate::{builtin, program::Program};

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
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
    RecordLiteral(Vec<(String, Expr)>),
    Propery {
        target: Box<Expr>,
        name: String,
    },
}

impl Expr {
    pub fn eval_expr(expr: &Expr, env: &Env) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(*n),
            Expr::Float(n) => Value::Float(*n),
            Expr::String(s) => Value::String(s.to_owned()),
            Expr::Variable(name) => env.get_vars(name),
            Expr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, env);
                let r = Self::eval_expr(right, env);

                match (l, r, op) {
                    (Value::Int(a), Value::Int(b), Operator::Add) => Value::Int(a + b),
                    (Value::Float(a), Value::Float(b), Operator::Add) => Value::Float(a + b),
                    (Value::Int(a), Value::Float(b), Operator::Add) => Value::Float(a as f64 + b),
                    (Value::Float(a), Value::Int(b), Operator::Add) => Value::Float(a + b as f64),

                    (Value::Int(a), Value::Int(b), Operator::Sub) => Value::Int(a - b),
                    (Value::Float(a), Value::Float(b), Operator::Sub) => Value::Float(a - b),
                    (Value::Int(a), Value::Float(b), Operator::Sub) => Value::Float(a as f64 - b),
                    (Value::Float(a), Value::Int(b), Operator::Sub) => Value::Float(a - b as f64),

                    (Value::Int(a), Value::Int(b), Operator::Mul) => Value::Int(a * b),
                    (Value::Float(a), Value::Float(b), Operator::Mul) => Value::Float(a * b),
                    (Value::Int(a), Value::Float(b), Operator::Mul) => Value::Float(a as f64 * b),
                    (Value::Float(a), Value::Int(b), Operator::Mul) => Value::Float(a * b as f64),

                    (Value::Int(a), Value::Int(b), Operator::Div) => Value::Int(a / b),
                    (Value::Float(a), Value::Float(b), Operator::Div) => Value::Float(a / b),
                    (Value::Int(a), Value::Float(b), Operator::Div) => Value::Float(a as f64 / b),
                    (Value::Float(a), Value::Int(b), Operator::Div) => Value::Float(a / b as f64),

                    (Value::Int(a), Value::Int(b), Operator::Greater) => Value::Bool(a > b),
                    (Value::Float(a), Value::Float(b), Operator::Greater) => Value::Bool(a > b),
                    (Value::Int(a), Value::Float(b), Operator::Greater) => {
                        Value::Bool(a as f64 > b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::Greater) => {
                        Value::Bool(a > b as f64)
                    }

                    (Value::Int(a), Value::Int(b), Operator::Smaller) => Value::Bool(a < b),
                    (Value::Float(a), Value::Float(b), Operator::Smaller) => Value::Bool(a < b),
                    (Value::Int(a), Value::Float(b), Operator::Smaller) => {
                        Value::Bool((a as f64) < b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::Smaller) => {
                        Value::Bool(a < b as f64)
                    }

                    (Value::Int(a), Value::Int(b), Operator::GreaterEqual) => Value::Bool(a >= b),
                    (Value::Float(a), Value::Float(b), Operator::GreaterEqual) => {
                        Value::Bool(a >= b)
                    }
                    (Value::Int(a), Value::Float(b), Operator::GreaterEqual) => {
                        Value::Bool(a as f64 >= b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::GreaterEqual) => {
                        Value::Bool(a >= b as f64)
                    }

                    (Value::Int(a), Value::Int(b), Operator::SmallerEqual) => Value::Bool(a <= b),
                    (Value::Float(a), Value::Float(b), Operator::SmallerEqual) => {
                        Value::Bool(a <= b)
                    }
                    (Value::Int(a), Value::Float(b), Operator::SmallerEqual) => {
                        Value::Bool(a as f64 <= b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::SmallerEqual) => {
                        Value::Bool(a <= b as f64)
                    }

                    (Value::Int(a), Value::Int(b), Operator::Percent) => Value::Int(a % b),
                    (Value::Float(a), Value::Float(b), Operator::Percent) => Value::Float(a % b),
                    (Value::Int(a), Value::Float(b), Operator::Percent) => {
                        Value::Float(a as f64 % b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::Percent) => {
                        Value::Float(a % b as f64)
                    }

                    (Value::Int(a), Value::Int(b), Operator::EqualEqual) => Value::Bool(a == b),
                    (Value::Float(a), Value::Float(b), Operator::EqualEqual) => Value::Bool(a == b),
                    (Value::Int(a), Value::Float(b), Operator::EqualEqual) => {
                        Value::Bool(a as f64 == b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::EqualEqual) => {
                        Value::Bool(a == b as f64)
                    }
                    (Value::String(a), Value::String(b), Operator::EqualEqual) => {
                        Value::Bool(a == b)
                    }

                    (Value::Int(a), Value::Int(b), Operator::NotEqual) => Value::Bool(a != b),
                    (Value::Float(a), Value::Float(b), Operator::NotEqual) => Value::Bool(a != b),
                    (Value::Int(a), Value::Float(b), Operator::NotEqual) => {
                        Value::Bool(a as f64 != b)
                    }
                    (Value::Float(a), Value::Int(b), Operator::NotEqual) => {
                        Value::Bool(a != b as f64)
                    }
                    (Value::String(a), Value::String(b), Operator::NotEqual) => Value::Bool(a != b),

                    (Value::String(a), Value::String(b), Operator::Add) => {
                        Value::String(format!("{}{}", a, b))
                    }

                    (Value::Bool(a), Value::Bool(b), Operator::Or) => Value::Bool(a || b),
                    (Value::Bool(a), Value::Bool(b), Operator::And) => Value::Bool(a && b),

                    _ => panic!("type error"),
                }
            }
            Expr::Call { name, args } => {
                let values: Vec<Value> = args.iter().map(|a| Self::eval_expr(a, env)).collect();

                if let Some(func) = env.get_functions(name) {
                    let mut local_env = env.child();

                    for (param, value) in func.params.iter().zip(values) {
                        local_env.set(param.clone(), value);
                    }

                    let mut result = Value::Bool(false);

                    for stmt in &func.body {
                        match Program::exec_stmt(stmt, &mut local_env) {
                            ExecResult::Continue => {}
                            ExecResult::Return(v) => return v,
                            ExecResult::Value(v) => result = v,
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
                    (Value::List(v), Value::Int(i)) => {
                        v.get(i as usize).cloned().expect("index out of bounds")
                    }

                    _ => panic!("invalid indexing"),
                }
            }
            Expr::RecordLiteral(fields) => {
                let mut map = HashMap::new();

                for (k, v) in fields {
                    map.insert(k.clone(), Self::eval_expr(v, env));
                }

                Value::Record(map)
            }
            Expr::Propery { target, name } => {
                let record = Self::eval_expr(target, env);

                match record {
                    Value::Record(map) => map
                        .get(name)
                        .cloned()
                        .unwrap_or_else(|| panic!("unknown property {}", name)),
                    _ => panic!("not a record"),
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
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
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
    Return(Expr),
}

pub enum ExecResult {
    Continue,
    Return(Value),
    Value(Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Record(HashMap<String, Value>),
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
            functions: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    fn get_vars(&self, name: &str) -> Value {
        if let Some(v) = self.vars.get(name) {
            return v.clone();
        }

        if let Some(parent) = &self.parent {
            return parent.get_vars(name);
        }

        panic!("undefined variable: {}", name)
    }

    fn get_functions(&self, name: &str) -> Option<Function> {
        if let Some(f) = self.functions.get(name) {
            return Some(f.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get_functions(name);
        }

        None
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}
