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
    match args.first() {
        Some(Value::String(s)) => match s.trim_ascii().parse() {
            Ok(f) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), Value::Float(f)),
            ]))),
            Err(e) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ]))),
        },

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn int(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => match s.trim_ascii().parse() {
            Ok(i) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), Value::Int(i)),
            ]))),
            Err(e) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ]))),
        },

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn string(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(v) => Ok(Value::String(v.to_string())),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),
    }
}
