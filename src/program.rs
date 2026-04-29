use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{ExecResult, Expr, Stmt},
    runtime::{Env, RuntimeError, Value},
};

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn run(&self) -> Result<ExecResult, RuntimeError> {
        let env = Rc::new(RefCell::new(Env::new()));

        env.borrow_mut().prelude();

        let mut last = ExecResult::Continue;

        for stmt in &self.statements {
            if let ExecResult::Value(v) = stmt.exec(env.clone())? {
                last = ExecResult::Value(v);
            }
        }

        if let Some(Value::Function(_)) = env.borrow().get_vars("main") {
            let main_call = Stmt::Expr(Expr::Call {
                callee: Box::new(Expr::Variable("main".to_string())),
                args: vec![],
            });

            if let ExecResult::Value(v) = main_call.exec(env.clone())? {
                last = ExecResult::Value(v);
            }
        }

        Ok(last)
    }
}
