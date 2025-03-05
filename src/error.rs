use std::ops::{Add, Range};
use colored::{Color, Colorize};
use crate::compiler::error::{InvalidDerefError, MismatchedAssignTypeError, MismatchedReturnTypeError};
use crate::lexer::error::SyntaxError;
use crate::parser::ast::Qualifier;
use crate::parser::cerium_type::{format_type, CeriumType};
use crate::parser::UnexpectedTokenError;

#[derive(Clone, Debug)]
pub enum CompilerError {
    SyntaxError(SyntaxError),
    UnexpectedTokenError(UnexpectedTokenError),
    MissingTokenError,
    MismatchedReturnTypeError(MismatchedReturnTypeError),
    MismatchedAssignTypeError(MismatchedAssignTypeError),
    InvalidDerefError(InvalidDerefError),
}

fn str_lines_within_range(src: &str, range: Range<usize>) -> Vec<(usize, Range<usize>, &str)> {
    let mut result = Vec::new();
    let mut offset = 0;
    for (line_num, line) in src.split_inclusive('\n').enumerate() {
        let start = range.start.saturating_sub(offset);
        let end = range.end.saturating_sub(offset).min(line.len());

        if start < end {
            result.push((line_num, start..end, line.trim_end()));
        }

        offset += line.len();
    }
    result
}

fn format_underline(underlines: Vec<(usize, Range<usize>, &str)>) -> String {
    underlines
        .into_iter()
        .map(|(line_num, Range { start, end }, line)| {
            format!(
                "{0:0>5} {3} {line}\n      {3} {1}{2}\n",
                line_num.add(1).to_string().color(Color::BrightBlue),
                " ".repeat(start),
                "^".repeat(end - start).color(Color::Red),
                "|".color(Color::BrightBlue),
            )
        })
        .collect()
}

impl CompilerError {
    pub fn format(&self, src: &str) -> String {
        match self {
            CompilerError::SyntaxError(SyntaxError { char_idx, found }) => {
                let lines = str_lines_within_range(src, *char_idx..*char_idx + 1);
                let underlined = format_underline(lines);
                format!(
                    "{0}{1}\n{underlined}",
                    "Syntax Error".color(Color::Red),
                    format!(": found unexpected character '{found}").color(Color::BrightWhite),
                )
            },
            CompilerError::UnexpectedTokenError(UnexpectedTokenError { range, found }) => {
                let lines = str_lines_within_range(src, range.clone());
                let underlined = format_underline(lines);
                format!(
                    "{0}{1}\n{underlined}",
                    "Syntax Error".color(Color::Red),
                    format!(": found unexpected token '{found:?}'").color(Color::BrightWhite),
                )
            },
            CompilerError::MissingTokenError => {
                let (line_num, line) = src.lines().enumerate().last().unwrap();
                let range = line.len()..line.len()+1;
                let underlined = format_underline(vec![(line_num, range, line)]);
                format!(
                    "{0}{1}\n{underlined}",
                    "Syntax Error".color(Color::Red),
                    ": unexpected ending".color(Color::BrightWhite),
                )
            },
            CompilerError::MismatchedReturnTypeError(MismatchedReturnTypeError { function_name, expected, actual, range }) => {
                let lines = str_lines_within_range(src, range.clone());
                let underlined = format_underline(lines);
                format!(
                    "{0}{1}\n{underlined}",
                    "Type Error".color(Color::Red),
                    format!(
                        ": function '{function_name}' has return value of type {0}, found value of type {1}",
                        format_type(expected),
                        format_type(actual),
                    ).color(Color::BrightWhite),
                )
            },
            CompilerError::MismatchedAssignTypeError(MismatchedAssignTypeError { dst_range, dst_type, src_range, src_type }) => {
                let dst_lines = format_underline(str_lines_within_range(src, dst_range.clone()));
                let src_lines = format_underline(str_lines_within_range(src, src_range.clone()));
                format!(
                    "{0}{1}\n{dst_lines}{src_lines}",
                    "Type Error".color(Color::Red),
                    format!(
                        ": wrote value of type {0} to pointer with inner type {1}",
                        format_type(src_type),
                        format_type(dst_type),
                    ).color(Color::BrightWhite),
                )
            },
            CompilerError::InvalidDerefError(InvalidDerefError { range, found }) => {
                let lines = str_lines_within_range(src, range.clone());
                let underlined = format_underline(lines);
                format!(
                    "{0}{1}\n{underlined}",
                    "Type Error".color(Color::Red),
                    format!(
                        ": attempted to deref value of type {0}, must be pointer",
                        format_type(found),
                    ).color(Color::BrightWhite),
                )
            }
        }
    }
}