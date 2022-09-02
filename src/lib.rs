pub mod instructor;
pub mod interpreter;
pub mod lexer;
pub mod types;
pub mod parser;
pub mod repl;

use crate::instructor::gen_instructions;
use crate::interpreter::interpret_asts;
use crate::lexer::lex;
use crate::types::element::Element;
use crate::types::frame_data::InterpreterData;
use crate::types::typeobj::Type;
use crate::types::value::Value;
use crate::parser::parse_token_list;
use ansi_term::Color::{White, Yellow};
use types::errors::ZyxtError;
use std::time::Instant;
use types::printer::Print;

pub fn compile(
    input: String,
    filename: &str,
    typelist: &mut InterpreterData<Type, impl Print>,
) -> Result<Vec<Element>, ZyxtError> {
    if typelist.out.verbosity() == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist);
    }

    typelist.out.debug(Yellow.bold().paint("Lexing"));
    let lex_start = Instant::now();
    let lexed = lex(input, filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    typelist.out.debug(White.dimmed().paint(format!("{:#?}", lexed)));

    typelist.out.debug(Yellow.bold().paint("\nParsing"));
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    typelist.out.debug(White.dimmed().paint(format!("{:#?}", parsed)));

    typelist.out.debug(Yellow.bold().paint("\nGenerating instructions"));
    let check_start = Instant::now();
    let instructions = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    typelist.out.debug(White.dimmed().paint(format!("{:#?}", instructions)));

    typelist.out.info(Yellow.bold().paint("\nStats"));
    typelist.out.info(Yellow.paint(format!("Lexing time: {}µs", lex_time)));
    typelist.out.info(Yellow.paint(format!("Parsing time: {}µs", parse_time)));
    typelist.out.info(Yellow.paint(format!("Instruction generation time: {}µs", check_time)));
    typelist.out.info(Yellow.paint(format!(
        "Total time: {}µs\n",
        lex_time + parse_time + check_time
    )));

    Ok(instructions)
}

pub fn interpret(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, impl Print>,
) -> Result<i32, ZyxtError> {
    if i_data.out.verbosity() == 0 {
        return interpret_asts(input, i_data);
    }
    i_data.out.debug(Yellow.bold().paint("\nInterpreting"));
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, i_data)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    i_data.out.debug(format!("\nExited with code {}", exit_code));
    i_data.out.info(Yellow.bold().paint("\nStats"));
    i_data.out.info(Yellow.paint(format!("Interpreting time: {}µs", interpret_time)));
    Ok(exit_code)
}
