use std::ops::Range;
use colored::{Color, Colorize};
use crate::error::{highlight_lines, lines_within_range, CompilerError};

#[derive(Debug)]
pub struct InvalidCharacterLiteralLength {
    pub indices: Range<usize>,
    pub literal: String,
}

impl CompilerError for InvalidCharacterLiteralLength {
    fn format(&self, code: &str) -> String {
        let lines = lines_within_range(code, self.indices.clone());
        let underlined = highlight_lines(&lines);
        format!(
            "{0}{1}\n{underlined}",
            "Invalid Character Literal Length Error".color(Color::Red),
            format!(": character literal '{}' has invalid length {}", self.literal, self.literal.len()).color(Color::BrightWhite),
        )
    }
}