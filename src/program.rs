use crate::{
    ast::{ExecResult, Stmt},
    runtime::{Env, RuntimeError},
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

        Ok(last)
    }
}
