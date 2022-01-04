mod errors;
mod syntax;
mod lexer;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use crate::lexer::lex;
use crate::syntax::Token;

const VERSION: &str = "0.0.0";

fn compile(input: String, filename: &String) -> Vec<Token> {
    println!("Lexing");

    let lex_start = Instant::now();
    let lexed = lex(input, filename);
    let lex_time = lex_start.elapsed().as_micros();
    for token in lexed {println!("{}", token);}

    //println!("Parsing");
    //let parse_start = Instant::now();
    //let parsed = parse(lexed, filename);
    //let parse_time = parse_start.elapsed().as_micros();
    //let total_time = lex_start.elapsed().as_micros();
    //println!(parsed);

    println!("Lexing time: {}µs", lex_time);
    //println!("Parsing time: {parse_time}µs");
    //println!("Total time: {total_time}µs");

    let out: Vec<Token> = vec![];
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() <= 0 {"help"} else {&args[1]};
    match cmd {
        "version" => {
            println!("Zyxt version {}", VERSION);
        }
        "run" => {
            if args.len() <= 1 {errors::error_0_1()};
            let filename = &args[2];
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap();
                },
                Err(_) => {errors::error_1_1(filename.clone())}
            };
            for thing in compile(content, filename) {println!("{}", thing);}

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
