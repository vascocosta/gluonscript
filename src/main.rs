mod ast;
mod builtin;
mod lexer;
mod operators;
mod parser;
mod program;

use lexer::Lexer;
use parser::Parser;
use std::{env, fs, process};

fn main() {
    let mut params = env::args();

    let source_file = params.nth(1).unwrap_or_else(|| {
        eprintln!("Usage: gluonscript <source_file>");
        process::exit(1);
    });

    let source = fs::read_to_string(source_file).unwrap_or_else(|_| {
        eprintln!("Could not open source file.");
        process::exit(1);
    });

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser { tokens, pos: 0 };
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(e) => {
            eprintln!(
                "Error parsing program: {} at position: {}",
                e.message, e.pos
            );
            process::exit(1);
        }
    };

    program.run();
}
