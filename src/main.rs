extern crate core;

pub mod lexer;
pub mod error;
mod parser;

fn main() {
    let code = "
// function
fn main() {
    let x: u16* = &[1, 2, 3, 4];
    io::print(x);
}

    ";
    let mut lexer = lexer::Lexer::new(code);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}