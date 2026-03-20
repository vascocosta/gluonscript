use crate::ast::{Env, ExecResult, Expr, Function, Stmt, Value};

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Result<ExecResult, RuntimeError> {
        match stmt {
            Stmt::Assign { name, value } => {
                let v = Expr::eval_expr(value, env)?;
                env.vars.insert(name.clone(), v);
                Ok(ExecResult::Continue)
            }
            Stmt::Expr(expr) => Ok(ExecResult::Value(Expr::eval_expr(expr, env)?)),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = Expr::eval_expr(condition, env)?;

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        match Self::exec_stmt(stmt, env)? {
                            ExecResult::Continue | ExecResult::Value(_) => {}
                            other => return Ok(other),
                        }
                    }
                } else {
                    for stmt in else_branch {
                        match Self::exec_stmt(stmt, env)? {
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
                let value = Expr::eval_expr(iterable, env)?;

                match value {
                    Value::List(items) => {
                        for item in items {
                            env.set(var.clone(), item);

                            for stmt in body {
                                match Self::exec_stmt(stmt, env)? {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return Ok(ExecResult::Continue),
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }
                    }

                    _ => {
                        return Err(RuntimeError {
                            message: "for loop expects a list".to_string(),
                        });
                    }
                }

                Ok(ExecResult::Continue)
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = Expr::eval_expr(condition, env)?;

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body {
                                match Self::exec_stmt(stmt, env)? {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return Ok(ExecResult::Continue),
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }

                        Value::Bool(false) => break,

                        _ => {
                            return Err(RuntimeError {
                                message: "while condition must be bool".to_string(),
                            });
                        }
                    }
                }

                Ok(ExecResult::Continue)
            }
            Stmt::Function { name, params, body } => {
                env.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(ExecResult::Continue)
            }
            Stmt::Return(expr) => {
                let value = Expr::eval_expr(expr, env)?;
                Ok(ExecResult::Return(value))
            }
            Stmt::Break => Ok(ExecResult::Break),
            Stmt::Continue => Ok(ExecResult::LoopContinue),
        }
    }

    pub fn run(&self) -> Result<ExecResult, RuntimeError> {
        let mut env = Env::new();
        let mut last = ExecResult::Continue;

        for stmt in &self.statements {
            if let ExecResult::Value(v) = Self::exec_stmt(stmt, &mut env)? {
                last = ExecResult::Value(v);
            }
        }

        Ok(last)
    }
}
