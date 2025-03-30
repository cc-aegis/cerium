use std::iter::Peekable;
use std::ops::Range;
use crate::error::CompilerError;
use crate::lexer::{Lexer, Token};
use crate::parser::ast::*;
use crate::parser::cerium_type::CeriumType;

pub mod ast;
pub mod cerium_type;

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
    fn parse_qualifier(&mut self) -> Result<RangeAnnotation<Qualifier>, CompilerError> {
        let (start, mut end, mut result) = expect_token!(self.lexer, (Range { start, end }, Token::Ident(ident)), (start, end, vec![ident]));
        while self.lexer.next_if(|t| matches!(t, Ok((_, Token::Scope)))).is_some() {
            result.push(expect_token!(self.lexer, (range, Token::Ident(ident)), {
                end = range.end;
                ident
            }));
        }
        Ok(RangeAnnotation::new(start..end, Qualifier::from(result)))
    }

    fn parse_type(&mut self) -> Result<RangeAnnotation<CeriumType>, CompilerError> {
        //fn(..type)->type fn(..type) &type [type] [type; N] i16 u16 f16 bool any S S<..type>
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (range, Token::I16) => Ok(RangeAnnotation::new(range, CeriumType::I16)),
            (range, Token::U16) => Ok(RangeAnnotation::new(range, CeriumType::U16)),
            (range, Token::F16) => Ok(RangeAnnotation::new(range, CeriumType::F16)),
            (range, Token::Bool) => Ok(RangeAnnotation::new(range, CeriumType::Bool)),
            (range, Token::Any) => Ok(RangeAnnotation::new(range, CeriumType::Any)),
            (range, Token::Ident(ident)) => Ok(RangeAnnotation::new(range, CeriumType::Struct(ident, Vec::new()))),
            (Range { start, .. }, Token::Ampersand) => {
                let RangeAnnotation { range, inner: inner_type } = self.parse_type()?;
                Ok(RangeAnnotation::new(start .. range.end, CeriumType::Pointer(Box::new(inner_type))))
            },
            (Range { start, .. }, Token::And) => {
                let RangeAnnotation { range, inner: inner_type } = self.parse_type()?;
                Ok(RangeAnnotation::new(start .. range.end, CeriumType::Pointer(Box::new(CeriumType::Pointer(Box::new(inner_type))))))
            },
            (Range { start, .. }, Token::Fn) => {
                expect_token!(self.lexer, (_, Token::LParen), {});
                let mut param_types = Vec::new();
                while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                    let param_type = self.parse_type()?.inner;
                    param_types.push(param_type);
                    if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                        break;
                    }
                }
                let mut end = expect_token!(self.lexer, (Range { end, .. }, Token::RParen), end);
                let return_type = if let Some(Ok((range, _))) = self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))) {
                    end = range.end;
                    Some(Box::new(self.parse_type()?.inner))
                } else {
                    None
                };
                Ok(RangeAnnotation::new(start .. end, CeriumType::Function(param_types, return_type)))

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
            let param_name = expect_token!(self.lexer, (range, Token::Ident(ident)), RangeAnnotation::new(range, Qualifier::from_str(ident)));
            expect_token!(self.lexer, (_, Token::Colon), {});
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
        let body = self.parse_scope()?;
        Ok(Definition::Function(Function {
            name,
            parameters,
            return_type,
            body,
        }))
    }

    fn parse_const(&mut self) -> Result<Definition, CompilerError> {
        expect_token!(self.lexer, (_, Token::Const), {});
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, (_, Token::Colon), {});
        let const_type = self.parse_type()?;
        expect_token!(self.lexer, (_, Token::Assign), {});
        let value = self.parse_expression()?;
        expect_token!(self.lexer, (_, Token::Semicolon), {});
        Ok(Definition::Const(Const {
            name,
            const_type,
            value,
        }))
    }

    fn parse_struct(&mut self) -> Result<Definition, CompilerError> {
        expect_token!(self.lexer, (_, Token::Struct), {});
        let name = self.parse_qualifier()?;
        let mut attributes = Vec::new();
        expect_token!(self.lexer, (_, Token::LBrace), {});
        loop {
            if matches!(self.lexer.peek(), Some(Ok((_, Token::RBrace)))) {
                break;
            }

            let name = expect_token!(self.lexer, (range, Token::Ident(ident)), RangeAnnotation::new(range, Qualifier::from_str(ident)));
            expect_token!(self.lexer, (_, Token::Colon), {});
            let param_type = self.parse_type()?;
            attributes.push((name, param_type));

            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                break;
            }
        }
        expect_token!(self.lexer, (_, Token::RBrace), {});
        Ok(Definition::Struct(Struct {
            name,
            attributes,
        }))
    }

    fn parse_definition(&mut self) -> Option<Result<Definition, CompilerError>> {
        match self.lexer.peek()? {
            Ok((_, Token::Fn)) => Some(self.parse_function()),
            Ok((_, Token::Struct)) => Some(self.parse_struct()),
            Ok((_, Token::Const)) => Some(self.parse_const()),
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

macro_rules! next_matches {
    ($iter:expr, $pattern:pat) => {
        $iter
            .next_if(|t| matches!(t, $pattern))
            .is_some()
    };
}

//TODO: design macros for repeatedly used logic
impl Parser<'_> {
    fn parse_expression(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        // = ||&& <><=>=!=== &|^<<>> +- */ aliasas !&*
        let target = self.parse_logical_operation()?;

        if next_matches!(self.lexer, Ok((_, Token::Assign))) {
            let value = self.parse_expression()?;
            let range = target.range.start .. value.range.end;
            let expression = Expression::Assignment(Box::new(Assignment { target, value }));
            Ok(RangeAnnotation::new(range, expression))
        } else {
            Ok(target)
        }
    }
    fn parse_logical_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut lhs = self.parse_compare_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::And))) {
                //TODO: macro that replaces lhs with stuff like below
                let rhs = self.parse_compare_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::LogicalAnd(Box::new(LogicalAnd { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Or))) {
                let rhs = self.parse_compare_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::LogicalOr(Box::new(LogicalOr { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_compare_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut lhs = self.parse_bitwise_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::LessThan))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::LessThan(Box::new(LessThan { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::LessThanEquals))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::LessThanEquals(Box::new(LessThanEquals { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::GreaterThan))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::GreaterThan(Box::new(GreaterThan { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::GreaterThanEquals))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::GreaterThanEquals(Box::new(GreaterThanEquals { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Equals))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::Equals(Box::new(Equals { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::NotEquals))) {
                let rhs = self.parse_bitwise_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::NotEquals(Box::new(NotEquals { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_bitwise_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut lhs = self.parse_dash_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::Ampersand))) {
                let rhs = self.parse_dash_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::BitwiseOr(Box::new(BitwiseOr { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Circumflex))) {
                let rhs = self.parse_dash_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::BitwiseXor(Box::new(BitwiseXor { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Pipe))) {
                let rhs = self.parse_dash_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::BitwiseAnd(Box::new(BitwiseAnd { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::LShift))) {
                let rhs = self.parse_dash_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::LeftShift(Box::new(LeftShift { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::RShift))) {
                let rhs = self.parse_dash_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::RightShift(Box::new(RightShift { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_dash_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut lhs = self.parse_point_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::Plus))) {
                let rhs = self.parse_point_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::Addition(Box::new(Addition { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Minus))) {
                let rhs = self.parse_point_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::Subtraction(Box::new(Subtraction { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_point_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut lhs = self.parse_typing_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::Asterisk))) {
                let rhs = self.parse_typing_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::Multiplication(Box::new(Multiplication { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else if next_matches!(self.lexer, Ok((_, Token::Slash))) {
                let rhs = self.parse_typing_operation()?;
                let range = lhs.range.start .. rhs.range.end;
                let expression = Expression::Division(Box::new(Division { lhs, rhs }));
                lhs = RangeAnnotation::new(range, expression);
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_typing_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let mut value = self.parse_prefix_operation()?;
        loop {
            if next_matches!(self.lexer, Ok((_, Token::Alias))) {
                let target_type = self.parse_type()?;
                let range = value.range.start .. target_type.range.end;
                let expression = Expression::TypeAlias(Box::new(TypeAlias { value, target_type}));
                value = RangeAnnotation::new(range, expression);
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::As)))).is_some() {
                todo!()
            } else {
                break Ok(value);
            }
        }
    }

    fn parse_prefix_operation(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        match self.lexer.peek().ok_or(CompilerError::MissingTokenError)? {
            Ok((_, Token::Ampersand)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = self.parse_prefix_operation()?;
                let range = range.start..inner.range.end;
                let expression = Expression::Borrow(Box::new(Borrow { inner }));
                Ok(RangeAnnotation::new(range, expression))
            },
            Ok((_, Token::Bang)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = self.parse_prefix_operation()?;
                let range = range.start..inner.range.end;
                let expression = Expression::Negation(Box::new(Negation { inner }));
                Ok(RangeAnnotation::new(range, expression))
            },
            Ok((_, Token::Asterisk)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = self.parse_prefix_operation()?;
                let range = range.start..inner.range.end;
                let expression = Expression::Deref(Box::new(Deref { inner }));
                Ok(RangeAnnotation::new(range, expression))
            },
            Ok((_, Token::Circumflex)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = self.parse_prefix_operation()?;
                let range = range.start..inner.range.end;
                let expression = Expression::Iter(Box::new(Iter { inner }));
                Ok(RangeAnnotation::new(range, expression))
            },
            Ok((_, Token::Minus)) => {
                let range = self.lexer.next().unwrap().unwrap().0;
                let inner = self.parse_prefix_operation()?;
                let range = range.start..inner.range.end;
                let expression = Expression::Inversion(Box::new(Inversion { inner }));
                Ok(RangeAnnotation::new(range, expression))
            },
            Ok(_) => self.parse_value(),
            Err(err) => Err(err.clone()),
        }
    }
}

impl Parser<'_> {
    fn parse_parens(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        expect_token!(self.lexer, (_, Token::LParen), {});
        let result = self.parse_expression();
        expect_token!(self.lexer, (_, Token::RParen), {});
        result
    }

    fn parse_let(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::Let), start);
        let name = expect_token!(self.lexer, (range, Token::Ident(name)), RangeAnnotation::new(range, Qualifier::from_str(name)));
        expect_token!(self.lexer, (_, Token::Assign), {});
        let value = self.parse_expression()?;
        if next_matches!(self.lexer, Ok((_, Token::In))) {
            let body = self.parse_expression()?;
            let range = start..body.range.end;
            Ok(RangeAnnotation::new(range, Expression::LetIn(Box::new(LetIn {
                name,
                value,
                body,
            }))))
        } else {
            let range = start..value.range.end;
            Ok(RangeAnnotation::new(range, Expression::Let(Box::new(Let {
                name,
                value,
            }))))
        }
    }

    fn parse_if(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::If), start);
        let condition = self.parse_expression()?;
        let if_branch = self.parse_scope()?;
        let (end, else_branch) = match self.lexer.next_if(|t| matches!(t, Ok((_, Token::Else)))) {
            Some(_) => {
                let else_branch = self.parse_scope()?;
                (else_branch.range.end, Some(else_branch))
            },
            None => (if_branch.range.end, None),
        };
        Ok(RangeAnnotation::new(start .. end, Expression::If(Box::new(If {
            condition,
            if_branch,
            else_branch,
        }))))
    }

    fn parse_for(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        //for <qualifier> [in | to | downto] <expr> <scope>
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::For), start);
        let var = self.parse_qualifier()?;
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (_, Token::To) => {
                let limit = self.parse_expression()?;
                let body = self.parse_scope()?;
                Ok(RangeAnnotation::new(start..body.range.end, Expression::ForTo(Box::new(ForTo {
                    var,
                    limit,
                    body,
                }))))
            },
            (_, Token::DownTo) => {
                let limit = self.parse_expression()?;
                let body = self.parse_scope()?;
                Ok(RangeAnnotation::new(start..body.range.end, Expression::ForDownTo(Box::new(ForDownTo {
                    var,
                    limit,
                    body,
                }))))
            },
            (_, Token::In) => {
                let iterator = self.parse_expression()?;
                let body = self.parse_scope()?;
                Ok(RangeAnnotation::new(start..body.range.end, Expression::ForIn(Box::new(ForIn {
                    var,
                    iterator,
                    body,
                }))))
            },
            (range, token) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found: token,
            }))
        }
    }

    fn parse_while(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::While), start);
        let condition = self.parse_expression()?;
        let body = self.parse_scope()?;
        Ok(RangeAnnotation::new(start .. body.range.end, Expression::While(Box::new(While {
            condition,
            body,
        }))))
    }

    fn parse_loop(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let start = expect_token!(self.lexer, (Range { start, .. }, Token::Loop), start);
        let body = self.parse_scope()?;
        Ok(RangeAnnotation::new(start .. body.range.end, Expression::Loop(Box::new(Loop { body }))))
    }

    fn parse_value(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
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
                let field = expect_token!(self.lexer, (range, Token::Ident(name)), RangeAnnotation::new(range, Qualifier::from_str(name)));
                result = RangeAnnotation::new(result.range.start .. field.range.end, Expression::FieldAccess(Box::new(FieldAccess {
                    base: result,
                    field,
                })));
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LBracket)))).is_some() {
                let index = self.parse_expression()?;
                let end = expect_token!(self.lexer, (Range { end, .. }, Token::RBracket), end);
                result = RangeAnnotation::new(result.range.start..end, Expression::ArrayAccess(Box::new(ArrayAccess {
                    base: result,
                    index,
                })));
            } else if self.lexer.next_if(|t| matches!(t, Ok((_, Token::LParen)))).is_some() {
                let mut params = Vec::new();
                loop {
                    if matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                        break;
                    }

                    params.push(self.parse_expression()?);

                    if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Comma)))).is_none() {
                        break;
                    }
                }
                let end = expect_token!(self.lexer, (Range { end, .. }, Token::RParen), end);
                result = RangeAnnotation::new(result.range.start..end, Expression::FunctionCall(Box::new(FunctionCall {
                    func: result,
                    params,
                })));
            } else {
                break Ok(result);
            }
        }
    }

    fn parse_constant(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        //TODO: + for signed positive
        match self.lexer.next().ok_or(CompilerError::MissingTokenError)?? {
            (range, Token::Plus) => todo!(),
            //TODO: minus?
            (range, Token::Integer(i)) => Ok(RangeAnnotation::new(range, Expression::UnsignedInteger(i as u16))),
            (range, Token::Float(f)) => Ok(RangeAnnotation::new(range, Expression::Float(f))),
            (range, Token::True) => Ok(RangeAnnotation::new(range, Expression::Boolean(true))),
            (range, Token::False) => Ok(RangeAnnotation::new(range, Expression::Boolean(false))),
            (range, Token::Nullptr) => Ok(RangeAnnotation::new(range, Expression::Nullptr)),
            (range, token) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                found: token,
            }))
        }
    }

    fn parse_variable(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let RangeAnnotation { range, inner: qualifier } = self.parse_qualifier()?;
        Ok(RangeAnnotation::new(range, Expression::Variable(qualifier)))
    }

    fn parse_scope(&mut self) -> Result<RangeAnnotation<Expression>, CompilerError> {
        let start = expect_token!(self.lexer, (range, Token::LBrace), { range.start });
        let mut instructions = Vec::new();
        let value = loop {
            if matches!(self.lexer.peek(), Some(Ok((_, Token::RBrace)))) {
                break None;
            }

            instructions.push(self.parse_expression()?);

            if self.lexer.next_if(|t| matches!(t, Ok((_, Token::Semicolon)))).is_none() {
                break Some(instructions.pop().unwrap());
            }
        };

        let end = expect_token!(self.lexer, (range, Token::RBrace), { range.end });

        Ok(RangeAnnotation::new(start..end, Expression::Scope(Box::new(Scope {
            instructions,
            value,
        }))))
    }
}