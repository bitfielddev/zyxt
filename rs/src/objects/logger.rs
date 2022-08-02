use std::fmt::Display;
use ansi_term::Color::{Red, White, Yellow};
use ansi_term::Style;
use crate::Print;

pub struct Logger<'a, O: Print> {
    pub verbosity: u8,
    pub out: &'a mut O,
}
impl<O: Print> Logger<'_, O> {
    fn print(&mut self, msg: impl Display, min_verbosity: u8, prefix: &str, color: Style) {
        if self.verbosity >= min_verbosity {
            self.out.eprintln(format!("{}{}", color.paint(prefix), msg));
        }
    }
    pub fn debug(&mut self, msg: impl Display) {
        self.print(msg, 2, "[D] ", White.bold().dimmed());
    }
    pub fn info(&mut self, msg: impl Display) {
        self.print(msg, 1, "[I] ", White.bold().dimmed());
    }
    pub fn warn(&mut self, msg: impl Display) {
        self.print(msg, 0, "[W] ", Yellow.bold().dimmed());
    }
    pub fn error(&mut self, msg: impl Display) {
        self.print(msg, 0, "[E] ", Red.bold().dimmed());
    }
}