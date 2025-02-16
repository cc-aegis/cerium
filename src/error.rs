use std::ops::Range;
use crate::lexer::error::SyntaxError;
use crate::parser::UnexpectedTokenError;

#[derive(Clone, Debug)]
pub enum CompilerError {
    SyntaxError(SyntaxError),
    UnexpectedTokenError(UnexpectedTokenError),
    MissingTokenError,
}

fn str_lines_within_range(src: &str, range: Range<usize>) -> Vec<(usize, Range<usize>, &str)> {
    let mut result = Vec::new();
    let mut offset = 0;
    for (line_num, line) in src.lines().enumerate() {
        let start = range.start.saturating_sub(offset);
        let end = range.end.saturating_sub(offset).min(line.len());

        if start < end {
            result.push((line_num, start..end, line));
        }

        offset += line.len();
    }
    result
}

fn format_underline(underlines: Vec<(usize, Range<usize>, &str)>) -> String {
    underlines
        .into_iter()
        .map(|(line_num, Range { start, end }, line)| {
            format!("{line_num:0>5}: {line}\n       {}{}", " ".repeat(start), "^".repeat(end - start))
        })
        .collect()
}

impl CompilerError {
    pub fn format(&self, src: &str) -> String {
        match self {
            CompilerError::SyntaxError(SyntaxError { char_idx, found }) => {
                let lines = str_lines_within_range(src, *char_idx..*char_idx + 1);
                let underlined = format_underline(lines);
                format!("Syntax Error: found unexpected character '{found}'\n{underlined}")
            },
            CompilerError::UnexpectedTokenError(UnexpectedTokenError { range, found }) => {
                let lines = str_lines_within_range(src, range.clone());
                let underlined = format_underline(lines);
                format!("Syntax Error: found unexpected token '{found:?}'\n{underlined}")
            },
            CompilerError::MissingTokenError => {
                let (line_num, line) = src.lines().enumerate().last().unwrap();
                let range = line.len()..line.len()+1;
                let underlined = format_underline(vec![(line_num, range, line)]);
                format!("Syntax Error: unexpected ending\n{underlined}")
            }
        }
    }
}