use std::{collections::HashMap, fmt::Display};

use crate::{ast::Stmt, builtin};

pub enum RuntimeError {
    Arity { expected: usize, got: usize },
    Message(&'static str),
    RichMessage(String),
    TypeError { expected: &'static str, got: String },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Arity { expected, got } => {
                writeln!(f, "arity error: expected: {} instead of: {}", expected, got)
            }

            RuntimeError::Message(m) => writeln!(f, "{m}"),
            RuntimeError::RichMessage(m) => writeln!(f, "{m}"),

            RuntimeError::TypeError { expected, got } => {
                writeln!(f, "type error: expected: {} instead of {}", expected, got)
            }
        }
    }
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
    Function(Function),
    BuiltinFunction(fn(Vec<Value>) -> Result<Value, RuntimeError>),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub env: Env,
}

#[derive(Clone, Debug)]
pub struct Env {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn child(&self) -> Self {
        Self {
            vars: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    pub fn get_vars(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.vars.get(name) {
            return Some(v.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get_vars(name);
        }

        None
    }

    pub fn prelude(&mut self) {
        self.set(
            "append".to_string(),
            Value::BuiltinFunction(builtin::append),
        );

        self.set(
            "import".to_string(),
            Value::BuiltinFunction(builtin::import),
        );

        self.set("len".to_string(), Value::BuiltinFunction(builtin::len));
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}
