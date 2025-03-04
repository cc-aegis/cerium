#![feature(result_flattening)]
#![feature(box_patterns)]
#![allow(unused)]
extern crate core;

use crate::compiler::compile;
use crate::error::CompilerError;

pub mod lexer;
pub mod error;
pub mod parser;
mod compiler;

fn main() {
    let code = include_str!("../cerium/copy.cer");
    match compile(code) {
        Ok(asm) => {
            let asm = asm
                .into_iter()
                .map(|inst| inst.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            println!("{asm}");
        },
        Err(err) => println!("{}", err.format(code)),
    }
}