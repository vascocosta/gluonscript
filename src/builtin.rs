mod conv;
mod env;
mod http;
mod io;
mod json;
mod strings;

use std::fs;

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

pub fn import(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match args.first() {
        Some(Value::String(s)) => match s.as_str() {
            "conv" => conv::module(),
            "env" => env::module(),
            "http" => http::module(),
            "io" => io::module(),
            "json" => json::module(),
            "strings" => strings::module(),

            _ => {
                let source = fs::read_to_string(s)
                    .map_err(|_| RuntimeError::Message("import: could not read source file"))?;

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
