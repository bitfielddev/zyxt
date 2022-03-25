mod errors;
mod lexer;
mod parser;
mod objects;
mod instructor;
mod interpreter;
mod repl;

use std::fs::File;
use std::io::Read;
use std::time::Instant;
use ansi_term::Color::{White, Yellow};
use clap::Parser;
use crate::errors::ZyxtError;
use crate::lexer::lex;
use crate::objects::token::Token;
use crate::parser::parse_token_list;
use crate::instructor::gen_instructions;
use crate::interpreter::interpret_asts;
use crate::objects::element::Element;
use crate::objects::typeobj::TypeObj;
use crate::objects::varstack::Stack;

fn compile(input: String, filename: &String, typelist: &mut Stack<TypeObj>, debug_info: bool) -> Result<Vec<Element>, ZyxtError> {
    if !debug_info {return gen_instructions(parse_token_list(lex(input, filename)?, filename)?, typelist)}

    println!("{}", Yellow.bold().paint("Lexing"));
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    for token in lexed.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", token)));}

    println!("{}", Yellow.bold().paint("\nParsing"));
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed, filename)?;
    let parse_time = parse_start.elapsed().as_micros();
    for ele in parsed.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", ele)));}

    println!("{}", Yellow.bold().paint("\nGenerating instructions"));
    let check_start = Instant::now();
    let out = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    for ele in out.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", ele)));}

    println!("{}", Yellow.bold().paint("\nStats"));
    println!("Lexing time: {}µs", lex_time);
    println!("Parsing time: {}µs", parse_time);
    println!("Instruction generation time: {}µs", check_time);
    println!("Total time: {}µs", lex_time+parse_time+check_time);

    Ok(out)
}

fn interpret(input: Vec<Element>, debug_info: bool) -> Result<(), ZyxtError>{
    if !debug_info {interpret_asts(input)?; return Ok(())}
    println!("{}", Yellow.bold().paint("\nInterpreting"));
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    println!("\nExited with code {}", exit_code);
    println!("{}", Yellow.bold().paint("\nStats"));
    println!("Interpreting time: {}µs", interpret_time);
    Ok(())
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long)]
    verbose: bool,
}
#[derive(Parser)]
enum Subcmd {
    /// Runs Zyxt source code
    Run(Run),
    /// Start a REPL for Zyxt
    Repl
}
#[derive(Parser)]
struct Run {
    filename: String
}

fn main() {
    let args = Args::parse();
    let verbose = if cfg!(debug_assertions) {true} else {args.verbose};
    match args.subcmd {
        Subcmd::Run(sargs) => {
            let filename = &sargs.filename;
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap_or_else(|e| {
                        if e.to_string() == "Is a directory (os error 21)".to_string() {
                            ZyxtError::no_pos().error_1_2(filename.clone()).print()
                        } else {panic!("{}", e.to_string())}
                    });
                },
                Err(_) => { ZyxtError::no_pos().error_1_1(filename.clone()).print() }
            };
            let mut typelist = Stack::<TypeObj>::default_type();
            interpret(compile(content, filename, &mut typelist,verbose).unwrap_or_else(|e| {
                e.print()
            }), verbose).unwrap_or_else(|e| {
                e.print()
            });
        },
        // TODO Compile, Interpret
        Subcmd::Repl => {
            repl::repl(verbose)
        }
    }
}
