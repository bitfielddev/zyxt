mod interpreter;
mod lexer;
mod parser;
mod type_check;

use std::{error::Error, fmt::Debug, process::exit};

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

pub type ZResult<T> = color_eyre::Result<T, ZError>;

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
                let mut contents = if let Some(input) = get_input(filename)? {
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
                let end_line = (span.end_pos.line - 1 + 3).min(contents.len());

                let start_vec = contents[span.start_pos.line - 1]
                    .chars()
                    .collect::<Vec<_>>();
                contents[span.start_pos.line - 1] = start_vec[..span.start_pos.column - 1]
                    .iter()
                    .copied()
                    .chain("\u{001b}[0;1;4;31m".chars())
                    .chain(start_vec[span.start_pos.column - 1..].iter().copied())
                    .join("");

                let end_vec = contents[span.end_pos.line - 1].chars().collect::<Vec<_>>();
                contents[span.end_pos.line - 1] = end_vec[..span.end_pos.column - 1]
                    .iter()
                    .copied()
                    .chain("\u{001b}[0;37;2m".chars())
                    .chain(end_vec[span.end_pos.column - 1..].iter().copied())
                    .join("");

                let surrounding = contents[start_line..=end_line].join("\n");

                Ok(format!("{pos}\n{}", surrounding.white().dimmed()))
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
