use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Ident(String),
    Integer(String),
    Float(String),
    True, False, Nullptr,
    Fn, Struct,
    Any, U16, I16, F16, Bool,
    As, Alias,
    Plus, Minus, Asterisk, Slash, Ampersand, Bang,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Colon, Semicolon, Scope, Comma, Dot,
    For, To, While, Loop, Break, Continue,
    Iter,
    Assign, Equals, NotEquals, LessThan, LessThanEquals, GreaterThan, GreaterThanEquals,
}

pub struct SyntaxError {

}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { chars: input.chars().peekable() }
    }

    fn parse_ident(&mut self) -> Token {
        let mut result = String::new();
        while self.chars.peek().is_some_and(|c| c.is_alphanumeric() || *c == '_') {
            result.push(self.chars.next().unwrap());
        }
        match result.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "nullptr" => Token::Nullptr,
            "fn" => Token::Fn,
            "struct" => Token::Struct,
            "any" => Token::Any,
            "u16" => Token::U16,
            "i16" => Token::I16,
            "f16" => Token::F16,
            "as" => Token::As,
            "alias" => Token::Alias,
            "bool" => Token::Bool,
            "for" => Token::For,
            "to" => Token::To,
            "while" => Token::While,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "iter" => Token::Iter,
            _ => Token::Ident(result),
        }
    }

    fn parse_number(&mut self) -> Token {
        let mut result = String::new();
        let mut dot_count = false;

        todo!()

    }

    pub fn next(&mut self) -> Result<Token, SyntaxError> {

    }
}