use std::{path::PathBuf, process::exit};

use clap::Parser;
use color_eyre::{config::HookBuilder, eyre::Result};
use itertools::Either;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use zyxt::{
    repl,
    types::sym_table::{InterpretSymTable, TypeCheckSymTable},
};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd,
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
    filename: PathBuf,
}

fn main() -> Result<()> {
    HookBuilder::new()
        .panic_section("If it is `not yet implemented`, handling of this will be complete in future versions.\nOtherwise, this shouldn't happen, open an issue on our GitHub: https://github.com/Segmential/zyxt/issues/new")
        .install()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(EnvFilter::from_env("RUST_LOG"))
        .with(ErrorLayer::default())
        .init();
    let args = Args::parse();

    match args.subcmd {
        Subcmd::Run(sargs) => {
            let mut ty_symt = TypeCheckSymTable::default();
            let mut val_symt = InterpretSymTable::default();
            let compiled = match zyxt::compile(&Either::Left(&sargs.filename), &mut ty_symt, true) {
                Ok(v) => v,
                Err(e) => {
                    e.print()?;
                    exit(1)
                }
            };
            let exit_code = match zyxt::interpret(&compiled, &mut val_symt) {
                Ok(v) => v,
                Err(e) => {
                    e.print()?;
                    exit(1)
                }
            };
            exit(exit_code);
        }
        Subcmd::Repl => repl::repl()?,
    }
    Ok(())
}
