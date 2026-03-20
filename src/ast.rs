use std::collections::HashMap;

use crate::operators::Operator;
use crate::program::RuntimeError;
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
    pub fn eval_expr(expr: &Expr, env: &Env) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Int(n) => Ok(Value::Int(*n)),
            Expr::Float(n) => Ok(Value::Float(*n)),
            Expr::String(s) => Ok(Value::String(s.to_owned())),
            Expr::Variable(name) => Ok(env.get_vars(name)?),
            Expr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, env)?;
                let r = Self::eval_expr(right, env)?;

                match (l, r, op) {
                    (Value::Int(a), Value::Int(b), Operator::Add) => Ok(Value::Int(a + b)),
                    (Value::Float(a), Value::Float(b), Operator::Add) => Ok(Value::Float(a + b)),
                    (Value::Int(a), Value::Float(b), Operator::Add) => {
                        Ok(Value::Float(a as f64 + b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Add) => {
                        Ok(Value::Float(a + b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Sub) => Ok(Value::Int(a - b)),
                    (Value::Float(a), Value::Float(b), Operator::Sub) => Ok(Value::Float(a - b)),
                    (Value::Int(a), Value::Float(b), Operator::Sub) => {
                        Ok(Value::Float(a as f64 - b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Sub) => {
                        Ok(Value::Float(a - b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Mul) => Ok(Value::Int(a * b)),
                    (Value::Float(a), Value::Float(b), Operator::Mul) => Ok(Value::Float(a * b)),
                    (Value::Int(a), Value::Float(b), Operator::Mul) => {
                        Ok(Value::Float(a as f64 * b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Mul) => {
                        Ok(Value::Float(a * b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Div) => Ok(Value::Int(a / b)),
                    (Value::Float(a), Value::Float(b), Operator::Div) => Ok(Value::Float(a / b)),
                    (Value::Int(a), Value::Float(b), Operator::Div) => {
                        Ok(Value::Float(a as f64 / b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Div) => {
                        Ok(Value::Float(a / b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Greater) => Ok(Value::Bool(a > b)),
                    (Value::Float(a), Value::Float(b), Operator::Greater) => Ok(Value::Bool(a > b)),
                    (Value::Int(a), Value::Float(b), Operator::Greater) => {
                        Ok(Value::Bool(a as f64 > b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Greater) => {
                        Ok(Value::Bool(a > b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Smaller) => Ok(Value::Bool(a < b)),
                    (Value::Float(a), Value::Float(b), Operator::Smaller) => Ok(Value::Bool(a < b)),
                    (Value::Int(a), Value::Float(b), Operator::Smaller) => {
                        Ok(Value::Bool((a as f64) < b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Smaller) => {
                        Ok(Value::Bool(a < b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::GreaterEqual) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (Value::Float(a), Value::Float(b), Operator::GreaterEqual) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (Value::Int(a), Value::Float(b), Operator::GreaterEqual) => {
                        Ok(Value::Bool(a as f64 >= b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::GreaterEqual) => {
                        Ok(Value::Bool(a >= b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::SmallerEqual) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (Value::Float(a), Value::Float(b), Operator::SmallerEqual) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (Value::Int(a), Value::Float(b), Operator::SmallerEqual) => {
                        Ok(Value::Bool(a as f64 <= b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::SmallerEqual) => {
                        Ok(Value::Bool(a <= b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::Percent) => Ok(Value::Int(a % b)),
                    (Value::Float(a), Value::Float(b), Operator::Percent) => {
                        Ok(Value::Float(a % b))
                    }
                    (Value::Int(a), Value::Float(b), Operator::Percent) => {
                        Ok(Value::Float(a as f64 % b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Percent) => {
                        Ok(Value::Float(a % b as f64))
                    }

                    (Value::Int(a), Value::Int(b), Operator::EqualEqual) => Ok(Value::Bool(a == b)),
                    (Value::Float(a), Value::Float(b), Operator::EqualEqual) => {
                        Ok(Value::Bool(a == b))
                    }
                    (Value::Int(a), Value::Float(b), Operator::EqualEqual) => {
                        Ok(Value::Bool(a as f64 == b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::EqualEqual) => {
                        Ok(Value::Bool(a == b as f64))
                    }
                    (Value::String(a), Value::String(b), Operator::EqualEqual) => {
                        Ok(Value::Bool(a == b))
                    }

                    (Value::Int(a), Value::Int(b), Operator::NotEqual) => Ok(Value::Bool(a != b)),
                    (Value::Float(a), Value::Float(b), Operator::NotEqual) => {
                        Ok(Value::Bool(a != b))
                    }
                    (Value::Int(a), Value::Float(b), Operator::NotEqual) => {
                        Ok(Value::Bool(a as f64 != b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::NotEqual) => {
                        Ok(Value::Bool(a != b as f64))
                    }
                    (Value::String(a), Value::String(b), Operator::NotEqual) => {
                        Ok(Value::Bool(a != b))
                    }

                    (Value::String(a), Value::String(b), Operator::Add) => {
                        Ok(Value::String(format!("{}{}", a, b)))
                    }

                    (Value::Bool(a), Value::Bool(b), Operator::Or) => Ok(Value::Bool(a || b)),
                    (Value::Bool(a), Value::Bool(b), Operator::And) => Ok(Value::Bool(a && b)),

                    _ => Err(RuntimeError {
                        message: "type error".to_string(),
                    }),
                }
            }
            Expr::Call { name, args } => {
                let values: Result<Vec<Value>, RuntimeError> =
                    args.iter().map(|a| Self::eval_expr(a, env)).collect();

                if let Some(func) = env.get_functions(name) {
                    let mut local_env = env.child();

                    for (param, value) in func.params.iter().zip(values?) {
                        local_env.set(param.clone(), value);
                    }

                    let mut result = Value::Bool(false);

                    for stmt in &func.body {
                        match Program::exec_stmt(stmt, &mut local_env)? {
                            ExecResult::Continue => {}
                            ExecResult::Break => {}
                            ExecResult::LoopContinue => {}
                            ExecResult::Return(v) => return Ok(v),
                            ExecResult::Value(v) => result = v,
                        }
                    }

                    Ok(result)
                } else {
                    Ok(builtin::call_builtin(name, &values?))
                }
            }
            Expr::ListLiteral(elements) => {
                let values: Result<Vec<Value>, RuntimeError> =
                    elements.iter().map(|e| Self::eval_expr(e, env)).collect();

                Ok(Value::List(values?))
            }
            Expr::Index { target, index } => {
                let list = Self::eval_expr(target, env)?;
                let idx = Self::eval_expr(index, env)?;

                match (list, idx) {
                    (Value::List(v), Value::Int(i)) => {
                        Ok(v.get(i as usize).cloned().ok_or(RuntimeError {
                            message: "index out of bounds".to_string(),
                        })?)
                    }

                    _ => Err(RuntimeError {
                        message: "invalid indexing".to_string(),
                    }),
                }
            }
            Expr::RecordLiteral(fields) => {
                let mut map = HashMap::new();

                for (k, v) in fields {
                    map.insert(k.clone(), Self::eval_expr(v, env)?);
                }

                Ok(Value::Record(map))
            }
            Expr::Propery { target, name } => {
                let record = Self::eval_expr(target, env)?;

                match record {
                    Value::Record(map) => Ok(map.get(name).cloned().ok_or(RuntimeError {
                        message: format!("unknown property: {}", name),
                    })?),
                    _ => Err(RuntimeError {
                        message: "not a record".to_string(),
                    }),
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
    Break,
    Continue,
}

pub enum ExecResult {
    Continue,
    Break,
    LoopContinue,
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

    fn get_vars(&self, name: &str) -> Result<Value, RuntimeError> {
        if let Some(v) = self.vars.get(name) {
            return Ok(v.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get_vars(name);
        }

        Err(RuntimeError {
            message: format!("undefined variable: {}", name),
        })
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
