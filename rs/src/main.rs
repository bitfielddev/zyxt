mod errors;
mod lexer;
mod parser;
mod syntax;
mod checker;
mod interpreter;

use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::time::Instant;
use ansi_term::Color::{White, Yellow};
use clap::Parser;
use crate::lexer::lex;
use crate::syntax::token::Token;
use crate::parser::parse_statements;
use crate::checker::check;
use crate::interpreter::interpret_asts;
use crate::syntax::element::Element;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn compile(input: String, filename: &String, debug_info: bool) -> Result<Vec<Element>, Error> {
    if !debug_info {return Ok(check(parse_statements(lex(input, filename)?, filename)))}

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

    println!("{}", Yellow.bold().paint("\nChecking"));
    let check_start = Instant::now();
    let out = check(parsed);
    let check_time = check_start.elapsed().as_micros();
    for ele in out.iter() {println!("{}", White.dimmed().paint(ele.to_string()));}

    println!("{}", Yellow.bold().paint("\nStats"));
    println!("Lexing time: {}µs", lex_time);
    println!("Parsing time: {}µs", parse_time);
    println!("Checking time: {}µs", check_time);
    println!("Total time: {}µs", lex_time+parse_time+check_time);

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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd
}
#[derive(Parser)]
enum Subcmd {
    Run(Run),
    Version
}
#[derive(Parser)]
struct Run {
    filename: String
}

fn main() {
    let args = Args::parse();
    match args.subcmd {
        Subcmd::Version => {
            println!("Zyxt version {}", VERSION);
        },
        Subcmd::Run(sargs) => {
            let filename = &sargs.filename;
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap();
                },
                Err(_) => { errors::error_1_1(filename.clone()) }
            };
            let debug_info = true;
            interpret(compile(content, filename, debug_info).unwrap(), debug_info);
        },
        // TODO Compile, Interpret
    }
}
