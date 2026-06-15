mod cli;
mod formatters;

use std::process;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(error) = cli::run(cli) {
        eprintln!("{error:#}");
        process::exit(1);
    }
}