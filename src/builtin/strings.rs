use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("contains".to_string(), Value::BuiltinFunction(contains));
    map.insert("join".to_string(), Value::BuiltinFunction(join));
    map.insert("lower".to_string(), Value::BuiltinFunction(lower));
    map.insert("upper".to_string(), Value::BuiltinFunction(upper));
    map.insert("replace".to_string(), Value::BuiltinFunction(replace));
    map.insert("split".to_string(), Value::BuiltinFunction(split));

    Ok(Value::Record(map))
}

pub fn contains(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::Arity {
            expected: 2,
            got: args.len(),
        });
    }

    let value = args.pop().ok_or(RuntimeError::Arity {
        expected: 2,
        got: args.len(),
    })?;

    match args.pop() {
        Some(Value::String(s)) => {
            let substr = match value {
                Value::String(substr) => substr,

                other => {
                    return Err(RuntimeError::TypeError {
                        expected: "string",
                        got: format!("{:?}", other),
                    });
                }
            };

            if s.contains(&substr) {
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
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

pub fn replace(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::Arity {
            expected: 3,
            got: args.len(),
        });
    }

    let new_value = args.pop().ok_or(RuntimeError::Arity {
        expected: 2,
        got: args.len(),
    })?;

    let old_value = args.pop().ok_or(RuntimeError::Arity {
        expected: 2,
        got: args.len(),
    })?;

    match args.pop() {
        Some(Value::String(s)) => {
            let old = match old_value {
                Value::String(old) => old,

                other => {
                    return Err(RuntimeError::TypeError {
                        expected: "string",
                        got: format!("{:?}", other),
                    });
                }
            };

            let new = match new_value {
                Value::String(new) => new,

                other => {
                    return Err(RuntimeError::TypeError {
                        expected: "string",
                        got: format!("{:?}", other),
                    });
                }
            };

            Ok(Value::String(s.replace(&old, &new)))
        }

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
