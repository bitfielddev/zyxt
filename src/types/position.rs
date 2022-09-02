use std::fmt::{Debug, Display, Formatter};

use unicode_segmentation::UnicodeSegmentation;

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
            line: self.line + string.graphemes(true).filter(|c| *c == "\n").count() as u32,
            column: if string.contains('\n') {
                string.split('\n').last().unwrap().graphemes(true).count() as u32
            } else {
                self.column + string.graphemes(true).count() as u32
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
