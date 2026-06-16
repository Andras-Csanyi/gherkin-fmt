use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::CommandFactory;
use clap::Parser;
use clap_stdin::FileOrStdin;

use crate::formatters::Config;
use crate::formatters::formatter::format;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(name = "gherkin-fmt", version, about = "Format Gherkin feature files")]
pub struct Cli {
    /// Print debug information during formatting
    #[arg(long)]
    debug: bool,

    /// Write formatted output to FILE instead of stdout
    #[arg(long, short, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Gherkin feature file to format, or `-` to read from stdin
    #[arg(value_name = "FILE")]
    input: Option<FileOrStdin<String>>,
}

pub fn run(cli: Cli) -> Result<()> {
    let input = match cli.input {
        Some(input) => input,
        None => {
            Cli::command()
                .print_help()
                .context("failed to print help information")?;
            return Ok(());
        }
    };

    let reads_from_stdin = input.is_stdin();
    let source_label = input.filename().to_owned();

    let input_content = read_input_content(input)?;

    let file_name = if reads_from_stdin {
        None
    } else {
        Some(source_label.as_str())
    };

    let config = Config::default();
    let formatted = format(&input_content, &config, cli.debug, file_name)
        .context("failed to format Gherkin content")?;

    write_output(
        cli.output.as_ref(),
        &formatted,
        reads_from_stdin,
        &source_label,
    )
}

fn read_input_content(input: FileOrStdin<String>) -> Result<String> {
    let mut reader = input
        .into_reader()
        .context("failed to open input source")?;
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .context("failed to read input content")?;
    Ok(buffer)
}

fn write_output(
    output_path: Option<&PathBuf>,
    content: &str,
    reads_from_stdin: bool,
    source_label: &str,
) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, content)
                .with_context(|| format!("failed to write output file: {}", path.display()))?;
        }
        None => {
            let error_context = if reads_from_stdin {
                "failed to write formatted output to stdout".to_string()
            } else {
                format!("failed to write formatted output for: {source_label}")
            };

            io::stdout()
                .write_all(content.as_bytes())
                .context(error_context)?;
        }
    }

    Ok(())
}