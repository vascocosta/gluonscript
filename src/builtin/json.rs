use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("parse".to_string(), Value::BuiltinFunction(parse));

    Ok(Value::Record(map))
}

pub fn parse(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => match serde_json::from_str(s) {
            Ok(parsed_json) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), json_to_value(parsed_json)),
            ]))),
            Err(e) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ]))),
        },
        _ => Err(RuntimeError {
            message: "json expects a string argument".to_string(),
        }),
    }
}

fn json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Value::Int(n.as_i64().unwrap_or_default())
            } else if n.is_u64() {
                Value::Int(n.as_i64().unwrap_or_default())
            } else if n.is_f64() {
                Value::Float(n.as_f64().unwrap_or_default())
            } else {
                Value::Null
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Array(arr) => Value::List(arr.into_iter().map(json_to_value).collect()),
        serde_json::Value::Object(map) => Value::Record(
            map.into_iter()
                .map(|(k, v)| (k, json_to_value(v)))
                .collect(),
        ),
    }
}
