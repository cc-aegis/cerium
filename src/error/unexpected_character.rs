use colored::{Color, Colorize};
use crate::error::{highlight_lines, lines_within_range, CompilerError};

#[derive(Debug)]
pub struct UnexpectedCharacter {
    pub expected: char,
    pub actual: char,
    pub idx: usize,
}

impl CompilerError for UnexpectedCharacter {
    fn format(&self, code: &str) -> String {
        let lines = lines_within_range(code, self.idx..self.idx + 1);
        let underlined = highlight_lines(&lines);
        format!(
            "{0}{1}\n{underlined}",
            "Unexpected Character Error".color(Color::Red),
            format!(": encountered character '{}', expected '{}'", self.actual, self.expected).color(Color::BrightWhite),
        )
    }
}