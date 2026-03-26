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
    match &args[0] {
        Value::String(s) => match s.as_str() {
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
    match &args[0] {
        Value::List(v) => Ok(Value::Int(v.len() as i64)),
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(RuntimeError::Message("len: unsuported type")),
    }
}
