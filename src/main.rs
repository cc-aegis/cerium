#![feature(result_flattening)]
extern crate core;

use crate::error::CompilerError;

pub mod lexer;
pub mod error;
mod parser;

fn main() {
    let code = include_str!("../cerium/simple.cer");
    println!("{code}");
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    match parser.parse() {
        Ok(program) => {
            let _ = dbg!(program);
        },
        Err(err) => println!("{}", err.format(code)),
    }
}