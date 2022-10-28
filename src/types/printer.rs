use std::fmt::{Debug, Display};

use ansi_term::{
    Color::{Red, White, Yellow},
    Style,
};

pub trait Print: Debug {
    fn println(&mut self, s: impl Display);
    fn print(&mut self, s: impl Display);
    fn eprintln(&mut self, s: impl Display);
    fn eprint(&mut self, s: impl Display);

    fn debug(&mut self, msg: impl Display);
    fn info(&mut self, msg: impl Display);
    fn warn(&mut self, msg: impl Display);
    fn error(&mut self, msg: impl Display);

    fn verbosity(&self) -> u8;
}

#[derive(Clone, Debug)]
pub struct StdIoPrint(pub u8);

impl StdIoPrint {
    fn log_print(&mut self, msg: impl Display, min_verbosity: u8, prefix: &str, color: Style) {
        if self.0 >= min_verbosity {
            self.eprintln(
                msg.to_string()
                    .split('\n')
                    .map(|s| format!("{} {s}", color.paint(prefix)))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
    }
}

impl Print for StdIoPrint {
    fn println(&mut self, s: impl Display) {
        println!("{s}")
    }
    fn print(&mut self, s: impl Display) {
        print!("{s}")
    }
    fn eprintln(&mut self, s: impl Display) {
        eprintln!("{s}")
    }
    fn eprint(&mut self, s: impl Display) {
        eprint!("{s}")
    }

    fn debug(&mut self, msg: impl Display) {
        self.log_print(msg, 2, "[D]", White.bold().dimmed());
    }
    fn info(&mut self, msg: impl Display) {
        self.log_print(msg, 1, "[I]", White.bold());
    }
    fn warn(&mut self, msg: impl Display) {
        self.log_print(msg, 0, "[W]", Yellow.bold());
    }
    fn error(&mut self, msg: impl Display) {
        self.log_print(msg, 0, "[E]", Red.bold());
    }

    fn verbosity(&self) -> u8 {
        self.0
    }
}
