use std::{io, io::Write, time::Instant};

use color_eyre::eyre::{eyre, Result};
use dirs::home_dir;
use itertools::Either;
use owo_colors::OwoColorize;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use smol_str::SmolStr;
use tracing::info;

use crate::{
    ast::AstData,
    compile,
    types::{
        sym_table::{InterpretSymTable, TypeCheckSymTable},
        value::Value,
    },
};

pub fn repl() -> Result<()> {
    let filename = SmolStr::from("[stdin]");
    let mut ty_symt = TypeCheckSymTable::default();
    let mut val_symt = InterpretSymTable::default();
    let mut rl = Editor::<(), FileHistory>::new()?;
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
                rl.add_history_entry(&input)?;
                rl.save_history(&*history_path.to_string_lossy())?;
                if input.starts_with(';') {
                    match &*input {
                        ";vars" => println!("{val_symt}"),
                        ";vars_det" => println!("{val_symt:#}"),
                        ";exit" => unreachable!(),
                        ";help" => {
                            println!("{}", "All commands start wih `;`".bold().yellow());
                            println!("{}", "help\tView this help page".cyan());
                            println!("{}", "exit\tExit the repl".cyan());
                            println!("{}", "vars\tView all variables".cyan());
                            println!(
                                "{}",
                                "vars\tView all variables, but with more detail".cyan()
                            );
                        }
                        _ => println!("{}", "Invalid command".red()),
                    };
                    continue;
                }
                let instructions = match compile(
                    &Either::Right((filename.to_owned(), input)),
                    &mut ty_symt,
                    false,
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        e.print()?;
                        continue;
                    }
                };

                let instr_len = instructions.len();
                info!("Interpreting");
                for (i, instr) in instructions.into_iter().enumerate() {
                    match {
                        let interpret_start = Instant::now();
                        let result = instr.interpret_expr(&mut val_symt);
                        let interpret_time = interpret_start.elapsed().as_micros();
                        info!("{interpret_time}\u{b5}s");
                        result
                    } {
                        Ok(result) => {
                            if result != Value::Unit && i == instr_len - 1 {
                                println!("{out_symbol}{}", format!("{result:?}").yellow());
                            }
                        }
                        Err(e) => {
                            e.print()?;
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
