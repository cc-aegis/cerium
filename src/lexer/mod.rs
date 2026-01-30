mod token;

use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use std::str::Chars;
use crate::error::CompilerError;
use crate::error::invalid_character::InvalidCharacter;
use crate::error::invalid_character_literal_length::InvalidCharacterLiteralLength;
use crate::error::unexpected_character::UnexpectedCharacter;
use crate::error::unexpected_eof::{Expected, UnexpectedEof};
use crate::lexer::token::Token;

pub struct Lexer<'a> {
    code: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Lexer { code: code.chars().enumerate().peekable() }
    }

    fn skip_whitespace(&mut self) {
        while self.code.next_if(|(_, c)| c.is_whitespace()).is_some() {}
    }

    fn parse_literal(&mut self, delim: char) -> Result<String, Box<dyn CompilerError>> {
        let mut literal = String::new();


        let mut escaped = false;
        loop {
            match self.code.next() {
                None => return Err(Box::new(UnexpectedEof { expected: Expected::Literal })),
                Some((_, c)) if escaped => {
                    literal.push(match c {
                        '0' => '\0',
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        c => c,
                    });
                    escaped = false;
                },
                Some((_, c)) if c == delim => break,
                Some((_, '\\')) => {
                    escaped = true;
                },
                Some((_, c)) => {
                    literal.push(c);
                },
            };
        }

        Ok(literal)
    }

    fn parse_char(&mut self) -> Result<(Range<usize>, Token), Box<dyn CompilerError>> {
        let start = match self.code.next() {
            None => Err::<usize, Box<dyn CompilerError>>(Box::new(UnexpectedEof { expected: Expected::Character('\'') })),
            Some((start, '\'')) => Ok(start),
            Some((idx, actual)) => Err::<usize, Box<dyn CompilerError>>(Box::new(UnexpectedCharacter {
                expected: '\'',
                actual,
                idx,
            }))
        }?;

        let content = self.parse_literal('\'')?;

        let end = match self.code.next() {
            None => return Err(Box::new(UnexpectedEof { expected: Expected::Character('\'') })),
            Some((end, '\'')) => end + 1,
            Some((idx, actual)) => return Err(Box::new(UnexpectedCharacter {
                expected: '\'',
                actual,
                idx,
            }))
        };

        match content.chars().collect::<Vec<char>>().as_slice() {
            [c] => Ok((start..end, Token::CharLiteral(*c))),
            _ => Err(Box::new(InvalidCharacterLiteralLength {
                indices: start..end,
                literal: content,
            }))
        }
    }

    fn parse_string(&mut self) -> Result<(Range<usize>, Token), Box<dyn CompilerError>> {
        let start = match self.code.next() {
            None => Err::<usize, Box<dyn CompilerError>>(Box::new(UnexpectedEof { expected: Expected::Character('"') })),
            Some((start, '"')) => Ok(start),
            Some((idx, actual)) => Err::<usize, Box<dyn CompilerError>>(Box::new(UnexpectedCharacter {
                expected: '"',
                actual,
                idx,
            }))
        }?;

        let content = self.parse_literal('\'')?;

        let end = match self.code.next() {
            None => return Err(Box::new(UnexpectedEof { expected: Expected::Character('"') })),
            Some((end, '"')) => end + 1,
            Some((idx, actual)) => return Err(Box::new(UnexpectedCharacter {
                expected: '"',
                actual,
                idx,
            }))
        };

        Ok((start..end, Token::String(content)))
    }

    fn parse_number(&mut self) -> Result<(Range<usize>, Token), Box<dyn CompilerError>> {
        let start = self.code.peek().ok_or(UnexpectedEof {
            expected: Expected::Number,
        })?.0;

        let sign = self.code
            .next_if(|(_, c)| *c == '+' || *c == '-')
            .map(|(_, c)| c == '+');

        if self.code.peek().is_none_or(|(_, c)| !c.is_ascii_digit()) && let Some(sign) = sign {
            return if sign {
                Ok((start..start + 1, Token::Plus))
            } else {
                if self.code.next_if(|(_, c)| *c == '>').is_some() {
                    Ok((start..start + 2, Token::Arrow))
                } else {
                    Ok((start..start + 1, Token::Minus))
                }
            }
        }

        let mut pre_comma = 0;

        let mut end = start;


        while let Some((idx, char)) = self.code.next_if(|(_, c)| c.is_ascii_digit() || *c == '_') {
            if let Some(digit) = char.to_digit(10) {
                pre_comma = pre_comma * 10 + digit;
            }
            end = idx + 1;
        }

        if self.code.next_if(|(_, c)| *c == '.').is_none() {
            if let Some(sign) = sign {
                let value = pre_comma as i16 * if sign { 1 } else { -1 };
                Ok((start..end, Token::SignedInt(value)))
            } else {
                Ok((start..end, Token::UnsignedInt(pre_comma as u16)))
            }
        } else {
            let mut num = pre_comma as f32;
            let mut m = 1.0;

            while let Some((idx, char)) = self.code.next_if(|(_, c)| c.is_ascii_digit() || *c == '_') {
                if let Some(digit) = char.to_digit(10) {
                    m *= 0.1;
                    num += digit as f32 * m;
                }
                end = idx + 1;
            }

            Ok((start..end, Token::Float(num)))
        }
    }

    fn parse_ident(&mut self) -> Option<Result<(Range<usize>, Token), Box<dyn CompilerError>>> {
        let mut word = String::new();
        let start = self.code.peek()?.0;

        while let Some((_, c)) = self.code.next_if(|(_, c)| c.is_ascii_alphanumeric() || *c == '_') {
            word.push(c);
        }

        let end = start + word.len();

        let token = match word.as_str() {
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
            "while" => Token::While,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "iter" => Token::Iter,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(word),
        };

        Some(Ok((start..end, token)))
    }

    fn parse_operator(&mut self) -> Option<Result<(Range<usize>, Token), Box<dyn CompilerError>>> {
        // TODO: does this really need to return option
        let (idx, next) = self.code.next()?;

        let (len, token) = match next {
            '+' => (1, Token::Plus),
            '-' => if self.code.next_if(|(_, c)| *c == '>').is_some() {
                (2, Token::Arrow)
            } else if self.code.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                todo!()
            } else {
                (1, Token::Minus)
            },
            '*' => (1, Token::Asterisk),
            '/' => if self.code.next_if(|(_, c)| *c == '/').is_some() {
                while self.code.next_if(|(_, c)| *c != '\n').is_some() {}
                return self.next();
            } else {
                (1, Token::Slash)
            },
            '!' => match self.code.next_if(|(_, c)| *c == '=') {
                Some(_) => (2, Token::NotEquals),
                None => (1, Token::Bang),
            },
            '(' => (1, Token::LParen),
            ')' => (1, Token::RParen),
            '[' => (1, Token::LBracket),
            ']' => (1, Token::RBracket),
            '{' => (1, Token::LBrace),
            '}' => (1, Token::RBrace),
            ':' => match self.code.next_if(|(_, c)| *c == ':') {
                Some(_) => (2, Token::Scope),
                None => (1, Token::Colon),
            },
            ';' => (1, Token::Semicolon),
            ',' => (1, Token::Comma),
            '.' => (1, Token::Dot),
            '=' => match self.code.next_if(|(_, c)| *c == '=') {
                Some(_) => (2, Token::Equals),
                None => (1, Token::Assign),
            },
            '<' => match self.code.next_if(|(_, c)| *c == '=' || *c == '<') {
                Some((_, '=')) => (2, Token::LessThanEquals),
                Some(_) => (2, Token::LShift),
                None => (1, Token::LessThan),
            },
            '>' => match self.code.next_if(|(_, c)| *c == '=' || *c == '>') {
                Some((_, '=')) => (2, Token::GreaterThanEquals),
                Some(_) => (2, Token::RShift),
                None => (1, Token::GreaterThan),
            },
            '|' => match self.code.next_if(|(_, c)| *c == '|') {
                Some(_) => (2, Token::Or),
                None => (1, Token::Pipe),
            },
            '&' => match self.code.next_if(|(_, c)| *c == '&') {
                Some(_) => (2, Token::And),
                None => (1, Token::Ampersand),
            },
            '^' => (1, Token::Circumflex),
            c => return Some(Err(Box::new(InvalidCharacter {
                found: c,
                idx,
            }))),
        };
        Some(Ok((idx..idx + len, token)))
    }

    pub fn next(&mut self) -> Option<Result<(Range<usize>, Token), Box<dyn CompilerError>>> {
        self.skip_whitespace();
        match self.code.peek() {
            Some((_, c)) if c.is_ascii_digit() => Some(self.parse_number()),
            Some((_, c)) if c.is_ascii_alphabetic() || *c == '_' => self.parse_ident(),
            Some((_, '\'')) => Some(self.parse_char()),
            Some((_, '"')) => Some(self.parse_string()),
            Some(_) => self.parse_operator(),
            None => None,
        }
    }

    /**
    same as Lexer::next, but will parse + and - as prefixes for numbers
    */
    pub fn next_signed(&mut self) -> Option<Result<(Range<usize>, Token), Box<dyn CompilerError>>> {
        self.skip_whitespace();
        match self.code.peek() {
            Some((_, c)) if c.is_ascii_digit() || *c == '+' || *c == '-' => Some(self.parse_number()),
            Some((_, c)) if c.is_ascii_alphabetic() || *c == '_' => self.parse_ident(),
            Some((_, '\'')) => Some(self.parse_char()),
            Some((_, '"')) => Some(self.parse_string()),
            Some(_) => self.parse_operator(),
            None => None,
        }
    }


    /**
    parse assembly lines
    */
    pub fn next_asm(&mut self) {
        todo!();
    }
}