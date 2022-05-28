use std::io;
use std::io::Write;
use std::time::Instant;
use ansi_term::Color::{Cyan, Green, White, Yellow};
use dirs::home_dir;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::{compile, ZyxtError};
use crate::interpreter::interpret_expr;
use crate::objects::typeobj::Type;
use crate::objects::variable::Variable;
use crate::objects::interpreter_data::InterpreterData;

pub fn repl(verbosity: u8) {
    let filename = "[stdin]".to_string();
    let mut typelist = InterpreterData::<Type>::default_type();
    let mut varlist = InterpreterData::<Variable>::default_variable();
    let mut rl = Editor::<()>::new();
    let mut history_path = home_dir().unwrap();
    history_path.push(".zyxt_history");
    rl.load_history(history_path.to_str().unwrap()).unwrap_or(());

    let in_symbol = Cyan.bold().paint(">>] ");
    let out_symbol = Green.bold().paint("[>> ");
    println!("{}", Yellow.bold().paint(format!("Zyxt Repl (v{})", env!("CARGO_PKG_VERSION"))));
    println!("{}", Cyan.paint("`;exit` to exit"));
    println!("{}", Cyan.paint("`;vars` to show variables"));
    loop {
        print!("{} ", in_symbol);
        io::stdout().flush().unwrap();
        let input = rl.readline(&*in_symbol.to_string());
        match input {
            Ok(input) => {
                rl.add_history_entry(&input);
                if input == *";exit" {break;}
                if input == *";vars" {
                    println!("{}", varlist.heap_to_string());
                    continue;
                }
                let instructions = match compile(input, &filename, &mut typelist, verbosity) {
                    Ok(v) => v,
                    Err(e) => {e.print_noexit(); continue}
                };

                let instr_len = instructions.len();
                if verbosity >= 2 {println!("{}", Yellow.bold().paint("\nInterpreting"));}
                for (i, instr) in instructions.into_iter().enumerate() {
                    match {
                        if verbosity == 0 {interpret_expr(instr, &mut varlist)} else {
                            let interpret_start = Instant::now();
                            let result = interpret_expr(instr, &mut varlist);
                            let interpret_time = interpret_start.elapsed().as_micros();
                            println!("{}", White.dimmed().paint(format!("{}µs", interpret_time)));
                            result
                        }} {
                        Ok(result) => {
                            if result != Variable::Null && i == instr_len-1 {
                                println!("{}{}", out_symbol, Yellow.paint(result.to_string()))
                            }
                        },
                        Err(e) => { e.print_noexit(); }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("{}", Cyan.paint("`;exit` to exit"));
            }
            Err(err) => {
                ZyxtError::no_pos().error_0_0(err.to_string());
            }
        }


    }
    rl.save_history(history_path.to_str().unwrap()).unwrap();
}