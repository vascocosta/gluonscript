use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert(
        "create_folder".to_string(),
        Value::BuiltinFunction(create_folder),
    );

    map.insert("read_file".to_string(), Value::BuiltinFunction(read_file));
    map.insert("write_file".to_string(), Value::BuiltinFunction(write_file));

    Ok(Value::Record(map))
}

pub fn create_folder(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(path)) => match std::fs::create_dir(path) {
            Ok(_) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), Value::None),
            ]))),

            Err(e) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ]))),
        },

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn read_file(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(path)) => match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), Value::String(contents)),
            ]))),

            Err(e) => Ok(Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ]))),
        },

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn write_file(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::Arity {
            expected: 2,
            got: args.len(),
        });
    }

    let contents = args.pop().ok_or(RuntimeError::Arity {
        expected: 2,
        got: args.len(),
    })?;

    match args.pop() {
        Some(Value::String(path)) => match contents {
            Value::String(contents) => match std::fs::write(path, contents) {
                Ok(_) => Ok(Value::Record(HashMap::from([
                    ("error".to_string(), Value::Bool(false)),
                    ("value".to_string(), Value::None),
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
        },

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}
