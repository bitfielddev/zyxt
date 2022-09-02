use crate::types::position::Position;
use crate::types::token::{Keyword, Token};
use crate::types::value::Value;
use crate::{Element, Type};
use ansi_term::Color::{Black, Red, White, Yellow};
use ansi_term::Style;
use backtrace::Backtrace;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use unicode_segmentation::UnicodeSegmentation;
use crate::types::printer::Print;

#[derive(Clone)]
pub struct ZyxtError {
    pub position: Vec<(Position, String)>,
    pub code: &'static str,
    pub message: String,
}
impl ZyxtError {
    /* 0. Internal errors, have to do with the compiler-interpreter itself */
    /// Rust error
    pub fn error_0_0(error: String, backtrace: Backtrace) -> Self {
        ZyxtError {
            position: vec![],
            code: "0.0",
            message: format!("Internal error: \n{}\n{:?}\n\nThis shouldn't happen! Open an issue on our Github repo page: [TODO]", error, backtrace)
        }
    }

    /// No file given
    pub fn error_0_1() -> Self {
        ZyxtError {
            position: vec![],
            code: "0.1",
            message: "No file given".to_string(),
        }
    }

    /* 1. File and I/O errors */
    /// File does not exist
    pub fn error_1_0(filename: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "1.0",
            message: format!("File `{}` does not exist", filename),
        }
    }

    /// file cannot be opened
    pub fn error_1_1(filename: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "1.1",
            message: format!("File `{}` cannot be opened", filename),
        }
    }

    pub fn error_1_2(dirname: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "1.2",
            message: format!("Directory given (Got `{}`)", dirname),
        }
    }

    /* 2. Syntax errors */
    /// parentheses not closed properly (try swapping)
    pub fn error_2_0_0(paren1: String, paren2: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.0.0",
            message: format!(
                "Parentheses `{}` and `{}` not closed properly; try swapping them",
                paren1, paren2
            ),
        }
    }
    /// parentheses not closed properly (not closed)
    pub fn error_2_0_1(paren: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.0.1",
            message: format!("Parenthesis `{}` not closed", paren),
        }
    }
    /// parentheses not closed properly (not opened)
    pub fn error_2_0_2(paren: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.0.2",
            message: format!("Parenthesis `{}` not opened", paren),
        }
    }

    /// unexpected ident (generic)
    pub fn error_2_1_0(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.0",
            message: format!("Unexpected ident `{}`", ident),
        }
    }
    /// unexpected ident (lexer didnt recognise)
    pub fn error_2_1_1(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.1",
            message: format!("Ident `{}` not recognised by lexer", ident),
        }
    }
    /// unexpected ident (dot at end of expression)
    pub fn error_2_1_2() -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.2",
            message: "Stray `.` at end of expression".to_string(),
        }
    }
    /// unexpected ident (binary operator at start/end of expression)
    pub fn error_2_1_3(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.3",
            message: format!(
                "Stray `{}` binary operator at start/end of expression",
                ident
            ),
        }
    }
    /// unexpected ident (unary operator at start/end of expression)
    pub fn error_2_1_4(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.4",
            message: format!(
                "Stray `{}` unary operator at start/end of expression",
                ident
            ),
        }
    }
    /// unexpected ident (declaration expr at start/end of expression)
    pub fn error_2_1_5() -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.5",
            message: "Stray `:=` at start/end of expression".to_string(),
        }
    }
    /// unexpected ident (non-flag between first flag and declared variable)
    pub fn error_2_1_6(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.6",
            message: format!("Stray `{}` between first flag and declared variable", ident),
        }
    }
    /// unexpected ident ('else/elif'  found after 'else' keyword)
    pub fn error_2_1_7(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.7",
            message: format!("`{}` detected after `else` keyword", ident),
        }
    }
    /// unexpected ident (block expected, not ident)
    pub fn error_2_1_8(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.8",
            message: format!("Block expected, not `{}`", ident),
        }
    }
    /// unexpected ident ('else/elif' found without 'if' keyword)
    pub fn error_2_1_9(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.9",
            message: format!("Stray `{}` without starting `if`", ident),
        }
    }
    /// unexpected ident (stray comment start / end)
    pub fn error_2_1_10(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.10",
            message: format!("Stray unclosed/unopened `{}`", ident),
        }
    }
    /// unexpected ident (must be variable)
    pub fn error_2_1_11(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.11",
            message: format!("Only variables can be deleted (Got `{}`)", ident),
        }
    }
    /// unexpected ident (cannot delete dereferenced variable)
    pub fn error_2_1_12(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.12",
            message: format!("Cannot delete dereferenced variable (Got `{}`)", ident),
        }
    }
    /// unexpected ident (bar not closed)
    pub fn error_2_1_13() -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.13",
            message: "Opening bar not closed".to_string(),
        }
    }
    /// unexpected ident (Extra values past default value)
    pub fn error_2_1_14(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.14",
            message: format!("Extra values past default value (Got `{}`)", ident),
        }
    }
    /// unexpected ident (Variable name isn't variable)
    pub fn error_2_1_15(ident: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.15",
            message: format!("Variable name isn't variable (Got `{}`)", ident),
        }
    }
    /// unexpected ident (pre keyword at end of expression)
    pub fn error_2_1_16() -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.16",
            message: "`pre` at end of line".to_string(),
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_17() -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.17",
            message: "Parameters found after `class` keyword".to_string(),
        }
    }
    /// unexpected ident (parameters with class keyword)
    pub fn error_2_1_18(kwd: &Keyword) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.1.18",
            message: format!("Block expected after `{:?}`", kwd),
        }
    }

    /// expected pattern, got something else
    pub fn error_2_2(ele: Element) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.2",
            message: format!("Expected pattern, got `{}`", ele.get_raw()),
        }
    }

    /// unfilled argument
    pub fn error_2_3(arg: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "2.3",
            message: format!("Unfilled argument `{}`", arg),
        }
    }

    /* 3. Variable & attribute errors */
    /// Variable not defined
    pub fn error_3_0(varname: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "3.0",
            message: format!("Undefined variable `{}`", varname),
        }
    }

    /// Type has no attribute (typechecker)
    pub fn error_3_1_0(parent: Element, parent_type: Type, attribute: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "3.1.0",
            message: format!(
                "`{}` (type `{}`) has no attribute `{}`",
                parent.get_raw().trim(),
                parent_type,
                attribute
            ),
        }
    }
    /// Type has no attribute (interpreter)
    pub fn error_3_1_1(parent: Value, attribute: String) -> Self {
        ZyxtError {
            position: vec![],
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
    pub fn error_4_0_0(operator: String, type1: String, type2: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "4.0.0",
            message: format!(
                "Operator {} not implemented for types `{}`, `{}`",
                operator, type1, type2
            ),
        }
    }
    /// Unary operator not implemented for type
    pub fn error_4_0_1(operator: String, type_: String) -> Self {
        ZyxtError {
            position: vec![],
            code: "4.0.1",
            message: format!("Operator {} not implemented for type `{}`", operator, type_),
        }
    }

    /// Binary operation unsuccessful
    pub fn error_4_1_0(operator: String, value1: Value, value2: Value) -> Self {
        ZyxtError {
            position: vec![],
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
    pub fn error_4_1_1(operator: String, value: Value) -> Self {
        ZyxtError {
            position: vec![],
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
        ZyxtError {
            position: vec![],
            code: "4.2",
            message: format!("Non-i32 script return value detected (Got `{}`)", value),
        }
    }

    /// Wrong type assigned to variable
    pub fn error_4_3(variable: String, var_type: Type, value_type: Type) -> Self {
        ZyxtError {
            position: vec![],
            code: "4.3",
            message: format!(
                "Value of type `{}` assigned to variable `{}` of type `{}`",
                value_type, variable, var_type
            ),
        }
    }

    /// inconsistent block return type (temporary)
    pub fn error_4_t(block_type: Type, return_type: Type) -> Self {
        ZyxtError {
            position: vec![],
            code: "4.4",
            message: format!("Block returns variable of type `{}` earlier on, but also returns variable of type `{}`", block_type, return_type)
        }
    }
    pub fn get_surrounding_text(&self) -> String {
        self.position
            .iter()
            .map(|(pos, raw)| {
                format!(
                    "{}\n{}",
                    Style::new().on(Red).bold().paint(format!(" {} ", pos)),
                    White
                        .dimmed()
                        .paint(if let Ok(mut file) = File::open(&pos.filename) {
                            let mut contents = String::new();
                            if file.read_to_string(&mut contents).is_ok() {
                                let mut contents = contents
                                    .split('\n')
                                    .map(|s| s.to_string())
                                    .collect::<Vec<_>>();

                                let start = contents.get_mut(pos.line as usize - 1).unwrap();
                                let start_graphemes = start.graphemes(true).collect::<Vec<_>>();
                                let index = pos.column as usize - 1;
                                *start = start_graphemes[0..index]
                                    .iter()
                                    .cloned()
                                    .chain(vec!["\u{001b}[0;1;4;31m"])
                                    .chain(start_graphemes[index..].iter().cloned())
                                    .collect::<Vec<_>>()
                                    .join("");

                                let end_pos = pos.pos_after(raw);
                                let end = contents.get_mut(end_pos.line as usize - 1).unwrap();
                                let end_graphemes = end.graphemes(true).collect::<Vec<_>>();
                                let index = end_pos.column as usize - 1
                                    + if pos.line == end_pos.line { 11 } else { 0 };
                                *end = end_graphemes[0..index]
                                    .iter()
                                    .cloned()
                                    .chain(vec!["\u{001b}[0;37;2m"])
                                    .chain(end_graphemes[index..].iter().cloned())
                                    .collect::<Vec<_>>()
                                    .join("");

                                contents
                                    .into_iter()
                                    .enumerate()
                                    .filter(|(i, _)| {
                                        pos.line as isize - 3 <= *i as isize
                                            && *i as u32 <= end_pos.line + 1
                                    })
                                    .map(|(_, s)| format!("  {}", s))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            } else {
                                raw.to_owned()
                            }
                        } else {
                            raw.to_owned()
                        })
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn print_exit(self, out: &mut impl Print) -> ! {
        self.print(out);
        exit(1)
    }
    pub fn print(&self, out: &mut impl Print) {
        out.println(self.get_surrounding_text());
        out.println(
            Black
                .on(Yellow)
                .paint(format!(" Error {} ", self.code))
                .to_string()
                + &*Red.bold().paint(format!(" {}", self.message)).to_string(),
        );
    }
    pub fn with_pos_and_raw(mut self, pos: &Position, raw: &String) -> Self {
        self.position = vec![(pos.to_owned(), raw.to_owned().trim().to_string())];
        self
    }
    pub fn with_element(mut self, element: &Element) -> Self {
        self.position = vec![(
            element.get_pos().to_owned(),
            element.get_raw().trim().to_string(),
        )];
        self
    }
    pub fn with_token(mut self, token: &Token) -> Self {
        self.position = vec![(
            token.position.to_owned(),
            token.value.to_owned().trim().to_string(),
        )];
        self
    }
}
