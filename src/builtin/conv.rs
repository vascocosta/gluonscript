use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("float".to_string(), Value::BuiltinFunction(float));
    map.insert("int".to_string(), Value::BuiltinFunction(int));
    map.insert("string".to_string(), Value::BuiltinFunction(string));

    Ok(Value::Record(map))
}

pub fn float(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => Ok(Value::Float(s.trim_ascii().parse().map_err(|_| {
            RuntimeError {
                message: "float expects a valid number string".to_string(),
            }
        })?)),
        _ => Err(RuntimeError {
            message: "float expects a string".to_string(),
        }),
    }
}

pub fn int(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.trim_ascii().parse().map_err(|_| {
            RuntimeError {
                message: "int expects a valid number string".to_string(),
            }
        })?)),
        _ => Err(RuntimeError {
            message: "int expects a string argument".to_string(),
        }),
    }
}

pub fn string(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::Int(n) => Ok(Value::String(n.to_string())),
        Value::Float(n) => Ok(Value::String(n.to_string())),
        Value::Bool(b) => Ok(Value::String(b.to_string())),
        Value::List(l) => Ok(Value::String(format!("{:?}", l))),
        Value::Record(r) => Ok(Value::String(format!("{:?}", r))),
        _ => Err(RuntimeError {
            message: "unable to convert type to string".to_string(),
        }),
    }
}
