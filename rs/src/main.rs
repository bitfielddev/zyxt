extern crate core;

mod errors;
mod lexer;
mod parser;
mod syntax;
mod typechecker;
mod interpreter;

use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::time::Instant;
use ansi_term::Color::{White, Yellow};
use crate::lexer::lex;
use crate::syntax::token::Token;
use crate::parser::parse_statements;
use crate::typechecker::typecheck;
use crate::interpreter::interpret_asts;
use crate::syntax::element::Element;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn compile(input: String, filename: &String, debug_info: bool) -> Result<Vec<Element>, Error> {
    if !debug_info {return Ok(typecheck(parse_statements(lex(input, filename)?, filename)))}

    println!("{}", Yellow.bold().paint("Lexing"));
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    for token in lexed.iter() {println!("{}", White.dimmed().paint(token.to_string()));}

    println!("{}", Yellow.bold().paint("\nParsing"));
    let parse_start = Instant::now();
    let parsed = parse_statements(lexed, filename);
    let parse_time = parse_start.elapsed().as_micros();
    for ele in parsed.iter() {println!("{}", White.dimmed().paint(ele.to_string()));}

    println!("{}", Yellow.bold().paint("\nTypechecking"));
    let typecheck_start = Instant::now();
    let out = typecheck(parsed);
    let typecheck_time = typecheck_start.elapsed().as_micros();
    for ele in out.iter() {println!("{}", White.dimmed().paint(ele.to_string()));}

    println!("{}", Yellow.bold().paint("\nStats"));
    println!("Lexing time: {}µs", lex_time);
    println!("Parsing time: {}µs", parse_time);
    println!("Typechecking time: {}µs", typecheck_time);
    println!("Total time: {}µs", lex_time+parse_time+typecheck_time);

    Ok(out)
}

fn interpret(input: Vec<Element>, debug_info: bool) {
    if !debug_info {interpret_asts(input); return}
    println!("{}", Yellow.bold().paint("\nInterpreting"));
    let interpret_start = Instant::now();
    interpret_asts(input);
    let interpret_time = interpret_start.elapsed().as_micros();
    println!("{}", Yellow.bold().paint("\nStats"));
    println!("Interpreting time: {}µs", interpret_time);
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
            if args.len() <= 2 { errors::error_0_1() };
            let filename = &args[2];
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap();
                },
                Err(_) => { errors::error_1_1(filename.clone()) }
            };
            let debug_info = true;
            interpret(compile(content, filename, debug_info).unwrap(), debug_info);
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
    }
}
