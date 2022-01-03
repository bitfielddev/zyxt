use std::process::exit;
use ansi_term::Color::{Red, Yellow};
use ansi_term::Style;
use fstrings::f;

fn error_main(code: string, message: string) {
    println!(Style::new().on(Yellow).paint(f!("Error {code}"))
    + Yellow.paint("")
    + Red.bold().paint(f!(" {message}")));
    exit(1)
}

pub fn error_pos(filename: string, line: i32, column: i32) {
    print!(Style::new().on(Red).bold().paint(f!("{filename} {line} {column}"))
    +Red.bold().on(Yellow).paint(""))
}

/* 0. Internal errors, have to do with the compiler-interpreter itself */
// Rust error
pub fn error_0_0(stack: string) {
    error_main("0.0", f!("Internal error: \n{stack}"))
}

// No file given
pub fn error_0_1() {
    error_main("0.1", "No file given")
}

/* 1. File and I/O errors */
// File does not exist
pub fn error_1_0(filename: string) {
    error_main("1.0", f!("File `{filename}` does not exist"))
}

// file cannot be opened
pub fn error_1_1(filename: string) {
    error_main("1.1", f!("File `{filename}` cannot be opened"))
}

/* 2. Syntax errors */
// parentheses not closed properly
pub fn error_2_0_0(paren1: string, paren2: string) {
    error_main("2.0.0", f!("Parentheses `{paren1}` and `{paren2}` not closed properly; try swapping them"))
}
pub fn error_2_0_1(paren: string) {
    error_main("2.0.1", f!("Parenthesis `{paren}` not closed"))
}
pub fn error_2_0_2(paren: string) {
    error_main("2.0.2", f!("Parenthesis `{paren}` not opened"))
}

// unexpected ident
pub fn error_2_1(ident: string) {
    error_main("2.1", f!("Unexpected ident `{ident}`"))
}

// assignment without variable name
pub fn error_2_2() {
    error_main("2.2", "Assignment without variable name")
}

/* 3. Variable & attribute errors */
// Variable not defined
pub fn error_3_0(varname: string) {
    error_main("3.0", f!("Undefined variable `{varname}`"))
}