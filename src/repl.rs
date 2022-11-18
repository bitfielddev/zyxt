use std::{io, io::Write, time::Instant};

use backtrace::Backtrace;
use dirs::home_dir;
use itertools::Either;
use owo_colors::OwoColorize;
use rustyline::{error::ReadlineError, Editor};
use smol_str::SmolStr;

use crate::{
    compile,
    types::{
        element::ElementData, interpreter_data::InterpreterData, printer::StdIoPrint, value::Value,
    },
    Element, Type, ZError,
};

pub fn repl(verbosity: u8) {
    let filename = SmolStr::from("[stdin]");
    let mut sip1 = StdIoPrint;
    let mut sip2 = StdIoPrint;
    let mut typelist = InterpreterData::<Type<Element>, _>::new(&mut sip1);
    let mut varlist = InterpreterData::<Value, _>::new(&mut sip2);
    let mut rl = Editor::<()>::new().unwrap();
    let mut history_path = home_dir().unwrap();
    history_path.push(".zyxt_history");
    rl.load_history(history_path.to_str().unwrap())
        .unwrap_or(());

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
        io::stdout().flush().unwrap();
        let input = rl.readline(&in_symbol);
        match input {
            Ok(input) => {
                if input == *";exit" {
                    break;
                }
                rl.add_history_entry(&input);
                rl.save_history(history_path.to_str().unwrap()).unwrap();
                if input.starts_with(';') {
                    match &*input {
                        ";vars" => println!("{}", varlist.heap_to_string()),
                        ";exit" => unreachable!(),
                        ";help" => {
                            println!("{}", "All commands start wih `;`".bold().yellow());
                            println!("{}", "help\tView this help page".cyan());
                            println!("{}", "exit\tExit the repl".cyan());
                            println!("{}", "vars\tView all variables".cyan())
                        }
                        _ => println!("{}", "Invalid command".red()),
                    };
                    continue;
                }
                let instructions =
                    match compile(Either::Right((filename.to_owned(), input)), &mut typelist) {
                        Ok(v) => v,
                        Err(e) => {
                            e.print(&mut StdIoPrint);
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
                            println!("{}", format!("{interpret_time}µs").dimmed().white());
                            result
                        }
                    } {
                        Ok(result) => {
                            if result != Value::Unit && i == instr_len - 1 {
                                println!("{out_symbol}{}", format!("{result:?}").yellow())
                            }
                        }
                        Err(e) => {
                            e.print(&mut StdIoPrint);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("{}", "`;exit` to exit".cyan());
            }
            Err(err) => {
                ZError::error_0_0(err.to_string(), Backtrace::new());
            }
        }
    }
    rl.save_history(history_path.to_str().unwrap()).unwrap();
}
