use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
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
    pub fn next(&mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {self.column += 1}
    }
}
