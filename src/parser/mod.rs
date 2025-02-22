use std::iter::Peekable;
use std::ops::Range;
use crate::error::CompilerError;
use crate::lexer::{Lexer, Token};
use crate::parser::ast::{Addition, ArrayAccess, Assignment, BitwiseAnd, BitwiseOr, BitwiseXor, Borrow, Definition, Deref, Division, Equals, Expression, FieldAccess, Function, GreaterThan, GreaterThanEquals, LeftShift, LessThan, LessThanEquals, Let, LogicalAnd, LogicalOr, Multiplication, Negation, NotEquals, Program, Qualifier, RightShift, Scope, Subtraction, TypeAlias};
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
            Some(Ok((range, token))) => {
                #[cfg(debug_assertions)]
                panic!();
                return Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                    range,
                    found: token,
                }))
            },
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
    fn parse_qualifier(&mut self) -> Result<(Range<usize>, Qualifier), CompilerError> {
        let (start, mut end, mut result) = expect_token!(self.lexer, (Range { start, end }, Token::Ident(ident)), (start, end, vec![ident]));
        while self.lexer.next_if(|t| matches!(t, Ok((_, Token::Scope)))).is_some() {
            result.push(expect_token!(self.lexer, (range, Token::Ident(ident)), {
                end = range.end;
                ident
            }));
        }
        Ok((start..end, Qualifier { names: result }))
    }

    fn parse_type(&mut self) -> Result<(Range<usize>, CeriumType), CompilerError> {
        //fn(..type)->type fn(..type) &type [type] [type; N] i16 u16 f16 bool any S S<..type>
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (range, Token::I16) => Ok((range, CeriumType::I16)),
            (range, Token::U16) => Ok((range, CeriumType::U16)),
            (range, Token::F16) => Ok((range, CeriumType::F16)),
            (range, Token::Bool) => Ok((range, CeriumType::Bool)),
            (range, Token::Any) => Ok((range, CeriumType::Any)),
            (range, Token::Ident(ident)) => Ok((range, CeriumType::Struct(ident, Vec::new()))),
            (Range { start, .. }, Token::Ampersand) => {
                let (Range { end, .. }, inner_type) = self.parse_type()?;
                Ok((start .. end, CeriumType::Pointer(Box::new(inner_type))))
            },
            (Range { start, .. }, Token::And) => {
                let (Range { end, .. }, inner_type) = self.parse_type()?;
                Ok((start .. end, CeriumType::Pointer(Box::new(CeriumType::Pointer(Box::new(inner_type))))))
            },
            (Range { start, .. }, Token::Fn) => {
                expect_token!(self.lexer, (_, Token::LParen), {});
                let mut param_types = Vec::new();
                while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                    let param_type = self.parse_type()?.1;
                    param_types.push(param_type);
                    if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                        break;
                    }
                }
                let mut end = expect_token!(self.lexer, (Range { end, .. }, Token::RParen), end);
                let return_type = if let Some(Ok((range, _))) = self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))) {
                    end = range.end;
                    Some(Box::new(self.parse_type()?.1))
                } else {
                    None
                };
                Ok((start .. end, CeriumType::Function(param_types, return_type)))

            },
            (range, found) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found,
            }))
        }
    }

    fn parse_function(&mut self) -> Result<Definition, CompilerError> {
        expect_token!(self.lexer, (_, Token::Fn), {});
        let name = self.parse_qualifier()?.1;
        expect_token!(self.lexer, (_, Token::LParen), {});
        let mut parameters = Vec::new();
        while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
            let param_name = expect_token!(self.lexer, (_, Token::Ident(ident)), ident);
            expect_token!(self.lexer, (_, Token::Colon), {});
            let param_type = self.parse_type()?.1;
            parameters.push((param_name, param_type));
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                break;
            }
        }
        expect_token!(self.lexer, (_, Token::RParen), {});
        let return_type = match self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))) {
            Some(_) => Some(self.parse_type()?.1),
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

//TODO: design macros for repeatedly used logic
impl Parser<'_> {
    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        // = ||&& <><=>=!=== &|^<<>> +- */ aliasas !&*
        let lhs = self.parse_logical_operation()?;

        if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Assign)))).is_some() {
            let rhs = self.parse_expression()?;
            let range = lhs.range().start..rhs.range().end;
            Ok(Expression::Assignment(range, Assignment { target: Box::new(lhs), value: Box::new(rhs) }))
        } else {
            Ok(lhs)
        }
    }
    fn parse_logical_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_compare_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::And)))).is_some() {
                let rhs = self.parse_compare_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::LogicalAnd(range, LogicalAnd { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Or)))).is_some() {
                let rhs = self.parse_compare_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::LogicalOr(range, LogicalOr { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_compare_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_bitwise_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LessThan)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::LessThan(range, LessThan { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LessThanEquals)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::LessThanEquals(range, LessThanEquals { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::GreaterThan)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::GreaterThan(range, GreaterThan { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::GreaterThanEquals)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::GreaterThanEquals(range, GreaterThanEquals { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Equals)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::Equals(range, Equals { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::NotEquals)))).is_some() {
                let rhs = self.parse_bitwise_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::NotEquals(range, NotEquals { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_bitwise_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_dash_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Ampersand)))).is_some() {
                let rhs = self.parse_dash_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::BitwiseOr(range, BitwiseOr { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Circumflex)))).is_some() {
                let rhs = self.parse_dash_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::BitwiseXor(range, BitwiseXor { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Pipe)))).is_some() {
                let rhs = self.parse_dash_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::BitwiseAnd(range, BitwiseAnd { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LShift)))).is_some() {
                let rhs = self.parse_dash_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::LeftShift(range, LeftShift { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::RShift)))).is_some() {
                let rhs = self.parse_dash_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::RightShift(range, RightShift { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_dash_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_point_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Plus)))).is_some() {
                let rhs = self.parse_point_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::Addition(range, Addition { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Minus)))).is_some() {
                let rhs = self.parse_point_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::Subtraction(range, Subtraction { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_point_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_typing_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Asterisk)))).is_some() {
                let rhs = self.parse_typing_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::Multiplication(range, Multiplication { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Slash)))).is_some() {
                let rhs = self.parse_typing_operation()?;
                let range = result.range().start .. rhs.range().end;
                result = Expression::Division(range, Division { lhs: Box::new(result), rhs: Box::new(rhs) });
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_typing_operation(&mut self) -> Result<Expression, CompilerError> {
        let mut result = self.parse_prefix_operation()?;
        loop {
            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Alias)))).is_some() {
                let (range, target_type) = self.parse_type()?;
                //let range = result.range().start .. rhs.range().end;
                result = Expression::TypeAlias(range, TypeAlias {
                    value: Box::new(result),
                    target_type: Box::new(target_type),
                })
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::As)))).is_some() {
                let target_type = self.parse_type()?;
                //let range = result.range().start .. rhs.range().end;
                todo!()
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_prefix_operation(&mut self) -> Result<Expression, CompilerError> {
        match self.lexer.peek().ok_or(CompilerError::MissingTokenError)? {
            Ok((_, Token::Ampersand)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = Box::new(self.parse_prefix_operation()?);
                Ok(Expression::Borrow(range.start..inner.range().end, Borrow { inner }))
            },
            Ok((_, Token::Bang)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = Box::new(self.parse_prefix_operation()?);
                Ok(Expression::Negation(range.start..inner.range().end, Negation { inner }))
            },
            Ok((_, Token::Asterisk)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = Box::new(self.parse_prefix_operation()?);
                Ok(Expression::Deref(range.start..inner.range().end, Deref { inner }))
            },
            Ok(_) => self.parse_value(),
            Err(err) => Err(err.clone()),
        }
    }
}

impl Parser<'_> {
    fn parse_parens(&mut self) -> Result<Expression, CompilerError> {
        expect_token!(self.lexer, (_, Token::LParen), {});
        let result = self.parse_expression();
        expect_token!(self.lexer, (_, Token::RParen), {});
        result
    }

    fn parse_let(&mut self) -> Result<Expression, CompilerError> {
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::Let), start);
        let name = expect_token!(self.lexer, (_, Token::Ident(i)), i);
        expect_token!(self.lexer, (_, Token::Assign), {});
        let value = self.parse_expression()?;
        Ok(Expression::Let(start .. value.range().end, Let {
            name: Qualifier { names: vec![name] },
            value: Box::new(value),
        }))
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
        // let (range, name) = expect_token!(self.lexer, (range, Token::Ident(ident)), (range, ident));
        // Ok(Expression::Variable(range, name))
        let (range, qualifier) = self.parse_qualifier()?;
        Ok(Expression::Variable(range, qualifier))
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