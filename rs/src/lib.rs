pub mod instructor;
pub mod interpreter;
pub mod lexer;
pub mod objects;
pub mod parser;
pub mod repl;

use crate::instructor::gen_instructions;
use crate::interpreter::interpret_asts;
use crate::lexer::lex;
use crate::objects::element::Element;
use crate::objects::interpreter_data::{InterpreterData, Print};
use crate::objects::logger::Logger;
use crate::objects::typeobj::Type;
use crate::objects::value::Value;
use crate::parser::parse_token_list;
use ansi_term::Color::{White, Yellow};
use objects::errors::ZyxtError;
use std::time::Instant;

pub fn compile(
    input: String,
    filename: &str,
    typelist: &mut InterpreterData<Type, impl Print>,
    logger: &mut Logger<impl Print>,
) -> Result<Vec<Element>, ZyxtError> {
    if logger.verbosity == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist);
    }

    logger.debug(Yellow.bold().paint("Lexing"));
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    logger.debug(White.dimmed().paint(format!("{:#?}", lexed)));

    logger.debug(Yellow.bold().paint("\nParsing"));
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    logger.debug(White.dimmed().paint(format!("{:#?}", parsed)));

    logger.debug(Yellow.bold().paint("\nGenerating instructions"));
    let check_start = Instant::now();
    let instructions = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    logger.debug(White.dimmed().paint(format!("{:#?}", instructions)));

    logger.info(Yellow.bold().paint("\nStats"));
    logger.info(Yellow.paint(format!("Lexing time: {}µs", lex_time)));
    logger.info(Yellow.paint(format!("Parsing time: {}µs", parse_time)));
    logger.info(Yellow.paint(format!("Instruction generation time: {}µs", check_time)));
    logger.info(Yellow.paint(format!(
        "Total time: {}µs\n",
        lex_time + parse_time + check_time
    )));

    Ok(instructions)
}

pub fn interpret(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, impl Print>,
    logger: &mut Logger<impl Print>,
) -> Result<i32, ZyxtError> {
    if logger.verbosity == 0 {
        return interpret_asts(input, i_data);
    }
    logger.debug(Yellow.bold().paint("\nInterpreting"));
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, i_data)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    logger.debug(format!("\nExited with code {}", exit_code));
    logger.info(Yellow.bold().paint("\nStats"));
    logger.info(Yellow.paint(format!("Interpreting time: {}µs", interpret_time)));
    Ok(exit_code)
}
