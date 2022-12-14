#![feature(box_patterns)]
#![feature(iterator_try_reduce)]

pub mod ast;
pub mod file_importer;
pub mod instructor;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod primitives;
pub mod repl;
pub mod types;

use std::{path::Path, time::Instant};

use itertools::Either;
use smol_str::SmolStr;
use tracing::{info, trace};
use types::errors::{ZError, ZResult};

use crate::{
    ast::Ast,
    file_importer::{import_file, register_input},
    instructor::gen_instructions,
    interpreter::interpret_asts,
    lexer::lex,
    parser::parse_token_list,
    types::{interpreter_data::SymTable, typeobj::Type, value::Value},
};

pub fn compile(
    file: Either<&Path, (SmolStr, String)>,
    typelist: &mut SymTable<Type<Ast>>,
) -> ZResult<Vec<Ast>> {
    /*if typelist.out.verbosity() == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, typelist);
    }*/
    // TODO --stats flag

    let (input, filename) = match &file {
        Either::Left(p) => (import_file(p), SmolStr::from(p.to_string_lossy())),
        Either::Right((name, input)) => (
            register_input(name.to_owned(), input.to_owned()),
            name.to_owned(),
        ),
    };

    info!("Lexing");
    let lex_start = Instant::now();
    let lexed = lex((*input).to_owned(), filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    trace!("{lexed:#?}");

    info!("Parsing");
    let parse_start = Instant::now();
    let parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    trace!("{parsed:#?}");

    info!("Generating instructions");
    let check_start = Instant::now();
    let instructions = gen_instructions(parsed, typelist)?;
    let check_time = check_start.elapsed().as_micros();
    trace!("{instructions:#?}");

    info!("Stats:");
    info!("Lexing time: {lex_time}µs");
    info!("Parsing time: {parse_time}µs");
    info!("Instruction generation time: {check_time}µs");
    info!("Total time: {}µs\n", lex_time + parse_time + check_time);

    Ok(instructions)
}

pub fn interpret(input: &Vec<Ast>, i_data: &mut SymTable<Value>) -> ZResult<i32> {
    /*if i_data.out.verbosity() == 0 {
        return interpret_asts(input, i_data);
    }*/
    // TODO --stats flag
    info!("Interpreting");
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, i_data)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    info!("Exited with code {exit_code}");
    info!("Stats");
    info!("Interpreting time: {interpret_time}µs");
    Ok(exit_code)
}
