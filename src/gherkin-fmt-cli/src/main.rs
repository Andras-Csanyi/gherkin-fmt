use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use gherkin_fmt::{format, Config};

#[derive(Parser)]
#[command(name = "gherkinfmt", version, about = "Format Gherkin feature files")]
struct Cli {
    /// Print debug information during formatting
    #[arg(long)]
    debug: bool,

    /// Write formatted output to FILE instead of stdout
    #[arg(long, short, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Gherkin feature file to format
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
}

fn run(cli: Cli) -> Result<()> {
    let input_path = match cli.input {
        Some(path) => path,
        None => {
            Cli::command()
                .print_help()
                .context("failed to print help information")?;
            println!();
            return Ok(());
        }
    };

    let input_content = fs::read_to_string(&input_path).with_context(|| {
        format!(
            "failed to read input file: {}",
            input_path.display()
        )
    })?;

    let config = Config::default();
    let file_name = input_path.to_str();
    let formatted = format(&input_content, &config, cli.debug, file_name)
        .context("failed to format Gherkin content")?;

    write_output(cli.output.as_ref(), &formatted, &input_path)
}

fn write_output(
    output_path: Option<&PathBuf>,
    content: &str,
    input_path: &PathBuf,
) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, content).with_context(|| {
                format!("failed to write output file: {}", path.display())
            })?;
        }
        None => {
            let mut stdout = io::stdout();
            stdout.write_all(content.as_bytes()).with_context(|| {
                format!(
                    "failed to write formatted output for: {}",
                    input_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("{error:#}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_has_expected_name() {
        let command = Cli::command();
        assert_eq!(command.get_name(), "gherkinfmt");
    }

    #[test]
    fn command_has_version_set() {
        let command = Cli::command();
        assert!(command.get_version().is_some());
    }
}