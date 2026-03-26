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
                let source = fs::read_to_string(s).map_err(|_| RuntimeError {
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

pub fn len(args: Vec<Value>) -> Result<Value, RuntimeError> {
    match &args[0] {
        Value::List(v) => Ok(Value::Int(v.len() as i64)),
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(RuntimeError {
            message: "len(): unsuported type".to_string(),
        }),
    }
}
