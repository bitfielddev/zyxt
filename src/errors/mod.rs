mod interpreter;
mod lexer;
mod parser;
mod type_check;

use std::{fmt::Debug, process::exit};

use backtrace::Backtrace;
use color_eyre::{Report, Result};
use itertools::Itertools;
use owo_colors::OwoColorize;
use tracing::{debug, warn};
use tracing_error::SpanTrace;

use crate::{
    file_importer::get_input,
    types::position::{GetSpan, Span},
};

pub type ZResult<T> = Result<T, ZError>;

#[derive(Clone, Debug)]
pub struct ZError {
    pub pos: Vec<Span>,
    pub code: &'static str,
    pub message: String,
    pub span_trace: Box<SpanTrace>,
    pub back_trace: Box<Backtrace>,
}

impl ZError {
    #[must_use]
    #[tracing::instrument(skip_all)]
    pub fn new(code: &'static str, message: String) -> Self {
        Self {
            code,
            message,
            pos: Vec::new(),
            span_trace: Box::new(SpanTrace::capture()),
            back_trace: Box::new(Backtrace::new()),
        }
    }
    #[tracing::instrument(skip_all)]
    pub fn get_surrounding_text(&self) -> Result<String> {
        Ok(self
            .pos
            .iter()
            .map(|span| {
                let pos = format!(" {} ", span.start_pos).bold().on_red().to_string();
                let filename = if let Some(filename) = &span.start_pos.filename {
                    filename.as_ref()
                } else {
                    warn!("Could not find filename");
                    return Result::<_, Report>::Ok(pos);
                };
                let contents = if let Some(input) = get_input(filename)? {
                    input
                        .split('\n')
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                } else {
                    warn!("Could not get file");
                    return Ok(pos);
                };
                debug!(start = ?span.start_pos, end = ?span.end_pos, "Generating surrounding text");
                let start_line = (span.start_pos.line - 1).saturating_sub(2);
                let end_line = (span.end_pos.line - 1 + 3).min(contents.len() - 1);
                let mut first_highlighted_line = String::new();
                let mut last_highlighted_line = String::new();

                let mut top_surroundings = contents[start_line..span.start_pos.line].to_owned();
                if let Some(last) = top_surroundings.last_mut() {
                    let new_last = last.chars().collect::<Vec<_>>();
                    let split = new_last.split_at(span.start_pos.column - 1);
                    *last = split.0.iter().join("");
                    first_highlighted_line = split.1.iter().join("");
                }
                let top_surroundings = top_surroundings.join("\n");

                let mut bottom_surroundings =
                    contents[(span.end_pos.line - 1)..=end_line].to_owned();
                if let Some(first) = bottom_surroundings.first_mut() {
                    let new_first = first.chars().collect::<Vec<_>>();
                    let split = new_first.split_at(span.end_pos.column.min(new_first.len()));
                    *first = split.1.iter().join("");
                    last_highlighted_line = split.0.iter().join("");
                }
                let bottom_surroundings = bottom_surroundings.join("\n");

                let highlighted = if span.start_pos.line == span.end_pos.line {
                    last_highlighted_line
                        .split_at(span.start_pos.column - 1)
                        .1
                        .to_owned()
                } else {
                    let highlighted =
                        contents[(span.start_pos.line - 1)..span.end_pos.line].join("\n");
                    format!("{first_highlighted_line}\n{highlighted}\n{last_highlighted_line}")
                };

                Ok(format!(
                    "{pos}\n{}{}{}",
                    top_surroundings.white().dimmed(),
                    highlighted.bright_red().underline(),
                    bottom_surroundings.white().dimmed()
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }
    pub fn print_exit(self) -> ! {
        self.print().unwrap();
        exit(1)
    }
    pub fn print(&self) -> Result<()> {
        println!("{}", self.get_surrounding_text()?);
        // TODO flag for showing span_trace
        println!("Span trace:\n{}", self.span_trace);
        println!("Back trace:\n{:#?}", self.back_trace);
        println!(
            " Error {}{} ",
            self.code.black().on_yellow(),
            format!(" {}", self.message).bold().red(),
        );
        Ok(())
    }
    #[must_use]
    pub fn with_span(mut self, span: impl GetSpan) -> Self {
        self.pos = if let Some(span) = span.span() {
            vec![span]
        } else {
            vec![]
        };
        self
    }
}

pub trait ToZResult<T> {
    fn z(self) -> ZResult<T>;
}

impl<T, E: Debug> ToZResult<T> for Result<T, E> {
    fn z(self) -> ZResult<T> {
        self.map_err(|e| ZError::new("X001", format!("{e:?}")))
    }
}
impl<T> ToZResult<T> for Option<T> {
    fn z(self) -> ZResult<T> {
        self.ok_or_else(|| ZError::new("X002", "Returned `None`".into()))
    }
}
