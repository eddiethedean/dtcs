//! DTCS command-line binary.

use clap::Parser;
use dtcs::cli::{run, Cli};

fn main() -> miette::Result<()> {
    let code = run(Cli::parse())?;
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}
