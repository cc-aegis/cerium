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

fn sqrt(base: f16) -> f16 {
    let approx = (base alias u16 + 15360) alias f16 >> 1;
    (base / approx + approx) * 0.5
}

    ";
    let mut lexer = lexer::Lexer::new(code);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}