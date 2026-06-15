use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::CommandFactory;
use clap::Parser;
use clap_stdin::FileOrStdin;

use crate::formatters::Config;
use crate::formatters::formatter::format;

#[derive(Parser)]
#[command(name = "gherkinfmt", version, about = "Format Gherkin feature files")]
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
            println!();
            return Ok(());
        }
    };

    let reads_from_stdin = input.is_stdin();
    let source_label = input.filename().to_owned();

    let input_content = input
        .contents()
        .context("failed to read input content")?;

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

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};

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

    #[test]
    fn file_or_stdin_recognizes_dash() {
        let cli = Cli::try_parse_from(["gherkinfmt", "-"]).expect("failed to parse cli args");
        let input = cli.input.expect("input argument should be present");
        assert!(input.is_stdin());
        assert!(!input.is_file());
        assert_eq!(input.filename(), "-");
    }

    #[test]
    fn file_or_stdin_recognizes_file_path() {
        let cli =
            Cli::try_parse_from(["gherkinfmt", "test.feature"]).expect("failed to parse cli args");
        let input = cli.input.expect("input argument should be present");
        assert!(input.is_file());
        assert!(!input.is_stdin());
        assert_eq!(input.filename(), "test.feature");
    }

    fn gherkin_fmt_binary() -> PathBuf {
        if let Ok(path) = env::var("CARGO_BIN_EXE_gherkin-fmt") {
            return PathBuf::from(path);
        }

        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "target/debug/gherkin-fmt{}",
            env::consts::EXE_SUFFIX
        ))
    }

    #[test]
    fn reads_from_stdin_and_writes_to_stdout() {
        let binary = gherkin_fmt_binary();
        assert!(
            binary.exists(),
            "gherkin-fmt binary not found at {}",
            binary.display()
        );

        let mut child = Command::new(binary)
            .arg("--")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn gherkin-fmt");

        child
            .stdin
            .as_mut()
            .expect("failed to open stdin")
            .write_all(b"feature: test feature\n")
            .expect("failed to write to stdin");

        let output = child.wait_with_output().expect("failed to wait for gherkin-fmt");

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout).expect("stdout is not valid utf-8"),
            "Feature: test feature\n"
        );
    }
}