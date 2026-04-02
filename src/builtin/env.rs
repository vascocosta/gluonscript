use std::{collections::HashMap, env};

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("args".to_string(), Value::BuiltinFunction(args));
    map.insert("vars".to_string(), Value::BuiltinFunction(vars));

    Ok(Value::Record(map))
}

pub fn args(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let args: Vec<String> = env::args().collect();
    let values = args.iter().map(|a| Value::String(a.to_owned())).collect();

    Ok(Value::List(values))
}

pub fn vars(_: Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Record(
        env::vars().map(|(k, v)| (k, Value::String(v))).collect(),
    ))
}
