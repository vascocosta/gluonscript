use std::fs::read_to_string;

use std::{collections::HashMap, env, io};

use crate::ast::{Env, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program::Program;

pub fn conv_module() -> Value {
    let mut map = HashMap::new();

    map.insert("float".to_string(), Value::BuiltinFunction(float));
    map.insert("int".to_string(), Value::BuiltinFunction(int));
    map.insert("string".to_string(), Value::BuiltinFunction(string));

    Value::Record(map)
}

pub fn core_module() -> Value {
    let mut map = HashMap::new();

    map.insert("import".to_string(), Value::BuiltinFunction(import));
    map.insert("len".to_string(), Value::BuiltinFunction(len));

    Value::Record(map)
}

pub fn env_module() -> Value {
    let mut map = HashMap::new();

    map.insert("args".to_string(), Value::BuiltinFunction(args));

    Value::Record(map)
}

pub fn http_module() -> Value {
    let mut map = HashMap::new();

    map.insert("get".to_string(), Value::BuiltinFunction(get));

    Value::Record(map)
}

pub fn io_module() -> Value {
    let mut map = HashMap::new();

    map.insert("input".to_string(), Value::BuiltinFunction(input));
    map.insert("print".to_string(), Value::BuiltinFunction(print));
    map.insert("println".to_string(), Value::BuiltinFunction(println));

    Value::Record(map)
}

pub fn json_module() -> Value {
    let mut map = HashMap::new();

    map.insert("parse".to_string(), Value::BuiltinFunction(parse));

    Value::Record(map)
}

pub fn lists_module() -> Value {
    let mut map = HashMap::new();

    map.insert("append".to_string(), Value::BuiltinFunction(append));
    map.insert("len".to_string(), Value::BuiltinFunction(len));

    Value::Record(map)
}

pub fn append(args: &[Value]) -> Value {
    match &args[0] {
        Value::List(v) => {
            if args.len() != 2 {
                panic!("append expects 2 arguments")
            }

            let mut new_list = v.clone();

            new_list.push(args[1].clone());

            Value::List(new_list)
        }
        _ => panic!("append expects a list"),
    }
}

pub fn args(_: &[Value]) -> Value {
    let args: Vec<String> = env::args().collect();
    let values = args.iter().map(|a| Value::String(a.to_owned())).collect();

    Value::List(values)
}

pub fn float(args: &[Value]) -> Value {
    match &args[0] {
        Value::String(s) => Value::Float(
            s.trim_ascii()
                .parse()
                .expect("float expects a valid number string"),
        ),
        _ => panic!("float expects a string"),
    }
}

pub fn get(args: &[Value]) -> Value {
    match &args[0] {
        Value::String(s) => {
            let response = reqwest::blocking::get(s);

            match response {
                Ok(reponse) => match reponse.text() {
                    Ok(text) => Value::Record(HashMap::from([
                        ("error".to_string(), Value::Bool(false)),
                        ("value".to_string(), Value::String(text)),
                    ])),
                    Err(e) => Value::Record(HashMap::from([
                        ("error".to_string(), Value::Bool(true)),
                        ("value".to_string(), Value::String(e.to_string())),
                    ])),
                },
                Err(e) => Value::Record(HashMap::from([
                    ("error".to_string(), Value::Bool(true)),
                    ("value".to_string(), Value::String(e.to_string())),
                ])),
            }
        }
        _ => panic!("get expects a string argument"),
    }
}

pub fn import(args: &[Value]) -> Value {
    match &args[0] {
        Value::String(s) => match s.as_str() {
            "conv" => conv_module(),
            "core" => core_module(),
            "env" => env_module(),
            "http" => http_module(),
            "io" => io_module(),
            "json" => json_module(),
            "lists" => lists_module(),

            _ => {
                let source = read_to_string(s).expect("import: could not read source file");

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.tokenize().expect("import: could not tokenize source");

                let mut parser = Parser { tokens, pos: 0 };
                let program = parser
                    .parse_program()
                    .expect("import: could not parse program");

                let mut env = Env::new();

                for stmt in &program.statements {
                    Program::exec_stmt(stmt, &mut env).expect("import: could not import module");
                }

                Value::Record(env.vars)
            }
        },

        _ => panic!("import expects a string argument"),
    }
}

pub fn input(_: &[Value]) -> Value {
    let mut buf: String = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("input expects stdin to work");

    Value::String(buf.trim_end_matches(['\n', '\r']).to_string())
}

pub fn int(args: &[Value]) -> Value {
    match &args[0] {
        Value::String(s) => Value::Int(
            s.trim_ascii()
                .parse()
                .expect("int expects a valid number string"),
        ),
        _ => panic!("int expects a string"),
    }
}

pub fn parse(args: &[Value]) -> Value {
    match &args[0] {
        Value::String(s) => match serde_json::from_str(s) {
            Ok(parsed_json) => Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(false)),
                ("value".to_string(), json_to_value(parsed_json)),
            ])),
            Err(e) => Value::Record(HashMap::from([
                ("error".to_string(), Value::Bool(true)),
                ("value".to_string(), Value::String(e.to_string())),
            ])),
        },
        _ => panic!("json expects a string argument"),
    }
}

pub fn len(args: &[Value]) -> Value {
    match &args[0] {
        Value::List(v) => Value::Int(v.len() as i64),
        Value::String(s) => Value::Int(s.len() as i64),
        _ => panic!("len() unsupported type"),
    }
}

pub fn print(args: &[Value]) -> Value {
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

    Value::Bool(true)
}

pub fn println(args: &[Value]) -> Value {
    print(args);
    println!();

    Value::Bool(true)
}

pub fn string(args: &[Value]) -> Value {
    match &args[0] {
        Value::Int(n) => Value::String(n.to_string()),
        Value::Float(n) => Value::String(n.to_string()),
        Value::Bool(b) => Value::String(b.to_string()),
        Value::List(l) => Value::String(format!("{:?}", l)),
        Value::Record(r) => Value::String(format!("{:?}", r)),
        _ => panic!("unable to convert type to string"),
    }
}

fn json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Value::Int(n.as_i64().expect("expected i64 format"))
            } else if n.is_u64() {
                Value::Int(n.as_i64().expect("expected u64 format"))
            } else if n.is_f64() {
                Value::Float(n.as_f64().expect("expected f64 format"))
            } else {
                panic!("unexpected number format")
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
