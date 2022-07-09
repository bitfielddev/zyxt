mod errors;
mod lexer;
mod parser;
mod objects;
mod instructor;
mod interpreter;
mod repl;

use backtrace::Backtrace;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::process::exit;
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
use crate::objects::typeobj::Type;
use crate::objects::interpreter_data::InterpreterData;

fn compile(input: String, filename: &str, typelist: &mut InterpreterData<Type>,
           verbosity: u8) -> Result<Vec<Element>, ZyxtError> {
    if verbosity == 0 {return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist)}

    if verbosity >= 2 {println!("{}", Yellow.bold().paint("Lexing"));}
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    if verbosity >= 2 {
        for token in lexed.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", token)));}
    }

    if verbosity >= 2 {println!("{}", Yellow.bold().paint("\nParsing"));}
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    if verbosity >= 2 {
        for ele in parsed.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", ele)));}
    }

    if verbosity >= 2 {println!("{}", Yellow.bold().paint("\nGenerating instructions"));}
    let check_start = Instant::now();
    let out = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    if verbosity >= 2 {
        for ele in out.iter() {println!("{}", White.dimmed().paint(format!("{:#?}", ele)));}
    }

    println!("{}", Yellow.bold().paint("\nStats"));
    println!("{}", Yellow.paint(format!("Lexing time: {}µs", lex_time)));
    println!("{}", Yellow.paint(format!("Parsing time: {}µs", parse_time)));
    println!("{}", Yellow.paint(format!("Instruction generation time: {}µs", check_time)));
    println!("{}", Yellow.paint(format!("Total time: {}µs\n", lex_time+parse_time+check_time)));

    Ok(out)
}

fn interpret(input: &Vec<Element>, verbosity: u8) -> Result<i32, ZyxtError>{
    if verbosity == 0 {return interpret_asts(input)}
    if verbosity >= 2 {println!("{}", Yellow.bold().paint("\nInterpreting"));}
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    println!("\nExited with code {}", exit_code);
    println!("{}", Yellow.bold().paint("\nStats"));
    println!("{}", Yellow.paint(format!("Interpreting time: {}µs", interpret_time)));
    Ok(exit_code)
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
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
    let verbose = args.verbose;

    panic::set_hook(Box::new(|a| {
        ZyxtError::no_pos().error_0_0(a.to_string(), Backtrace::new()).print_noexit();
    }));

    match args.subcmd {
        Subcmd::Run(sargs) => {
            let filename = &sargs.filename;
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap_or_else(|e| {
                        if e.to_string() == *"Is a directory (os error 21)" {
                            ZyxtError::no_pos().error_1_2(filename.to_owned()).print()
                        } else {panic!("{}", e.to_string())}
                    });
                },
                Err(_) => { ZyxtError::no_pos().error_1_1(filename.to_owned()).print() }
            };
            let mut typelist = InterpreterData::<Type>::default_type();
            let exit_code = interpret(&compile(content, filename, &mut typelist,verbose)
                                          .unwrap_or_else(|e| {
                e.print()
            }), verbose).unwrap_or_else(|e| {
                e.print()
            });
            exit(exit_code);
        },
        // TODO Compile, Interpret
        Subcmd::Repl => {
            repl::repl(verbose)
        }
    }
}
