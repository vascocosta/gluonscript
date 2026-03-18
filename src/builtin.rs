use std::io;

use crate::ast::Value;

pub fn call_builtin(name: &str, args: &[Value]) -> Value {
    match name {
        "input" => {
            let mut buf: String = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            Value::String(buf)
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
            _ => panic!("unable to convert type to string"),
        },
        _ => panic!("unknown function {}", name),
    }
}
