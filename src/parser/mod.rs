use std::iter::Peekable;
use crate::lexer::Lexer;

mod ast;
mod cerium_type;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

/*
 ;
 =
 < > <= >= != ==
 * /
 + -
 ! &
 alias as
 () [] a.b {} x

 let y: i16* = &x alias i16
 */

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer.peekable() }
    }
}