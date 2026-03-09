mod eval;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = "
        x = 10

        while x > 10 {
            x = x - 1
        }

        x
    ";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser { tokens, pos: 0 };

    let program = parser.parse_program();
    let result = program.run();

    println!("\nProgram result: {:?}", result);
}
