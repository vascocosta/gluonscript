use crate::ast::{Env, ExecResult, Expr, Function, Stmt, Value};

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn exec_stmt(stmt: &Stmt, env: &mut Env) -> ExecResult {
        match stmt {
            Stmt::Assign { name, value } => {
                let v = Expr::eval_expr(value, env);
                env.vars.insert(name.clone(), v);
                ExecResult::Continue
            }
            Stmt::Expr(expr) => ExecResult::Value(Expr::eval_expr(expr, env)),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = Expr::eval_expr(condition, env);

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        match Self::exec_stmt(stmt, env) {
                            ExecResult::Continue | ExecResult::Value(_) => {}
                            other => return other,
                        }
                    }
                } else {
                    for stmt in else_branch {
                        match Self::exec_stmt(stmt, env) {
                            ExecResult::Continue | ExecResult::Value(_) => {}
                            other => return other,
                        }
                    }
                }

                ExecResult::Continue
            }
            Stmt::For {
                var,
                iterable,
                body,
            } => {
                let value = Expr::eval_expr(iterable, env);

                match value {
                    Value::List(items) => {
                        for item in items {
                            env.set(var.clone(), item);

                            for stmt in body {
                                match Self::exec_stmt(stmt, env) {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return ExecResult::Continue,
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return ExecResult::Return(v),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }
                    }

                    _ => panic!("for loop expects a list"),
                }

                ExecResult::Continue
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = Expr::eval_expr(condition, env);

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body {
                                match Self::exec_stmt(stmt, env) {
                                    ExecResult::Continue => {}
                                    ExecResult::Break => return ExecResult::Continue,
                                    ExecResult::LoopContinue => break,
                                    ExecResult::Return(v) => return ExecResult::Return(v),
                                    ExecResult::Value(_) => {}
                                }
                            }
                        }

                        Value::Bool(false) => break,

                        _ => panic!("while condition must be bool"),
                    }
                }

                ExecResult::Continue
            }
            Stmt::Function { name, params, body } => {
                env.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                ExecResult::Continue
            }
            Stmt::Return(expr) => {
                let value = Expr::eval_expr(expr, env);
                ExecResult::Return(value)
            }
            Stmt::Break => ExecResult::Break,
            Stmt::Continue => ExecResult::LoopContinue,
        }
    }

    pub fn run(&self) -> ExecResult {
        let mut env = Env::new();
        let mut last = ExecResult::Continue;

        for stmt in &self.statements {
            if let ExecResult::Value(v) = Self::exec_stmt(stmt, &mut env) {
                last = ExecResult::Value(v);
            }
        }

        last
    }
}
