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
                    Value::Number(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::String(s) => print!("{}", s),
                    Value::List(l) => println!("{:#?}", l),
                }
            }

            if name == "println" {
                println!();
            }

            Value::Bool(true)
        }
        "number" => match &args[0] {
            Value::String(s) => Value::Number(
                s.trim_ascii()
                    .parse()
                    .expect("number expects a valid number string"),
            ),
            _ => panic!("number expects a string"),
        },
        "len" => match &args[0] {
            Value::List(v) => Value::Number(v.len() as i64),
            Value::String(s) => Value::Number(s.len() as i64),
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
            Value::Number(n) => Value::String(n.to_string()),
            _ => panic!("string expects a number"),
        },
        _ => panic!("unknown function {}", name),
    }
}
