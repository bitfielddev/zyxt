extern crate core;

mod errors;
mod lexer;
mod parser;
mod syntax;
mod typechecker;
mod interpret;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use regex::Error;
use crate::lexer::lex;
use crate::syntax::lexing::Token;
use crate::parser::parse_statements;
use crate::typechecker::typecheck;
use crate::interpret::interpret_asts;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn compile(input: String, filename: &String) -> Result<Vec<Token>, Error> {
    println!("Lexing");
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    for token in lexed.iter() {println!("{}", token);}

    println!("\nParsing");
    let parse_start = Instant::now();
    let parsed = parse_statements(lexed, filename);
    let parse_time = parse_start.elapsed().as_micros();
    for ele in parsed.iter() {println!("{}", ele);}

    println!("\nTypechecking");
    let typecheck_start = Instant::now();
    let typechecked = typecheck(parsed);
    let typecheck_time = typecheck_start.elapsed().as_micros();
    for ele in typechecked.iter() {println!("{}", ele);}

    println!("\nInterpreting"); // temporarily here
    let interpret_start = Instant::now();
    interpret_asts(typechecked);
    let interpret_time = interpret_start.elapsed().as_micros();

    println!("Lexing time: {}µs", lex_time);
    println!("Parsing time: {}µs", parse_time);
    println!("Typechecking time: {}µs", typecheck_time);
    println!("Interpreting time: {}µs", interpret_time);
    println!("Total time: {}µs", lex_time+parse_time+typecheck_time+interpret_time);

    let out: Vec<Token> = vec![];
    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::new();
    let cmd = &**args.get(1).unwrap_or(&default);
    match cmd {
        "version" => {
            println!("Zyxt version {}", VERSION);
        }
        "run" => {
            if args.len() <= 2 {errors::error_0_1()};
            let filename = &args[2];
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap();
                },
                Err(_) => {errors::error_1_1(filename.clone())}
            };
            for thing in compile(content, filename).unwrap() {println!("{}", thing);}

        }
        "compile" => {
            println!("Coming soon!");
        }
        "interpret" => {
            println!("Coming soon!");
        }
        _ => {
            // print help page
            println!("Coming soon!")
        }
    };
}
