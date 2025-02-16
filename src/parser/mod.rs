use std::iter::Peekable;
use std::ops::Range;
use crate::error::CompilerError;
use crate::lexer::{Lexer, Token};
use crate::parser::ast::{ArrayAccess, Definition, Expression, FieldAccess, Function, Program, Qualifier, Scope};
use crate::parser::cerium_type::CeriumType;

mod ast;
mod cerium_type;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

#[derive(Clone, Debug)]
pub struct UnexpectedTokenError {
    pub range: Range<usize>,
    pub found: Token,
}

/*
 ;
 =
 && ||
 < > <= >= != ==
 * /
 + -
 & | ^ << >>
 ! & *
 alias as
 () a[b] a.b {} x

 let y: i16* = &x alias i16
 */

macro_rules! expect_token {
    ($lexer:expr, $pattern:pat, $result:expr) => {
        match $lexer.next() {
            Some(Ok($pattern)) => $result,
            Some(Ok((range, token))) => return Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found: token,
            })),
            Some(Err(e)) => return Err(e),
            None => return Err(CompilerError::MissingTokenError),
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer.peekable() }
    }

    pub fn parse(&mut self) -> Result<Program, CompilerError> {
        let mut definitions = Vec::new();
        while let Some(token) = self.parse_definition() {
            definitions.push(token?);
        }
        Ok(Program { definitions })
    }
}

impl Parser<'_> {
    fn parse_qualifier(&mut self) -> Result<Qualifier, CompilerError> {
        let mut result = vec![expect_token!(self.lexer, (_, Token::Ident(ident)), ident)];
        while self.lexer.next_if(|t| matches!(t, Ok((_, Token::Scope)))).is_some() {
            result.push(expect_token!(self.lexer, (_, Token::Ident(ident)), ident));
        }
        Ok(Qualifier { names: result })
    }

    fn parse_type(&mut self) -> Result<CeriumType, CompilerError> {
        //fn(..type)->type fn(..type) &type [type] [type; N] i16 u16 f16 bool any S S<..type>
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (_, Token::I16) => Ok(CeriumType::I16),
            (_, Token::U16) => Ok(CeriumType::U16),
            (_, Token::F16) => Ok(CeriumType::F16),
            (_, Token::Bool) => Ok(CeriumType::Bool),
            (_, Token::Any) => Ok(CeriumType::Any),
            (_, Token::Ident(ident)) => Ok(CeriumType::Struct(ident, Vec::new())),
            (_, Token::Ampersand) => Ok(CeriumType::Pointer(Box::new(self.parse_type()?))),
            (_, Token::Fn) => {
                expect_token!(self.lexer, (_, Token::LParen), {});
                let mut param_types = Vec::new();
                while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                    let param_type = self.parse_type()?;
                    param_types.push(param_type);
                    if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                        break;
                    }
                }
                expect_token!(self.lexer, (_, Token::RParen), {});
                let return_type = if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))).is_some() {
                    Some(Box::new(self.parse_type()?))
                } else {
                    None
                };
                Ok(CeriumType::Function(param_types, return_type))

            },
            (range, found) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found,
            }))
        }
    }

    fn parse_function(&mut self) -> Result<Definition, CompilerError> {
        expect_token!(self.lexer, (_, Token::Fn), {});
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, (_, Token::LParen), {});
        let mut parameters = Vec::new();
        while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
            let param_name = expect_token!(self.lexer, (_, Token::Ident(ident)), ident);
            expect_token!(self.lexer, (_, Token::LParen), {});
            let param_type = self.parse_type()?;
            parameters.push((param_name, param_type));
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                break;
            }
        }
        expect_token!(self.lexer, (_, Token::RParen), {});
        let return_type = match self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))) {
            Some(_) => Some(self.parse_type()?),
            None => None,
        };
        let body = Box::new(self.parse_scope()?);
        Ok(Definition::Function(Function {
            name,
            parameters,
            return_type,
            body,
        }))
    }

    fn parse_definition(&mut self) -> Option<Result<Definition, CompilerError>> {
        match self.lexer.peek()? {
            Ok((_, Token::Fn)) => Some(self.parse_function()),
            Ok((range, token)) => {
                let (range, token) = self.lexer.next().unwrap().unwrap();
                Some(Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError { range, found: token })))
            }
            Err(err) => Some(Err(err.clone())),
        }
    }
}

/*
const   Number etc
var     Qualifier
call    Qualifier LParen [Expression Comma]* Expression? RParen
expr    AdressableValue | Assignment
scope   LBrace [Expression Comma]* Expression? RBrace
assign  Value Assign Expression
decl    Let Qualifier Assign Expression
if      If Expression Scope [Else IfStatement | Scope]?
for     For Qualifier [Assign Expression]? [To Expression]? [Step Expression]? Scope
loop    Loop Scope
while   While Expression Scope
andor   Compare [And | Or Compare]*
compare MulDiv [Less | LessThan | .. MulDiv]*
muldiv  PlusMinus "
plusmin Bitwise "
bitwise Prefix "
prefix  [Ampersand | Asterisk | Bang]* Postfix
postfix Value [[As Type] | [Alias Type]]*
value   [LParen Expression RParen] | [Value LBracket Expression RBracket] | [Value Dot Ident] | Scope | Declaration | IfStatement | ForLoop | Loop | WhileLoop | Const | Ident

*thing.field[3] = 4
 */
impl Parser<'_> {
    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_parens(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_let(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_if(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_for(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_loop(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_while(&mut self) -> Result<Expression, CompilerError> {
        todo!()
    }

    fn parse_value(&mut self) -> Result<Expression, CompilerError> {
        let mut result = match self.lexer.peek().ok_or(CompilerError::MissingTokenError)? {
            Ok((_, Token::LBrace)) => self.parse_scope(),
            Ok((_, Token::LParen)) => self.parse_parens(),
            Ok((_, Token::Let)) => self.parse_let(),
            Ok((_, Token::If)) => self.parse_if(),
            Ok((_, Token::For)) => self.parse_for(),
            Ok((_, Token::Loop)) => self.parse_loop(),
            Ok((_, Token::While)) => self.parse_while(),
            Ok((_, Token::Integer(_) | Token::Float(_) | Token::True | Token::False | Token::Nullptr)) => self.parse_constant(),
            Ok((_, Token::Ident(_))) => self.parse_variable(),
            Ok((range, token)) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError { range: range.clone(), found: token.clone() })),
            Err(err) => Err(err.clone()),
        }?;

        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Dot)))).is_some() {
                let (end, field) = expect_token!(self.lexer, (range, Token::Ident(ident)), (range.end, ident));
                result = Expression::FieldAccess(result.range().start .. end, FieldAccess {
                    base: Box::new(result),
                    field,
                });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LBracket)))).is_some() {
                let index = self.parse_expression()?;
                result = Expression::ArrayAccess(result.range().start .. index.range().end, ArrayAccess {
                    base: Box::new(result),
                    index: Box::new(index),
                });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_constant(&mut self) -> Result<Expression, CompilerError> {
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (range, Token::Integer(i)) => Ok(Expression::Integer(range, i)),
            (range, Token::Float(f)) => Ok(Expression::Float(range, f)),
            (range, Token::True) => Ok(Expression::Boolean(range, true)),
            (range, Token::False) => Ok(Expression::Boolean(range, false)),
            (range, Token::Nullptr) => Ok(Expression::Nullptr(range)),
            (range, token) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found: token,
            }))
        }
    }

    fn parse_variable(&mut self) -> Result<Expression, CompilerError> {
        let (range, name) = expect_token!(self.lexer, (range, Token::Ident(ident)), (range, ident));
        Ok(Expression::Variable(range, name))
    }

    fn parse_scope(&mut self) -> Result<Expression, CompilerError> {
        let start = expect_token!(self.lexer, (range, Token::LBrace), { range.start });
        let mut instructions = Vec::new();
        let value = loop {
            if matches!(self.lexer.peek(), Some(Ok((_, Token::RBrace)))) {
                break None;
            }

            instructions.push(self.parse_expression()?);

            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Semicolon)))).is_none() {
                break Some(Box::new(instructions.pop().unwrap()));
            }
        };

        let end = expect_token!(self.lexer, (range, Token::RBrace), { range.end });

        Ok(Expression::Scope(start..end, Scope {
            instructions,
            value,
        }))
    }
}