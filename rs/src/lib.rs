pub mod errors;
pub mod instructor;
pub mod interpreter;
pub mod lexer;
pub mod objects;
pub mod parser;
pub mod repl;

use crate::errors::ZyxtError;
use crate::instructor::gen_instructions;
use crate::interpreter::interpret_asts;
use crate::lexer::lex;
use crate::objects::element::Element;
use crate::objects::interpreter_data::{InterpreterData, Print};
use crate::objects::typeobj::Type;
use crate::objects::value::Value;
use crate::parser::parse_token_list;
use ansi_term::Color::{White, Yellow};
use std::time::Instant;

pub fn compile<O: Print>(
    input: String,
    filename: &str,
    typelist: &mut InterpreterData<Type, O>,
    verbosity: u8
) -> Result<Vec<Element>, ZyxtError> {
    if verbosity == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist);
    }

    if verbosity >= 2 {
        typelist.out.eprintln(Yellow.bold().paint("Lexing"));
    }
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    if verbosity >= 2 {
        for token in lexed.iter() {
            typelist.out.eprintln(White.dimmed().paint(format!("{:#?}", token)));
        }
    }

    if verbosity >= 2 {
        typelist.out.eprintln(Yellow.bold().paint("\nParsing"));
    }
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    if verbosity >= 2 {
        for ele in parsed.iter() {
            typelist.out.eprintln(White.dimmed().paint(format!("{:#?}", ele)));
        }
    }

    if verbosity >= 2 {
        typelist.out.eprintln(Yellow.bold().paint("\nGenerating instructions"));
    }
    let check_start = Instant::now();
    let instructions = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    if verbosity >= 2 {
        for ele in instructions.iter() {
            typelist.out.eprintln(White.dimmed().paint(format!("{:#?}", ele)));
        }
    }

    typelist.out.eprintln(Yellow.bold().paint("\nStats"));
    typelist.out.eprintln(Yellow.paint(format!("Lexing time: {}µs", lex_time)));
    typelist.out.eprintln(
        Yellow.paint(format!("Parsing time: {}µs", parse_time))
    );
    typelist.out.eprintln(
        Yellow.paint(format!("Instruction generation time: {}µs", check_time))
    );
    typelist.out.eprintln(
        Yellow.paint(format!(
            "Total time: {}µs\n",
            lex_time + parse_time + check_time
        ))
    );

    Ok(instructions)
}

pub fn interpret<O: Print>(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, O>,
    verbosity: u8,
) -> Result<i32, ZyxtError> {
    if verbosity == 0 {
        return interpret_asts(input, i_data);
    }
    if verbosity >= 2 {
        i_data.out.eprintln(Yellow.bold().paint("\nInterpreting"));
    }
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, i_data)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    i_data.out.eprintln(format!("\nExited with code {}", exit_code));
    i_data.out.eprintln(Yellow.bold().paint("\nStats"));
    i_data.out.eprintln(
        Yellow.paint(format!("Interpreting time: {}µs", interpret_time))
    );
    Ok(exit_code)
}
