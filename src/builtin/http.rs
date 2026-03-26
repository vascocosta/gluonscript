use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("get".to_string(), Value::BuiltinFunction(get));

    Ok(Value::Record(map))
}

pub fn get(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => {
            let response = reqwest::blocking::get(s);

            match response {
                Ok(reponse) => match reponse.text() {
                    Ok(text) => Ok(Value::Record(HashMap::from([
                        ("error".to_string(), Value::Bool(false)),
                        ("value".to_string(), Value::String(text)),
                    ]))),
                    Err(e) => Ok(Value::Record(HashMap::from([
                        ("error".to_string(), Value::Bool(true)),
                        ("value".to_string(), Value::String(e.to_string())),
                    ]))),
                },
                Err(e) => Ok(Value::Record(HashMap::from([
                    ("error".to_string(), Value::Bool(true)),
                    ("value".to_string(), Value::String(e.to_string())),
                ]))),
            }
        }
        _ => Err(RuntimeError {
            message: "get expects a string argument".to_string(),
        }),
    }
}
