use std::fmt::{Debug, Display};

pub trait Print: Debug {
    fn println(&mut self, s: impl Display);
    fn print(&mut self, s: impl Display);
    fn eprintln(&mut self, s: impl Display);
    fn eprint(&mut self, s: impl Display);
}

#[derive(Clone, Debug)]
pub struct StdIoPrint;

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
}
