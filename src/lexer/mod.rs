pub mod error;

use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::str::Chars;
use crate::error::CompilerError;
use crate::lexer::error::SyntaxError;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Integer(usize),
    Float(f32),
    String(String),
    Char(char),
    True, False, Nullptr,
    Const, Asm, Fn, Struct, Arrow,
    Any, U16, I16, F16, Bool,
    As, Alias,
    Plus, Minus, Asterisk, Slash, Ampersand, Pipe, Circumflex, Bang,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Colon, Semicolon, Scope, Comma, Dot,
    For, To, DownTo, In, While, Loop, Break, Continue, Let, If, Else,
    Iter, Step,
    Assign, Equals, NotEquals, LessThan, LessThanEquals, GreaterThan, GreaterThanEquals,
    And, Or,
    LShift, RShift,
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

    fn parse_ident(&mut self) -> Option<Result<(Range<usize>, Token), CompilerError>> {
        let mut result = String::new();
        let idx = self.chars.peek()?.0;
        while self.chars.peek().is_some_and(|(_, c)| c.is_alphanumeric() || *c == '_') {
            result.push(self.chars.next().unwrap().1);
        }
        let len = result.len();
        let token = match result.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "nullptr" => Token::Nullptr,
            "const" => Token::Const,
            "asm" => Token::Asm,
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
            "downto" => Token::DownTo,
            "in" => Token::In,
            "while" => Token::While,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "step" => Token::Step,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(result),
        };
        Some(Ok((idx..idx + len, token)))
    }

    fn parse_number(&mut self) -> Option<Result<(Range<usize>, Token), CompilerError>> {
        let idx = self.chars.peek()?.0;

        let mut result = String::new();

        while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_digit()) {
            result.push(c);
        }

        Some(Ok((idx..idx + result.len(), match self.chars.peek() {
            Some((_, '.')) => {
                result.push(self.chars.next().unwrap().1);
                while let Some((_, c)) = self.chars.next_if(|(_, c)| c.is_ascii_digit()) {
                    result.push(c);
                }
                Token::Float(result.parse().unwrap())
            }
            _ => Token::Integer(result.parse().unwrap()),
        })))
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

    fn parse_operator(&mut self) -> Option<Result<(Range<usize>, Token), CompilerError>> {
        let (idx, next) = self.chars.next()?;

        let (len, token) = match next {
            '+' => (1, Token::Plus),
            '-' => match self.chars.next_if(|(_, c)| *c == '>') {
                Some(_) => (2, Token::Arrow),
                None => (1, Token::Minus),
            },
            '*' => (1, Token::Asterisk),
            '/' => if self.chars.next_if(|(_, c)| *c == '/').is_some() {
                while self.chars.next_if(|(_, c)| *c != '\n').is_some() {}
                return self.next();
            } else {
                (1, Token::Slash)
            },
            '!' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => (2, Token::NotEquals),
                None => (1, Token::Bang),
            },
            '(' => (1, Token::LParen),
            ')' => (1, Token::RParen),
            '[' => (1, Token::LBracket),
            ']' => (1, Token::RBracket),
            '{' => (1, Token::LBrace),
            '}' => (1, Token::RBrace),
            ':' => match self.chars.next_if(|(_, c)| *c == ':') {
                Some(_) => (2, Token::Scope),
                None => (1, Token::Colon),
            },
            ';' => (1, Token::Semicolon),
            ',' => (1, Token::Comma),
            '.' => (1, Token::Dot),
            '=' => match self.chars.next_if(|(_, c)| *c == '=') {
                Some(_) => (2, Token::Equals),
                None => (1, Token::Assign),
            },
            '<' => match self.chars.next_if(|(_, c)| *c == '=' || *c == '<') {
                Some((_, '=')) => (2, Token::LessThanEquals),
                Some(_) => (2, Token::LShift),
                None => (1, Token::LessThan),
            },
            '>' => match self.chars.next_if(|(_, c)| *c == '=' || *c == '>') {
                Some((_, '=')) => (2, Token::GreaterThanEquals),
                Some(_) => (2, Token::RShift),
                None => (1, Token::GreaterThan),
            },
            '|' => match self.chars.next_if(|(_, c)| *c == '|') {
                Some(_) => (2, Token::Or),
                None => (1, Token::Pipe),
            },
            '&' => match self.chars.next_if(|(_, c)| *c == '&') {
                Some(_) => (2, Token::And),
                None => (1, Token::Ampersand),
            },
            '^' => (1, Token::Circumflex),
            c => return Some(Err(CompilerError::SyntaxError(SyntaxError {
                char_idx: idx,
                found: c,
            }))),
        };
        Some(Ok((idx..idx + len, token)))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<(Range<usize>, Token), CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some((_, c)) if c.is_ascii_digit() => self.parse_number(),
            Some((_, c)) if c.is_alphabetic() || *c == '_' => self.parse_ident(),
            Some(_) => self.parse_operator(),
            None => None,
        }
    }
}