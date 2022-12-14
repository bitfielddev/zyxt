use std::{path::PathBuf, process::exit};

use clap::Parser;
use color_eyre::config::HookBuilder;
use itertools::Either;
use tracing_subscriber::EnvFilter;
use zyxt::{
    ast::Ast,
    repl,
    types::{interpreter_data::SymTable, typeobj::Type, value::Value},
};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long, action = clap::ArgAction::Count)]
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
    HookBuilder::new()
        .panic_section("This shouldn't happen!\nOpen an issue on our GitHub: https://github.com/Segmential/zyxt/issues/new")
        .install().unwrap();
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();
    let args = Args::parse();
    let verbose = args.verbose;

    match args.subcmd {
        Subcmd::Run(sargs) => {
            let filename = PathBuf::try_from(sargs.filename).unwrap(); // TODO
            let mut typelist = SymTable::<Type<Ast>>::default();
            let mut i_data = SymTable::<Value>::default();
            let exit_code = zyxt::interpret(
                &zyxt::compile(&Either::Left(&filename), &mut typelist)
                    .unwrap_or_else(|e| e.print_exit()),
                &mut i_data,
            )
            .unwrap_or_else(|e| e.print_exit());
            exit(exit_code);
        }
        // TODO Compile, Interpret
        Subcmd::Repl => repl::repl(verbose),
    }
}
