use std::{io, io::Write, time::Instant};

use color_eyre::eyre::{eyre, Result};
use dirs::home_dir;
use itertools::Either;
use owo_colors::OwoColorize;
use rustyline::{error::ReadlineError, Editor};
use smol_str::SmolStr;

use crate::{
    ast::{Ast, AstData},
    compile,
    types::{interpreter_data::SymTable, value::Value},
    Type,
};

pub fn repl(verbosity: u8) -> Result<()> {
    let filename = SmolStr::from("[stdin]");
    let mut typelist = SymTable::<Type<Ast>>::default();
    let mut varlist = SymTable::<Value>::default();
    let mut rl = Editor::<()>::new()?;
    let mut history_path = home_dir().ok_or_else(|| eyre!("No home dir"))?;
    history_path.push(".zyxt_history");
    let _ = rl.load_history(&*history_path.to_string_lossy());

    let in_symbol = ">>] ".bold().cyan().to_string();
    let out_symbol = "[>> ".bold().green().to_string();
    println!(
        "{}",
        format!("Zyxt Repl (v{})", env!("CARGO_PKG_VERSION"))
            .bold()
            .yellow()
    );
    println!("{}", "`;exit` to exit".cyan());
    println!("{}", "`;help` for more commands".cyan());
    loop {
        print!("{in_symbol} ");
        io::stdout().flush()?;
        let input = rl.readline(&in_symbol);
        match input {
            Ok(input) => {
                if input == *";exit" {
                    break;
                }
                rl.add_history_entry(&input);
                rl.save_history(&*history_path.to_string_lossy())?;
                if input.starts_with(';') {
                    match &*input {
                        ";vars" => println!("{}", varlist.heap_to_string()),
                        ";exit" => unreachable!(),
                        ";help" => {
                            println!("{}", "All commands start wih `;`".bold().yellow());
                            println!("{}", "help\tView this help page".cyan());
                            println!("{}", "exit\tExit the repl".cyan());
                            println!("{}", "vars\tView all variables".cyan());
                        }
                        _ => println!("{}", "Invalid command".red()),
                    };
                    continue;
                }
                let instructions =
                    match compile(&Either::Right((filename.to_owned(), input)), &mut typelist) {
                        Ok(v) => v,
                        Err(e) => {
                            e.print();
                            continue;
                        }
                    };

                let instr_len = instructions.len();
                if verbosity >= 2 {
                    println!("{}", "\nInterpreting".bold().yellow());
                }
                for (i, instr) in instructions.into_iter().enumerate() {
                    match {
                        if verbosity == 0 {
                            instr.interpret_expr(&mut varlist)
                        } else {
                            let interpret_start = Instant::now();
                            let result = instr.interpret_expr(&mut varlist);
                            let interpret_time = interpret_start.elapsed().as_micros();
                            println!("{}", format!("{interpret_time}\u{b5}s").dimmed().white());
                            result
                        }
                    } {
                        Ok(result) => {
                            if result != Value::Unit && i == instr_len - 1 {
                                println!("{out_symbol}{}", format!("{result:?}").yellow());
                            }
                        }
                        Err(e) => {
                            e.print();
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("{}", "`;exit` to exit".cyan());
            }
            Err(err) => return Err(err.into()),
        }
    }
    rl.save_history(&*history_path.to_string_lossy())?;
    Ok(())
}
