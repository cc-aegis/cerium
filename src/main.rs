#![feature(result_flattening)]
extern crate core;

pub mod lexer;
pub mod error;
mod parser;

fn main() {
    let code = include_str!("../cerium/render.cer");
    let mut lexer = lexer::Lexer::new(code);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}