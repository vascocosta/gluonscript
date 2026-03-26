use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("args".to_string(), Value::BuiltinFunction(args));

    Ok(Value::Record(map))
}

pub fn args(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let args: Vec<String> = std::env::args().collect();
    let values = args.iter().map(|a| Value::String(a.to_owned())).collect();

    Ok(Value::List(values))
}
