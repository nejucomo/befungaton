//! Commandline interface API
use std::path::PathBuf;

use clap::Parser;
use miette::Result;

/// befungaton - a befunge-like interpreter
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Options {
    /// The source file to run
    source: PathBuf,
}

/// Run the curses interpreter
pub fn run() -> Result<()> {
    miette::set_panic_hook();
    let opts = Options::parse();
    todo!("{opts:#?}");
}
