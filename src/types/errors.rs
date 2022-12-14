use std::{
    fmt::{Debug, Display},
    process::exit,
};

use backtrace::Backtrace;
use color_eyre::eyre::Result;
use itertools::Itertools;
use owo_colors::OwoColorize;
use tracing::debug;

use crate::{
    ast::{Ast, AstData},
    file_importer::get_input,
    types::{
        position::{GetSpan, Span},
        token::Keyword,
        value::Value,
    },
    Type,
};

pub type ZResult<T> = Result<T, ZError>;

#[derive(Clone, Debug)]
pub struct ZError {
    pub pos: Vec<Span>,
    pub code: &'static str,
    pub message: String,
}
impl ZError {
    /* 0. Internal errors, have to do with the compiler-interpreter itself */
    /// Rust error
    pub fn error_0_0(error: impl Display, backtrace: Backtrace) -> Self {
        ZError {
            pos: vec![],
            code: "0.0",
            message: format!("Internal error: \n{error}\n{backtrace:?}\n\nThis shouldn't happen! Open an issue on our Github repo page: [TODO]")
        }
    }

    /// No file given
    pub fn error_0_1() -> Self {
        ZError {
            pos: vec![],
            code: "0.1",
            message: "No file given".to_string(),
        }
    }

    /* 1. File and I/O errors */
    /// File does not exist
    pub fn error_1_0(filename: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "1.0",
            message: format!("File `{filename}` does not exist"),
        }
    }

    /// file cannot be opened
    pub fn error_1_1(filename: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "1.1",
            message: format!("File `{filename}` cannot be opened"),
        }
    }

    pub fn error_1_2(dirname: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "1.2",
            message: format!("Directory given (Got `{dirname}`)"),
        }
    }

    /* 2. Syntax errors */
    /// parentheses not closed properly (try swapping)
    pub fn error_2_0_0(paren1: impl Display, paren2: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.0.0",
            message: format!(
                "Parentheses `{}` and `{}` not closed properly; try swapping them",
                paren1, paren2
            ),
        }
    }
    /// parentheses not closed properly (not closed)
    pub fn error_2_0_1(paren: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.0.1",
            message: format!("Parenthesis `{paren}` not closed"),
        }
    }
    /// parentheses not closed properly (not opened)
    pub fn error_2_0_2(paren: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.0.2",
            message: format!("Parenthesis `{paren}` not opened"),
        }
    }

    /// unexpected ident (generic)
    pub fn error_2_1_0(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.0",
            message: format!("Unexpected ident `{ident}`"),
        }
    }
    /// unexpected ident (lexer didnt recognise)
    pub fn error_2_1_1(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.1",
            message: format!("Ident `{ident}` not recognised by lexer"),
        }
    }
    /// unexpected ident (dot at end of expression)
    pub fn error_2_1_2() -> Self {
        ZError {
            pos: vec![],
            code: "2.1.2",
            message: "Stray `.` at end of expression".to_string(),
        }
    }
    /// unexpected ident (binary operator at start/end of expression)
    pub fn error_2_1_3(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.3",
            message: format!(
                "Stray `{}` binary operator at start/end of expression",
                ident
            ),
        }
    }
    /// unexpected ident (unary operator at start/end of expression)
    pub fn error_2_1_4(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.4",
            message: format!(
                "Stray `{}` unary operator at start/end of expression",
                ident
            ),
        }
    }
    /// unexpected ident (declaration expr at start/end of expression)
    pub fn error_2_1_5() -> Self {
        ZError {
            pos: vec![],
            code: "2.1.5",
            message: "Stray `:=` at start/end of expression".to_string(),
        }
    }
    /// unexpected ident (non-flag between first flag and declared variable)
    pub fn error_2_1_6(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.6",
            message: format!("Stray `{ident}` between first flag and declared variable"),
        }
    }
    /// unexpected ident ('else/elif'  found after 'else' keyword)
    pub fn error_2_1_7(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.7",
            message: format!("`{ident}` detected after `else` keyword"),
        }
    }
    /// unexpected ident (block expected, not ident)
    pub fn error_2_1_8(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.8",
            message: format!("Block expected, not `{ident}`"),
        }
    }
    /// unexpected ident ('else/elif' found without 'if' keyword)
    pub fn error_2_1_9(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.9",
            message: format!("Stray `{ident}` without starting `if`"),
        }
    }
    /// unexpected ident (stray comment start / end)
    pub fn error_2_1_10(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.10",
            message: format!("Stray unclosed/unopened `{ident}`"),
        }
    }
    /// unexpected ident (must be variable)
    pub fn error_2_1_11(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.11",
            message: format!("Only variables can be deleted (Got `{ident}`)"),
        }
    }
    /// unexpected ident (cannot delete dereferenced variable)
    pub fn error_2_1_12(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.12",
            message: format!("Cannot delete dereferenced variable (Got `{ident}`)"),
        }
    }
    /// unexpected ident (bar not closed)
    pub fn error_2_1_13() -> Self {
        ZError {
            pos: vec![],
            code: "2.1.13",
            message: "Opening bar not closed".to_string(),
        }
    }
    /// unexpected ident (Extra values past default value)
    pub fn error_2_1_14(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.14",
            message: format!("Extra values past default value (Got `{ident}`)"),
        }
    }
    /// unexpected ident (Variable name isn't variable)
    pub fn error_2_1_15(ident: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.15",
            message: format!("Variable name isn't variable (Got `{ident}`)"),
        }
    }
    /// unexpected ident (pre keyword at end of expression)
    pub fn error_2_1_16() -> Self {
        ZError {
            pos: vec![],
            code: "2.1.16",
            message: "`pre` at end of line".to_string(),
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_17() -> Self {
        ZError {
            pos: vec![],
            code: "2.1.17",
            message: "Parameters found after `class` keyword".to_string(),
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_18(kwd: &Keyword) -> Self {
        ZError {
            pos: vec![],
            code: "2.1.18",
            message: format!("Block expected after `{kwd:?}`"),
        }
    }

    /// expected pattern, got something else
    pub fn error_2_2(_ele: impl AstData) -> Self {
        ZError {
            pos: vec![],
            code: "2.2",
            message: format!("Expected pattern, got `{}`", /*ele.span.raw*/ ""), // TODO
        }
    }

    /// unfilled argument
    pub fn error_2_3(arg: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "2.3",
            message: format!("Unfilled argument `{arg}`"),
        }
    }

    /* 3. Variable & attribute errors */
    /// Variable not defined
    pub fn error_3_0(varname: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "3.0",
            message: format!("Undefined variable `{varname}`"),
        }
    }

    /// Type has no attribute (typechecker)
    pub fn error_3_1_0<T: Clone + PartialEq + Debug>(
        _parent: Ast,
        parent_type: Type<T>,
        attribute: impl Display,
    ) -> Self {
        ZError {
            pos: vec![],
            code: "3.1.0",
            message: format!(
                "`{}` (type `{}`) has no attribute `{}`",
                /*parent.span.raw.trim()*/ "", parent_type, attribute
            ),
        }
    }
    /// Type has no attribute (interpreter)
    pub fn error_3_1_1(parent: Value, attribute: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "3.1.1",
            message: format!(
                "`{}` (type `{}`) has no attribute `{}`",
                parent,
                parent.get_type_obj(),
                attribute
            ),
        }
    }

    /* 4. Type errors */
    /// Binary operator not implemented for type
    pub fn error_4_0_0(operator: impl Display, type1: impl Display, type2: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "4.0.0",
            message: format!(
                "Operator {} not implemented for types `{}`, `{}`",
                operator, type1, type2
            ),
        }
    }
    /// Unary operator not implemented for type
    pub fn error_4_0_1(operator: impl Display, ty: impl Display) -> Self {
        ZError {
            pos: vec![],
            code: "4.0.1",
            message: format!("Operator {operator} not implemented for type `{ty}`"),
        }
    }

    /// Binary operation unsuccessful
    pub fn error_4_1_0(operator: impl Display, value1: Value, value2: Value) -> Self {
        ZError {
            pos: vec![],
            code: "4.1.0",
            message: format!(
                "Operator {} unsuccessful on `{}` (type `{}`), `{}` (type `{}`)",
                operator,
                value1,
                value1.get_type_obj(),
                value2,
                value2.get_type_obj()
            ),
        }
    }
    /// Unary operation unsuccessful
    pub fn error_4_1_1(operator: impl Display, value: Value) -> Self {
        ZError {
            pos: vec![],
            code: "4.1.1",
            message: format!(
                "Operator {} unsuccessful on `{}` (type `{}`)",
                operator,
                value,
                value.get_type_obj()
            ),
        }
    }

    /// Non-i32 script return value
    pub fn error_4_2(value: Value) -> Self {
        ZError {
            pos: vec![],
            code: "4.2",
            message: format!("Non-i32 script return value detected (Got `{value}`)"),
        }
    }

    /// Wrong type assigned to variable
    pub fn error_4_3<T1: Clone + PartialEq + Debug, T2: Clone + PartialEq + Debug>(
        variable: impl Display,
        var_type: Type<T1>,
        value_type: Type<T2>,
    ) -> Self {
        ZError {
            pos: vec![],
            code: "4.3",
            message: format!(
                "Value of type `{}` assigned to variable `{}` of type `{}`",
                value_type, variable, var_type
            ),
        }
    }

    /// inconsistent block return type (temporary)
    pub fn error_4_t<T1: Clone + PartialEq + Debug, T2: Clone + PartialEq + Debug>(
        block_type: Type<T1>,
        return_type: Type<T2>,
    ) -> Self {
        ZError {
            pos: vec![],
            code: "4.4",
            message: format!("Block returns variable of type `{block_type}` earlier on, but also returns variable of type `{return_type}`")
        }
    }
    #[tracing::instrument(skip_all)]
    pub fn get_surrounding_text(&self) -> String {
        self.pos
            .iter()
            .map(|span| {
                let pos = format!(" {} ", span.start_pos).bold().on_red().to_string();
                let filename = if let Some(filename) = &span.start_pos.filename {
                    filename.as_ref()
                } else {
                    return pos;
                };
                let mut contents = if let Some(input) = get_input(filename) {
                    input.split('\n').map(|a| a.to_string()).collect::<Vec<_>>()
                } else {
                    return pos;
                };
                debug!(start = ?span.start_pos, end = ?span.end_pos, "Generating surrounding text");
                let start_line = (span.start_pos.line - 1).saturating_sub(2);
                let end_line = (span.end_pos.line - 1 + 3).min(contents.len());

                let start_vec = contents[span.start_pos.line - 1]
                    .chars()
                    .collect::<Vec<_>>();
                contents[span.start_pos.line - 1] = start_vec[..span.start_pos.column - 1]
                    .iter()
                    .cloned()
                    .chain("\u{001b}[0;1;4;31m".chars())
                    .chain(start_vec[span.start_pos.column - 1..].iter().cloned())
                    .join("");

                let end_vec = contents[span.end_pos.line - 1].chars().collect::<Vec<_>>();
                contents[span.end_pos.line - 1] = end_vec[..span.end_pos.column - 1]
                    .iter()
                    .cloned()
                    .chain("\u{001b}[0;37;2m".chars())
                    .chain(end_vec[span.end_pos.column - 1..].iter().cloned())
                    .join("");

                let surrounding = contents[start_line..=end_line].join("\n");

                format!("{pos}\n{}", surrounding.white().dimmed())
            })
            .join("\n")
    }
    pub fn print_exit(self) -> ! {
        self.print();
        exit(1)
    }
    pub fn print(&self) {
        println!("{}", self.get_surrounding_text());
        println!(
            " Error {} ",
            self.code.black().on_yellow().to_string()
                + &*format!(" {}", self.message).bold().red().to_string(),
        );
    }
    pub fn with_span(mut self, span: impl GetSpan) -> Self {
        self.pos = if let Some(span) = span.span() {
            vec![span]
        } else {
            vec![]
        };
        self
    }
}
