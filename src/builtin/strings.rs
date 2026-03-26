use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("join".to_string(), Value::BuiltinFunction(join));
    map.insert("lower".to_string(), Value::BuiltinFunction(lower));
    map.insert("upper".to_string(), Value::BuiltinFunction(upper));
    map.insert("split".to_string(), Value::BuiltinFunction(split));

    Ok(Value::Record(map))
}

pub fn join(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::List(v)) => {
            let v: Vec<String> = v.iter().map(|v| v.to_string()).collect();

            let sep = match args.iter().nth(1) {
                Some(Value::String(s)) => s,
                _ => " ",
            };

            Ok(Value::String(v.join(sep)))
        }

        other => Err(RuntimeError::TypeError {
            expected: "list",
            got: format!("{:?}", other),
        }),
    }
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

pub fn split(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => {
            let sep = match args.iter().nth(1) {
                Some(Value::String(s)) => s,
                _ => " ",
            };

            let split: Vec<&str> = s.split(sep).collect();
            let v: Vec<Value> = split.iter().map(|e| Value::String(e.to_string())).collect();

            Ok(Value::List(v))
        }

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}
