use crate::{
    ast::{ExecResult, Expr, Stmt},
    runtime::{Env, RuntimeError, Value},
};

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn run(&self) -> Result<ExecResult, RuntimeError> {
        let mut env = Env::new();

        env.prelude();

        let mut last = ExecResult::Continue;

        for stmt in &self.statements {
            if let ExecResult::Value(v) = stmt.exec(&mut env)? {
                last = ExecResult::Value(v);
            }
        }

        if let Some(Value::Function(_)) = env.get_vars("main") {
            let main_call = Stmt::Expr(Expr::Call {
                callee: Box::new(Expr::Variable("main".to_string())),
                args: vec![],
            });

            if let ExecResult::Value(v) = main_call.exec(&mut env)? {
                last = ExecResult::Value(v);
            }
        }

        Ok(last)
    }
}
