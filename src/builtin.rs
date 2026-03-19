use std::{collections::HashMap, io};

use crate::ast::Value;

pub fn call_builtin(name: &str, args: &[Value]) -> Value {
    match name {
        "input" => {
            let mut buf: String = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("input expects stdin to work");

            Value::String(buf.trim_end_matches(['\n', '\r']).to_string())
        }
        "print" | "println" => {
            for a in args {
                match a {
                    Value::Int(n) => print!("{}", n),
                    Value::Float(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::String(s) => print!("{}", s),
                    Value::List(l) => print!("{:?}", l),
                    Value::Record(o) => print!("{:?}", o),
                    Value::Null => print!("Null"),
                }
            }

            if name == "println" {
                println!();
            }

            Value::Bool(true)
        }
        "int" => match &args[0] {
            Value::String(s) => Value::Int(
                s.trim_ascii()
                    .parse()
                    .expect("int expects a valid number string"),
            ),
            _ => panic!("int expects a string"),
        },
        "float" => match &args[0] {
            Value::String(s) => Value::Float(
                s.trim_ascii()
                    .parse()
                    .expect("float expects a valid number string"),
            ),
            _ => panic!("float expects a string"),
        },
        "len" => match &args[0] {
            Value::List(v) => Value::Int(v.len() as i64),
            Value::String(s) => Value::Int(s.len() as i64),
            _ => panic!("len() unsupported type"),
        },
        "append" => match &args[0] {
            Value::List(v) => {
                if args.len() != 2 {
                    panic!("append expects 2 arguments")
                }

                let mut new_list = v.clone();

                new_list.push(args[1].clone());

                Value::List(new_list)
            }
            _ => panic!("append expects a list"),
        },
        "string" => match &args[0] {
            Value::Int(n) => Value::String(n.to_string()),
            Value::Float(n) => Value::String(n.to_string()),
            Value::Bool(b) => Value::String(b.to_string()),
            Value::List(l) => Value::String(format!("{:?}", l)),
            Value::Record(r) => Value::String(format!("{:?}", r)),
            _ => panic!("unable to convert type to string"),
        },
        "json" => match &args[0] {
            Value::String(s) => {
                let parsed_json: serde_json::Value =
                    serde_json::from_str(s).expect("invalid json data");

                json_to_value(parsed_json)
            }
            _ => panic!("json expects a string argument"),
        },
        "get" => match &args[0] {
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
        },
        _ => panic!("unknown function {}", name),
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
