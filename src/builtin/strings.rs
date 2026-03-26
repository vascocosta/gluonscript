use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("lower".to_string(), Value::BuiltinFunction(lower));
    map.insert("upper".to_string(), Value::BuiltinFunction(upper));

    Ok(Value::Record(map))
}

pub fn lower(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
        _ => Err(RuntimeError {
            message: "lower expects a string argument".to_string(),
        }),
    }
}

pub fn upper(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        _ => Err(RuntimeError {
            message: "upper expects a string argument".to_string(),
        }),
    }
}
