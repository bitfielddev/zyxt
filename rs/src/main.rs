mod errors;
mod syntax;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Instant, SystemTime};
use fstrings::println_f;
use crate::syntax::Token;

const VERSION: &str = "0.0.0";

fn compile(input: string, filename: string) -> Vec<Token> {
    println!("Lexing");

    let lex_start = Instant::now();
    let lexed = lex(input, filename);
    let lex_time = lex_start.elapsed().as_micros();
    println!(lexed);

    println!("Parsing");
    let parse_start = Instant::now();
    let parsed = parse(lexed, filename);
    let parse_time = parse_start.elapsed().as_micros();
    let total_time = lex_start.elapsed().as_micros();
    println!(parsed);

    println_f!("Lexing time: {lex_time}µs");
    println_f!("Parsing time: {parse_time}µs");
    println_f!("Total time: {total_time}µs");

    let out = Vec<Token>;
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() <= 0 {"help"} else {&args[1]};
    match cmd {
        "version" => {
            println!("Zyxt version {}", version);
        }
        "run" => {
            if args.len() <= 1 {errors::error_0_1()};
            let filename = &args[2];
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap();
                },
                Err(_) => {errors::error_1_1(filename)}
            };
            println!(compile(content, filename));

        }
        "compile" => {
            println!("Coming soon!");
        }
        "interpret" => {
            println!("Coming soon!");
        }
        _ => {
            // print help page
        }
    };
}
