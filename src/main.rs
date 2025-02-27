#![feature(result_flattening)]
#![allow(unused)]
extern crate core;

use crate::compiler::compile;
use crate::error::CompilerError;

pub mod lexer;
pub mod error;
mod parser;
mod compiler;

fn main() {
    let code = include_str!("../cerium/simple.cer");
    //println!("{code}");
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    match parser.parse() {
        Ok(program) => {
            let _ = dbg!(compile(program));
        },
        Err(err) => println!("{}", err.format(code)),
    }
}