use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("input".to_string(), Value::BuiltinFunction(input));
    map.insert("print".to_string(), Value::BuiltinFunction(print));
    map.insert("println".to_string(), Value::BuiltinFunction(println));

    Ok(Value::Record(map))
}

pub fn input(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let mut buf: String = String::new();

    io::stdin()
        .read_line(&mut buf)
        .map_err(|_| RuntimeError::Message("input: could not read from stdin"))?;

    Ok(Value::String(
        buf.trim_end_matches(['\n', '\r']).to_string(),
    ))
}

pub fn print(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for a in args {
        print!("{a}");
    }

    io::stdout().flush().unwrap();

    Ok(Value::None)
}

pub fn println(args: Vec<Value>) -> Result<Value, RuntimeError> {
    print(args)?;
    println!();

    Ok(Value::None)
}
