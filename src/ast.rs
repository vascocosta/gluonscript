use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::operators::Operator;
use crate::runtime::{Env, Function, RuntimeError, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Variable(String),

    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    ListLiteral(Vec<Expr>),

    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },

    RecordLiteral(Vec<(String, Expr)>),

    Property {
        target: Box<Expr>,
        name: String,
    },

    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

impl Expr {
    pub fn eval(&self, env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
        match self {
            Expr::Int(n) => Ok(Value::Int(*n)),
            Expr::Float(n) => Ok(Value::Float(*n)),
            Expr::String(s) => Ok(Value::String(s.to_owned())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Variable(name) => {
                Ok(env
                    .borrow()
                    .get_vars(name)
                    .ok_or(RuntimeError::RichMessage(format!(
                        "undefined variable: {}",
                        name
                    )))?)
            }

            Expr::Binary { left, op, right } => {
                if let Operator::Pipe = op {
                    let l = left.eval(env.clone())?;

                    match &**right {
                        Expr::Call { callee, args } => {
                            let mut values = vec![l];

                            for arg in args {
                                values.push(arg.eval(env.clone())?);
                            }

                            let func_val = callee.eval(env)?;

                            return func_val.call(values);
                        }

                        _ => return right.eval(env)?.call(vec![l]),
                    }
                }

                let l = left.eval(env.clone())?;
                let r = right.eval(env.clone())?;

                match (l, r, op) {
                    // Operator::Add
                    (Value::Int(a), Value::Int(b), Operator::Add) => Ok(Value::Int(a + b)),
                    (Value::Float(a), Value::Float(b), Operator::Add) => Ok(Value::Float(a + b)),

                    (Value::Int(a), Value::Float(b), Operator::Add) => {
                        Ok(Value::Float(a as f64 + b))
                    }

                    (Value::Float(a), Value::Int(b), Operator::Add) => {
                        Ok(Value::Float(a + b as f64))
                    }

                    (Value::String(a), Value::String(b), Operator::Add) => {
                        Ok(Value::String(format!("{}{}", a, b)))
                    }

                    // Operator::Sub
                    (Value::Int(a), Value::Int(b), Operator::Sub) => Ok(Value::Int(a - b)),
                    (Value::Float(a), Value::Float(b), Operator::Sub) => Ok(Value::Float(a - b)),

                    (Value::Int(a), Value::Float(b), Operator::Sub) => {
                        Ok(Value::Float(a as f64 - b))
                    }

                    (Value::Float(a), Value::Int(b), Operator::Sub) => {
                        Ok(Value::Float(a - b as f64))
                    }

                    // Operator::Mul
                    (Value::Int(a), Value::Int(b), Operator::Mul) => Ok(Value::Int(a * b)),
                    (Value::Float(a), Value::Float(b), Operator::Mul) => Ok(Value::Float(a * b)),

                    (Value::Int(a), Value::Float(b), Operator::Mul) => {
                        Ok(Value::Float(a as f64 * b))
                    }
                    (Value::Float(a), Value::Int(b), Operator::Mul) => {
                        Ok(Value::Float(a * b as f64))
                    }

                    // Operator::Div
                    (Value::Int(a), Value::Int(b), Operator::Div) => Ok(Value::Int(a / b)),
                    (Value::Float(a), Value::Float(b), Operator::Div) => Ok(Value::Float(a / b)),

                    (Value::Int(a), Value::Float(b), Operator::Div) => {
                        Ok(Value::Float(a as f64 / b))
                    }

                    (Value::Float(a), Value::Int(b), Operator::Div) => {
                        Ok(Value::Float(a / b as f64))
                    }

                    // Operator::Greater
                    (Value::Int(a), Value::Int(b), Operator::Greater) => Ok(Value::Bool(a > b)),
                    (Value::Float(a), Value::Float(b), Operator::Greater) => Ok(Value::Bool(a > b)),

                    (Value::Int(a), Value::Float(b), Operator::Greater) => {
                        Ok(Value::Bool(a as f64 > b))
                    }

                    (Value::Float(a), Value::Int(b), Operator::Greater) => {
                        Ok(Value::Bool(a > b as f64))
                    }

                    // Operator::Smaller
                    (Value::Int(a), Value::Int(b), Operator::Smaller) => Ok(Value::Bool(a < b)),
                    (Value::Float(a), Value::Float(b), Operator::Smaller) => Ok(Value::Bool(a < b)),

                    (Value::Int(a), Value::Float(b), Operator::Smaller) => {
                        Ok(Value::Bool((a as f64) < b))
                    }

                    (Value::Float(a), Value::Int(b), Operator::Smaller) => {
                        Ok(Value::Bool(a < b as f64))
                    }

                    // Operator::GreaterEqual
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

                    // Operator::SmallerEqual
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

                    // Operator::Percent
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

                    // Operator::EqualEqual
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

                    (Value::Bool(a), Value::Bool(b), Operator::EqualEqual) => {
                        Ok(Value::Bool(a == b))
                    }

                    // Operator::NotEqual
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

                    (Value::Bool(a), Value::Bool(b), Operator::NotEqual) => Ok(Value::Bool(a != b)),

                    // Operator::Or
                    (Value::Bool(a), Value::Bool(b), Operator::Or) => Ok(Value::Bool(a || b)),

                    // Operator::And
                    (Value::Bool(a), Value::Bool(b), Operator::And) => Ok(Value::Bool(a && b)),

                    _ => Err(RuntimeError::Message("unsupported operation")),
                }
            }

            Expr::Call { callee, args } => {
                let args: Result<Vec<Value>, RuntimeError> =
                    args.iter().map(|a| a.eval(env.clone())).collect();

                let func_val = callee.eval(env.clone())?;

                func_val.call(args?)
            }

            Expr::ListLiteral(elements) => {
                let values: Result<Vec<Value>, RuntimeError> =
                    elements.iter().map(|e| e.eval(env.clone())).collect();

                Ok(Value::List(values?))
            }

            Expr::Index { target, index } => {
                let list = target.eval(env.clone())?;
                let idx = index.eval(env.clone())?;

                match (list, idx) {
                    (Value::List(v), Value::Int(i)) => Ok(v
                        .get(i as usize)
                        .cloned()
                        .ok_or(RuntimeError::Message("index out of bounds"))?),

                    _ => Err(RuntimeError::Message("type is not indexable")),
                }
            }

            Expr::RecordLiteral(fields) => {
                let mut map = HashMap::new();

                for (k, v) in fields {
                    map.insert(k.clone(), v.eval(env.clone())?);
                }

                Ok(Value::Record(map))
            }

            Expr::Property { target, name } => {
                let record = target.eval(env.clone())?;

                match record {
                    Value::Record(map) => {
                        Ok(map
                            .get(name)
                            .cloned()
                            .ok_or(RuntimeError::RichMessage(format!(
                                "unknown property: {}",
                                name
                            )))?)
                    }

                    _ => Err(RuntimeError::RichMessage(format!(
                        "cannot access: {}, {:?} is not a record",
                        name, target
                    ))),
                }
            }

            Expr::Lambda { params, body } => Ok(Value::Function(Function {
                params: params.clone(),
                body: body.clone(),
                env: env.clone(),
            })),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },

    TryAssign {
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

impl Stmt {
    pub fn exec(&self, env: Rc<RefCell<Env>>) -> Result<ExecResult, RuntimeError> {
        match self {
            Stmt::Assign { name, value } => {
                let v = value.eval(env.clone())?;

                env.borrow_mut().vars.insert(name.clone(), v);

                Ok(ExecResult::Continue)
            }

            Stmt::TryAssign { name, value } => {
                let v = value.eval(env.clone())?;

                match v {
                    Value::Record(map) => {
                        let error = map
                            .get("error")
                            .ok_or(RuntimeError::RichMessage("missing error field".to_string()))?;

                        let value = map
                            .get("value")
                            .ok_or(RuntimeError::RichMessage("missing error field".to_string()))?;

                        match error {
                            Value::Bool(false) => {
                                env.borrow_mut().set(name.to_string(), value.clone());

                                Ok(ExecResult::Continue)
                            }

                            Value::Bool(true) => Ok(ExecResult::Return(Value::Record(map))),

                            _ => Err(RuntimeError::RichMessage(
                                "error field must be bool".to_string(),
                            )),
                        }
                    }

                    _ => {
                        return Err(RuntimeError::RichMessage(
                            "?= expects a record on the right side".to_string(),
                        ));
                    }
                }
            }

            Stmt::Expr(expr) => Ok(ExecResult::Value(expr.eval(env.clone())?)),

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = condition.eval(env.clone())?;

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        match stmt.exec(env.clone())? {
                            ExecResult::Continue | ExecResult::Value(_) => {}
                            other => return Ok(other),
                        }
                    }
                } else {
                    for stmt in else_branch {
                        match stmt.exec(env.clone())? {
                            ExecResult::Continue | ExecResult::Value(_) => {}
                            other => return Ok(other),
                        }
                    }
                }

                Ok(ExecResult::Continue)
            }

            Stmt::For {
                var,
                iterable,
                body,
            } => {
                let value = iterable.eval(env.clone())?;

                match value {
                    Value::List(items) => {
                        for item in items {
                            env.borrow_mut().set(var.clone(), item);

                            for stmt in body {
                                match stmt.exec(env.clone())? {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return Ok(ExecResult::Continue),
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }
                    }

                    other => {
                        return Err(RuntimeError::TypeError {
                            expected: "list",
                            got: format!("{:?}", other),
                        });
                    }
                }

                Ok(ExecResult::Continue)
            }

            Stmt::While { condition, body } => {
                loop {
                    let cond = condition.eval(env.clone())?;

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body {
                                match stmt.exec(env.clone())? {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return Ok(ExecResult::Continue),
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }

                        Value::Bool(false) => break,

                        other => {
                            return Err(RuntimeError::TypeError {
                                expected: "bool",
                                got: format!("{:?}", other),
                            });
                        }
                    }
                }

                Ok(ExecResult::Continue)
            }

            Stmt::Function { name, params, body } => {
                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                    env: env.clone(),
                };

                env.borrow_mut().set(name.clone(), Value::Function(func));

                Ok(ExecResult::Continue)
            }

            Stmt::Return(expr) => {
                let value = expr.eval(env.clone())?;

                Ok(ExecResult::Return(value))
            }

            Stmt::Break => Ok(ExecResult::Break),
            Stmt::Continue => Ok(ExecResult::LoopContinue),
        }
    }
}

pub enum ExecResult {
    Continue,
    Break,
    LoopContinue,
    Return(Value),
    Value(Value),
}
