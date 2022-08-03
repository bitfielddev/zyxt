use backtrace::Backtrace;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::process::exit;
use zyxt::objects::errors::ZyxtError;
use zyxt::objects::interpreter_data::{InterpreterData, StdIoPrint};
use zyxt::objects::logger::Logger;
use zyxt::repl;

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
    let mut logger = Logger {
        verbosity: verbose,
        out: &mut StdIoPrint,
    };

    panic::set_hook(Box::new(|a| {
        ZyxtError::error_0_0(a.to_string(), Backtrace::new()).print(&mut StdIoPrint);
    }));

    match args.subcmd {
        Subcmd::Run(sargs) => {
            let filename = &sargs.filename;
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => {
                    file.read_to_string(&mut content).unwrap_or_else(|e| {
                        if e.to_string() == *"Is a directory (os error 21)" {
                            ZyxtError::error_1_2(filename.to_owned()).print_exit(&mut StdIoPrint)
                        } else {
                            panic!("{}", e.to_string())
                        }
                    });
                }
                Err(_) => ZyxtError::error_1_1(filename.to_owned()).print_exit(&mut StdIoPrint),
            };
            let mut sip1 = StdIoPrint;
            let mut sip2 = StdIoPrint;
            let mut typelist = InterpreterData::default_type(&mut sip1);
            let mut i_data = InterpreterData::default_variable(&mut sip2);
            let exit_code = zyxt::interpret(
                &zyxt::compile(content, filename, &mut typelist, &mut logger)
                    .unwrap_or_else(|e| e.print_exit(&mut StdIoPrint)),
                &mut i_data,
                &mut logger,
            )
            .unwrap_or_else(|e| e.print_exit(&mut StdIoPrint));
            exit(exit_code);
        }
        // TODO Compile, Interpret
        Subcmd::Repl => repl::repl(verbose),
    }
}
