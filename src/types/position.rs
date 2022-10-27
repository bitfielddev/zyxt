use std::fmt::{Debug, Display, Formatter};

use itertools::Either;
use smol_str::SmolStr;

use crate::{types::token::Token, Element};

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub filename: String,
    pub line: u32,
    pub column: u32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            filename: String::from("[unknown]"),
            line: 1,
            column: 1,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Position {
    pub fn pos_after(&self, string: &str) -> Position {
        Position {
            filename: self.filename.to_owned(),
            line: self.line + string.chars().filter(|c| *c == '\n').count() as u32,
            column: if string.contains('\n') {
                string.split('\n').last().unwrap().chars().count() as u32
            } else {
                self.column + string.chars().count() as u32
            },
        }
    }
    pub fn next_str(&mut self, c: &str) {
        if c == "\n" {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1
        }
    }
    pub fn next_char(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PosRaw {
    pub position: Position,
    pub raw: SmolStr,
}
pub trait GetPosRaw {
    fn pos_raw(&self) -> PosRaw;
}
impl GetPosRaw for Element {
    fn pos_raw(&self) -> PosRaw {
        self.pos_raw.to_owned()
    }
}
impl GetPosRaw for Token {
    fn pos_raw(&self) -> PosRaw {
        PosRaw {
            position: self.position.to_owned(),
            raw: self.get_raw().into(),
        }
    }
}
impl GetPosRaw for Either<Element, Token> {
    fn pos_raw(&self) -> PosRaw {
        match self {
            Either::Left(c) => c.pos_raw(),
            Either::Right(c) => c.pos_raw(),
        }
    }
}
