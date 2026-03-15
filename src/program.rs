use crate::ast::{Env, Expr, Function, Stmt, Value};

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
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
                else_branch,
            } => {
                let cond = Expr::eval_expr(condition, env);

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        Self::exec_stmt(stmt, env);
                    }
                } else {
                    for stmt in else_branch {
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
            Stmt::Function { name, params, body } => {
                env.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
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
