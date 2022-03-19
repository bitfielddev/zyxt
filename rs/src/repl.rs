use std::io;
use std::io::Write;
use std::time::Instant;
use ansi_term::Color::{White, Yellow};
use text_io::read;
use crate::compile;
use crate::interpreter::interpret_expr;
use crate::objects::typeobj::TypeObj;
use crate::objects::variable::Variable;
use crate::objects::varstack::Varstack;

pub fn repl(debug_info: bool) {
    let filename = "[stdin]".to_string();
    let mut typelist = Varstack::<TypeObj>::default_type();
    let mut varlist = Varstack::<Variable>::default_variable();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let input: String = read!("{}\n");
        // TODO support for multiline

        if input == ";exit".to_string() {break;}
        let instructions = match compile(input, &filename, &mut typelist, debug_info) {
            Ok(v) => v,
            Err(e) => {e.print_noexit(); continue}
        };

        let instr_len = instructions.len();
        if debug_info {println!("{}", Yellow.bold().paint("\nInterpreting"));}
        for (i, instr) in instructions.into_iter().enumerate() {
            match {
                if !debug_info {interpret_expr(instr, &mut varlist)} else {
                    let interpret_start = Instant::now();
                    let result = interpret_expr(instr, &mut varlist);
                    let interpret_time = interpret_start.elapsed().as_micros();
                    println!("{}", White.dimmed().paint(format!("{}µs", interpret_time)));
                    result
                }} {
                Ok(result) => {
                    if result != Variable::Null && i == instr_len-1 { println!("{}", result) }
                },
                Err(e) => { e.print_noexit(); }
            }
        }
    }
}