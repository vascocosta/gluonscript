mod conv;
mod env;
mod fs;
mod http;
mod io;
mod json;
mod strings;

use std::io::ErrorKind;
use std::path::Path;
use std::process;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::{Env, RuntimeError, Value};

pub fn append(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
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
        Some(Value::List(mut list)) => {
            list.push(value);

            Ok(Value::List(list))
        }

        Some(Value::String(s1)) => match value {
            Value::String(s2) => Ok(Value::String(format!("{}{}", s1, s2))),

            other => Err(RuntimeError::TypeError {
                expected: "string",
                got: format!("{:?}", other),
            }),
        },

        other => Err(RuntimeError::TypeError {
            expected: "list",
            got: format!("{:?}", other),
        }),
    }
}

pub fn exit(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::Int(code)) => process::exit(*code as i32),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "int",
            got: format!("{:?}", other),
        }),
    }
}

pub fn import(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => match s.as_str() {
            // Handle built-in function cases for which a file does not exist\.
            "std/conv" => conv::module(),
            "std/env" => env::module(),
            "std/fs" => fs::module(),
            "std/http" => http::module(),
            "std/io" => io::module(),
            "std/json" => json::module(),
            "std/strings" => strings::module(),

            // Handle cases where the imported module is a file stored on the file system.
            _ => {
                let source = match std::fs::read_to_string(Path::new(s).with_extension("gs")) {
                    // The source file was found in the current directory.
                    Ok(source) => source,

                    // The source file was not found in the current directory.
                    // Fallback to .gluonscript within the user's home folder.
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            let root = if std::env::consts::OS == "windows" {
                                std::env::var("USERPROFILE")
                            } else {
                                std::env::var("HOME")
                            }
                            .map_err(|_| {
                                RuntimeError::Message("import: could not read source file")
                            })?;

                            std::fs::read_to_string(
                                Path::new(&root)
                                    .join(".gluonscript")
                                    .join(s)
                                    .with_extension("gs"),
                            )
                            .map_err(|_| {
                                RuntimeError::Message("import: could not read source file")
                            })?
                        }

                        _ => {
                            return Err(RuntimeError::Message(
                                "import: could not read source file",
                            ));
                        }
                    },
                };

                let mut lexer = Lexer::new(&source);
                let tokens = lexer
                    .tokenize()
                    .map_err(|_| RuntimeError::Message("import: could not tokenize source"))?;

                let mut parser = Parser { tokens, pos: 0 };
                let program = parser
                    .parse_program()
                    .map_err(|_| RuntimeError::Message("import: could not parse program"))?;

                let mut env = Env::new();

                env.prelude();

                for stmt in &program.statements {
                    stmt.exec(&mut env)
                        .map_err(|_| RuntimeError::Message("import: could not tokenize source"))?;
                }

                Ok(Value::Record(env.vars))
            }
        },

        other => Err(RuntimeError::TypeError {
            expected: "string",
            got: format!("{:?}", other),
        }),
    }
}

pub fn len(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::List(v)) => Ok(Value::Int(v.len() as i64)),
        Some(Value::String(s)) => Ok(Value::Int(s.len() as i64)),
        _ => Err(RuntimeError::Message("len: unsuported type")),
    }
}

pub fn slice(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::List(l)) => {
            let start = args
                .iter()
                .nth(1)
                .ok_or(RuntimeError::Message("no start provided"))?;

            let end = args
                .iter()
                .nth(2)
                .ok_or(RuntimeError::Message("no end provided"))?;

            match (start, end) {
                (Value::Int(start), Value::Int(end)) => {
                    Ok(Value::List(l[*start as usize..*end as usize].to_vec()))
                }

                other => Err(RuntimeError::TypeError {
                    expected: "int, int",
                    got: format!("{}, {}", other.0, other.1),
                }),
            }
        }

        Some(other) => Err(RuntimeError::TypeError {
            expected: "list",
            got: format!("{}", other),
        }),

        _ => Err(RuntimeError::Arity {
            expected: 3,
            got: 0,
        }),
    }
}

pub fn update(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::Arity {
            expected: 3,
            got: args.len(),
        });
    }

    let new_value = args.pop().ok_or(RuntimeError::Arity {
        expected: 3,
        got: args.len(),
    })?;

    match args.pop() {
        Some(Value::Int(index)) => match args.pop() {
            Some(Value::List(mut values)) => {
                if index < 0 || index as usize >= values.len() {
                    return Err(RuntimeError::Message("index out of bounds"));
                }

                values[index as usize] = new_value;

                Ok(Value::List(values))
            }

            Some(other) => Err(RuntimeError::TypeError {
                expected: "list",
                got: other.to_string(),
            }),

            None => Err(RuntimeError::Arity {
                expected: 3,
                got: args.len(),
            }),
        },

        Some(Value::String(key)) => match args.pop() {
            Some(Value::Record(mut map)) => {
                if !map.contains_key(&key) {
                    return Err(RuntimeError::Message("unknown key"));
                }

                map.insert(key, new_value);

                Ok(Value::Record(map))
            }

            Some(other) => Err(RuntimeError::TypeError {
                expected: "record",
                got: other.to_string(),
            }),

            None => Err(RuntimeError::Arity {
                expected: 3,
                got: args.len(),
            }),
        },

        _ => Err(RuntimeError::TypeError {
            expected: "int/string",
            got: "unsuported index/key".to_string(),
        }),
    }
}
