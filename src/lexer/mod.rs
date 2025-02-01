pub mod error;

use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use crate::error::CompilerError;
use crate::lexer::error::SyntaxError;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Integer(String),
    Float(String),
    String(String),
    Char(char),
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


pub struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { chars: input.chars().enumerate().peekable() }
    }

    fn skip_whitespace(&mut self) {
        while self.chars.next_if(|(_, c)| c.is_whitespace()).is_some() {}
    }

    fn parse_ident(&mut self) -> Option<(usize, Token)> {
        let mut result = String::new();
        let idx = self.chars.peek()?.0;
        while self.chars.peek().is_some_and(|(_, c)| c.is_alphanumeric() || *c == '_') {
            result.push(self.chars.next().unwrap().1);
        }
        let token = match result.as_str() {
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
        };
        Some((idx, token))
    }

    fn parse_number(&mut self) -> Option<(usize, Token)> {
        let idx = self.chars.peek()?.0;

        let mut result = String::new();

        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_digit()) {
            result.push(c);
        }

        Some((idx, match self.chars.peek() {
            Some((_, '.')) => {
                let _ = self.chars.next();
                while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_digit()) {
                    result.push(c);
                }
                Token::Float(result)
            }
            _ => Token::Integer(result)
        }))
    }

    /*
    Plus, Minus, Asterisk, Slash, Ampersand, Bang,
        + - * / & !
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
        ( ) [ ] { }
    Colon, Semicolon, Scope, Comma, Dot,
        : ; :: , .
    Assign, Equals, NotEquals, LessThan, LessThanEquals, GreaterThan, GreaterThanEquals,
        = == != < <= > >=

        //
     */

    fn parse_operator(&mut self) -> Result<Option<(usize, Token)>, CompilerError> {
        let Some((idx, next)) = self.chars.next() else {
            return Ok(None);
        };

        let token = match next {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => if self.chars.next_if(|(_, c)| *c == '/').is_some() {
                while self.chars.next_if(|(_, c)| *c != '\n').is_some() {}
                return self.next();
            } else {
                Token::Slash
            },
            '&' => Token::Ampersand,
            '!' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => Token::NotEquals,
                None => Token::Bang,
            },
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' => match self.chars.next_if(|(_, c)| *c == ':') {
                Some(_) => Token::Scope,
                None => Token::Colon,
            },
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '=' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => Token::Equals,
                None => Token::Assign,
            },
            '<' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => Token::LessThanEquals,
                None => Token::LessThan,
            },
            '>' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => Token::GreaterThanEquals,
                None => Token::GreaterThan,
            },
            c => return Err(CompilerError::SyntaxError(SyntaxError {
                char_idx: idx,
                found: c,
            })),
        };
        Ok(Some((idx, token)))
    }

    pub fn next(&mut self) -> Result<Option<(usize, Token)>, CompilerError> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some((_, c)) if c.is_ascii_digit() => Ok(self.parse_number()),
            Some((_, c)) if c.is_alphabetic() || *c == '_' => Ok(self.parse_ident()),
            Some(_) => self.parse_operator(),
            None => Ok(None),
        }
    }
}