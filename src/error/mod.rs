use std::fmt::Debug;
use std::ops::{Add, Range};
use colored::{Color, Colorize};

pub mod unexpected_character;
pub mod unexpected_eof;
pub mod invalid_character_literal_length;
pub mod invalid_character;

// TODO: consider turning this into an enum
pub trait CompilerError: Debug {
    fn format(&self, code: &str) -> String;
}

impl <'a, E: CompilerError + 'a> From<E> for Box<dyn CompilerError + 'a> {
    fn from(value: E) -> Self {
        Box::new(value)
    }
}

fn lines_within_range(code: &str, range: Range<usize>) -> Vec<(usize, &str, Range<usize>)> {
    let mut offset = 0;
    let mut result = Vec::new();
    for (line_number, line) in code.split_inclusive('\n').enumerate() {
        let length = line.len();
        let line = line.trim_end();

        let start = range.start.saturating_sub(offset);
        let end = range.end.saturating_sub(offset).min(line.len());

        if start < end {
            result.push((line_number, line, start..end));
        }

        offset += length;
    }
    result
}

fn highlight_lines(lines: &[(usize, &str, Range<usize>)]) -> String {
    lines
        .into_iter()
        .map(|(line_num, line, Range { start, end })| {
            format!(
                "{0:0>5} {3} {line}\n      {3} {1}{2}\n",
                line_num.add(1).to_string().color(Color::BrightBlue),
                " ".repeat(*start),
                "^".repeat(end - start).color(Color::Red),
                "|".color(Color::BrightBlue),
            )
        })
        .collect()
}