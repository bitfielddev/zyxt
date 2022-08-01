mod errors;
mod instructor;
mod interpreter;
mod lexer;
mod objects;
mod parser;
mod repl;

use crate::errors::ZyxtError;
use crate::instructor::gen_instructions;
use crate::interpreter::interpret_asts;
use crate::lexer::lex;
use crate::objects::element::Element;
use crate::objects::interpreter_data::{InterpreterData, Print, StdIoPrint};
use crate::objects::token::Token;
use crate::objects::typeobj::Type;
use crate::objects::value::Value;
use crate::parser::parse_token_list;
use ansi_term::Color::{White, Yellow};
use backtrace::Backtrace;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::process::exit;
use std::time::Instant;

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
    Repl,
}
#[derive(Parser)]
struct Run {
    filename: String,
}

fn main() {
    let args = Args::parse();
    let verbose = args.verbose;

    panic::set_hook(Box::new(|a| {
        ZyxtError::no_pos()
            .error_0_0(a.to_string(), Backtrace::new())
            .print_noexit();
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
                        } else {
                            panic!("{}", e.to_string())
                        }
                    });
                }
                Err(_) => ZyxtError::no_pos().error_1_1(filename.to_owned()).print(),
            };
            let mut typelist = InterpreterData::<_, StdIoPrint>::default_type();
            let mut i_data = InterpreterData::default_variable(StdIoPrint);
            let exit_code = zyxt::interpret(
                &zyxt::compile(content, filename, &mut typelist, verbose).unwrap_or_else(|e| e.print()),
                &mut i_data, verbose,
            )
            .unwrap_or_else(|e| e.print());
            exit(exit_code);
        }
        // TODO Compile, Interpret
        Subcmd::Repl => repl::repl(verbose),
    }
}
