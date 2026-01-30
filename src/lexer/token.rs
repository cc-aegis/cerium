#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Float(f32),
    SignedInt(i16),
    UnsignedInt(u16),
    CharLiteral(char),
    String(String),
    True, False, Nullptr,
    Any, U16, I16, F16, Bool, Char,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    LShift, RShift, And, Or,
    Plus, Minus, Asterisk, Slash, Pipe,
    Loop, Break, Continue,
    Asm, Fn, Struct,
    Iter, Circumflex, Bang, Ampersand,
    Scope, Assign, Arrow,
    Let, Const, Mut,
    If, Else, For, To, DownTo, While,
    LessThan, GreaterThan, GreaterThanEquals, LessThanEquals, Equals, NotEquals,
    As, Alias,
    Dot, Comma, Colon, Semicolon,
}