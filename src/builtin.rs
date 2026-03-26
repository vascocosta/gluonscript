use std::fs::read_to_string;
use std::{collections::HashMap, env, io};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::{Env, RuntimeError, Value};

pub fn conv_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("float".to_string(), Value::BuiltinFunction(float));
    map.insert("int".to_string(), Value::BuiltinFunction(int));
    map.insert("string".to_string(), Value::BuiltinFunction(string));

    Ok(Value::Record(map))
}

pub fn core_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("import".to_string(), Value::BuiltinFunction(import));
    map.insert("len".to_string(), Value::BuiltinFunction(len));

    Ok(Value::Record(map))
}

pub fn env_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("args".to_string(), Value::BuiltinFunction(args));

    Ok(Value::Record(map))
}

pub fn http_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("get".to_string(), Value::BuiltinFunction(get));

    Ok(Value::Record(map))
}

pub fn io_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("input".to_string(), Value::BuiltinFunction(input));
    map.insert("print".to_string(), Value::BuiltinFunction(print));
    map.insert("println".to_string(), Value::BuiltinFunction(println));

    Ok(Value::Record(map))
}

pub fn json_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("parse".to_string(), Value::BuiltinFunction(parse));

    Ok(Value::Record(map))
}

pub fn lists_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("append".to_string(), Value::BuiltinFunction(append));
    map.insert("len".to_string(), Value::BuiltinFunction(len));

    Ok(Value::Record(map))
}

pub fn strings_module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("lower".to_string(), Value::BuiltinFunction(lower));
    map.insert("upper".to_string(), Value::BuiltinFunction(upper));

    Ok(Value::Record(map))
}

pub fn append(mut args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError {
            message: "append expects 2 arguments".to_string(),
        });
    }

    let value = args.pop().ok_or(RuntimeError {
        message: "append expects 2 arguments".to_string(),
    })?;

    match args.pop() {
        Some(Value::List(mut list)) => {
            list.push(value);

            Ok(Value::List(list))
        }

        Some(Value::String(s1)) => match value {
            Value::String(s2) => Ok(Value::String(format!("{}{}", s1, s2))),
            _ => Err(RuntimeError {
                message: "append expects a string when appending to a string".to_string(),
            }),
        },

        _ => Err(RuntimeError {
            message: "append expects a list as first argument".to_string(),
        }),
    }
}

pub fn args(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let args: Vec<String> = env::args().collect();
    let values = args.iter().map(|a| Value::String(a.to_owned())).collect();

    Ok(Value::List(values))
}

pub fn float(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => Ok(Value::Float(s.trim_ascii().parse().map_err(|_| {
            RuntimeError {
                message: "float expects a valid number string".to_string(),
            }
        })?)),
        _ => Err(RuntimeError {
            message: "float expects a string".to_string(),
        }),
    }
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

pub fn import(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => match s.as_str() {
            "conv" => conv_module(),
            "core" => core_module(),
            "env" => env_module(),
            "http" => http_module(),
            "io" => io_module(),
            "json" => json_module(),
            "lists" => lists_module(),
            "strings" => strings_module(),

            _ => {
                let source = read_to_string(s).map_err(|_| RuntimeError {
                    message: "import: could not read source file".to_string(),
                })?;

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.tokenize().map_err(|_| RuntimeError {
                    message: "import: could not tokenize source".to_string(),
                })?;

                let mut parser = Parser { tokens, pos: 0 };
                let program = parser.parse_program().map_err(|_| RuntimeError {
                    message: "import: could not parse program".to_string(),
                })?;

                let mut env = Env::new();

                env.prelude();

                for stmt in &program.statements {
                    stmt.exec(&mut env).map_err(|_| RuntimeError {
                        message: "import: could not tokenize source".to_string(),
                    })?;
                }

                Ok(Value::Record(env.vars))
            }
        },

        _ => Err(RuntimeError {
            message: "import expects a string argument".to_string(),
        }),
    }
}

pub fn input(_: Vec<Value>) -> Result<Value, RuntimeError> {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).map_err(|_| RuntimeError {
        message: "input expects stdin to work".to_string(),
    })?;

    Ok(Value::String(
        buf.trim_end_matches(['\n', '\r']).to_string(),
    ))
}

pub fn int(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.trim_ascii().parse().map_err(|_| {
            RuntimeError {
                message: "int expects a valid number string".to_string(),
            }
        })?)),
        _ => Err(RuntimeError {
            message: "int expects a string argument".to_string(),
        }),
    }
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

pub fn len(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::List(v) => Ok(Value::Int(v.len() as i64)),
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(RuntimeError {
            message: "len(): unsuported type".to_string(),
        }),
    }
}

pub fn print(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for a in args {
        match a {
            Value::Int(n) => print!("{}", n),
            Value::Float(n) => print!("{}", n),
            Value::Bool(b) => print!("{}", b),
            Value::String(s) => print!("{}", s),
            Value::List(l) => print!("{:?}", l),
            Value::Record(o) => print!("{:?}", o),
            Value::Null => print!("Null"),
            Value::Function(f) => print!("{:?}", f),
            Value::BuiltinFunction(f) => print!("{:?}", f),
        }
    }

    Ok(Value::Bool(true))
}

pub fn println(args: Vec<Value>) -> Result<Value, RuntimeError> {
    print(args)?;
    println!();

    Ok(Value::Bool(true))
}

pub fn string(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::Int(n) => Ok(Value::String(n.to_string())),
        Value::Float(n) => Ok(Value::String(n.to_string())),
        Value::Bool(b) => Ok(Value::String(b.to_string())),
        Value::List(l) => Ok(Value::String(format!("{:?}", l))),
        Value::Record(r) => Ok(Value::String(format!("{:?}", r))),
        _ => Err(RuntimeError {
            message: "unable to convert type to string".to_string(),
        }),
    }
}

pub fn lower(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
        _ => Err(RuntimeError {
            message: "lower expects a string argument".to_string(),
        }),
    }
}

pub fn upper(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        _ => Err(RuntimeError {
            message: "upper expects a string argument".to_string(),
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
