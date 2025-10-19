//! Commandline interface API
use std::path::PathBuf;

use clap::Parser;
use miette::{IntoDiagnostic as _, Result};

use crate::Space;
use crate::errors::IOParseError;

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
    run_inner().into_diagnostic()
}

fn run_inner() -> Result<(), IOParseError> {
    let opts = Options::parse();
    let _space = Space::try_from(opts.source)?;
    todo!();
}
