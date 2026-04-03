use std::{collections::HashMap, fmt::Display};

use crate::{
    ast::{ExecResult, Stmt},
    builtin,
};

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
                write!(f, "arity error: expected: {} instead of: {}", expected, got)
            }

            RuntimeError::Message(m) => write!(f, "{m}"),
            RuntimeError::RichMessage(m) => write!(f, "{m}"),

            RuntimeError::TypeError { expected, got } => {
                write!(f, "type error: expected: {} instead of {}", expected, got)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Record(HashMap<String, Value>),
    Function(Function),
    BuiltinFunction(fn(Vec<Value>) -> Result<Value, RuntimeError>),
}

impl Value {
    pub fn call(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match self {
            Value::Function(func) => {
                let mut local_env = func.env.child();

                for (param, value) in func.params.iter().zip(args) {
                    local_env.set(param.clone(), value);
                }

                let mut result = Value::None;

                for stmt in &func.body {
                    match stmt.exec(&mut local_env)? {
                        ExecResult::Continue => {}
                        ExecResult::Break => {}
                        ExecResult::LoopContinue => {}
                        ExecResult::Return(v) => return Ok(v),
                        ExecResult::Value(v) => result = v,
                    }
                }

                Ok(result)
            }

            Value::BuiltinFunction(f) => f(args),
            _ => Err(RuntimeError::Message("type is not callable")),
        }
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(fmt, "None"),
            Value::Int(i) => write!(fmt, "{i}"),
            Value::Float(f) => write!(fmt, "{}", f),
            Value::String(s) => write!(fmt, "{s}"),
            Value::Bool(b) => write!(fmt, "{b}"),

            Value::List(v) => {
                let values: Vec<String> = v.iter().map(|v| format!("{}", v)).collect();

                write!(fmt, "[{}]", values.join(", "))
            }

            Value::Record(o) => write!(fmt, "{:?}", o),
            Value::Function(f) => write!(fmt, "{:?}", f),
            Value::BuiltinFunction(f) => write!(fmt, "{:?}", f),
        }
    }
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
        self.set("slice".to_string(), Value::BuiltinFunction(builtin::slice));
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}
