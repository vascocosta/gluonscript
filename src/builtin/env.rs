use std::{collections::HashMap, env};

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("args".to_string(), Value::BuiltinFunction(args));
    map.insert("consts".to_string(), Value::BuiltinFunction(consts));
    map.insert("vars".to_string(), Value::BuiltinFunction(vars));

    Ok(Value::Record(map))
}

pub fn args(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let args: Vec<String> = env::args().collect();
    let values = args.iter().map(|a| Value::String(a.to_owned())).collect();

    Ok(Value::List(values))
}

pub fn consts(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert(
        "ARCH".to_string(),
        Value::String(env::consts::ARCH.to_string()),
    );

    map.insert(
        "DLL_EXTENSION".to_string(),
        Value::String(env::consts::DLL_EXTENSION.to_string()),
    );

    map.insert(
        "DLL_PREFIX".to_string(),
        Value::String(env::consts::DLL_PREFIX.to_string()),
    );

    map.insert(
        "DLL_SUFFIX".to_string(),
        Value::String(env::consts::DLL_SUFFIX.to_string()),
    );

    map.insert(
        "EXE_EXTENSION".to_string(),
        Value::String(env::consts::EXE_EXTENSION.to_string()),
    );

    map.insert(
        "EXE_SUFFIX".to_string(),
        Value::String(env::consts::EXE_SUFFIX.to_string()),
    );

    map.insert(
        "FAMILY".to_string(),
        Value::String(env::consts::FAMILY.to_string()),
    );

    map.insert("OS".to_string(), Value::String(env::consts::OS.to_string()));

    Ok(Value::Record(map))
}

pub fn vars(_: Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Record(
        env::vars().map(|(k, v)| (k, Value::String(v))).collect(),
    ))
}
