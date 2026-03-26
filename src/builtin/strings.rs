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
        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn upper(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}
