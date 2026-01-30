use colored::{Color, Colorize};
use crate::error::{highlight_lines, lines_within_range, CompilerError};

#[derive(Debug)]
pub struct InvalidCharacter {
    pub found: char,
    pub idx: usize,
}

impl CompilerError for InvalidCharacter {
    fn format(&self, code: &str) -> String {
        let lines = lines_within_range(code, self.idx..self.idx + 1);
        let underlined = highlight_lines(&lines);
        format!(
            "{0}{1}\n{underlined}",
            "Syntax Error".color(Color::Red),
            format!(": found unexpected character '{}'", self.found).color(Color::BrightWhite),
        )
    }
}