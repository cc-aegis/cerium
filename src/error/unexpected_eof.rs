use std::fmt::{Display, Formatter};
use std::ops::Not;
use colored::{Color, Colorize};
use crate::error::{highlight_lines, lines_within_range, CompilerError};

#[derive(Debug)]
pub enum Expected {
    Character(char),
    Literal,
    Number,
    Identifier,
}

impl Display for Expected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expected::Character(c) => write!(f, "'{}'", *c),
            Expected::Literal => f.write_str("literal"),
            Expected::Number => f.write_str("number"),
            Expected::Identifier => f.write_str("identifier"),
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedEof {
    pub expected: Expected,
}

impl CompilerError for UnexpectedEof {
    fn format(&self, code: &str) -> String {
        let idx = code
            .char_indices()
            .flat_map(|(idx, c)| c.is_whitespace().not().then(|| idx))
            .last()
            .unwrap_or(code.len());
        let lines = lines_within_range(code, idx..idx + 1);
        let underlined = highlight_lines(&lines);
        format!(
            "{0}{1}\n{underlined}",
            "Unexpected Eof Error".color(Color::Red),
            format!(": expected {}, found nothing", self.expected).color(Color::BrightWhite),
        )
    }
}