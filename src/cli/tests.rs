use std::fs;

use assert_cmd::Command;
use clap::CommandFactory;
use clap::Parser;
use predicates::prelude::*;

use super::Cli;

fn gherkin_fmt() -> Command {
    Command::cargo_bin("gherkin-fmt").expect("gherkin-fmt binary should be built for tests")
}

#[test]
fn command_has_expected_name() {
    let command = Cli::command();
    assert_eq!(command.get_name(), "gherkin-fmt");
}

#[test]
fn command_has_version_set() {
    let command = Cli::command();
    assert!(command.get_version().is_some());
}

#[test]
fn file_or_stdin_recognizes_dash() {
    let cli = Cli::try_parse_from(["gherkin-fmt", "-"]).expect("failed to parse cli args");
    let input = cli.input.expect("input argument should be present");
    assert!(input.is_stdin());
    assert!(!input.is_file());
    assert_eq!(input.filename(), "-");
}

#[test]
fn file_or_stdin_recognizes_file_path() {
    let cli =
        Cli::try_parse_from(["gherkin-fmt", "test.feature"]).expect("failed to parse cli args");
    let input = cli.input.expect("input argument should be present");
    assert!(input.is_file());
    assert!(!input.is_stdin());
    assert_eq!(input.filename(), "test.feature");
}

#[test]
fn reads_from_stdin_and_writes_to_stdout() {
    gherkin_fmt()
        .arg("--")
        .arg("-")
        .write_stdin("feature: test feature\n")
        .assert()
        .success()
        .stdout("Feature: test feature\n");
}

#[test]
fn reads_input_file_and_writes_output_file() {
    let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
    let input_path = temp_dir.path().join("test.feature");
    let output_path = temp_dir.path().join("test-result.feature");

    fs::write(
        &input_path,
        "Feature: test feature\nMisaligned user story part\n",
    )
    .expect("failed to write input feature file");

    gherkin_fmt()
        .arg("--output")
        .arg(&output_path)
        .arg(&input_path)
        .assert()
        .success()
        .stdout("");

    let expected = "Feature: test feature\n\n  Misaligned user story part\n";
    assert_eq!(
        fs::read_to_string(&output_path).expect("failed to read output feature file"),
        expected
    );
}

#[test]
fn prints_help_when_no_arguments() {
    let help_output = gherkin_fmt()
        .arg("--help")
        .output()
        .expect("failed to run gherkin-fmt --help");

    let no_args_output = gherkin_fmt()
        .output()
        .expect("failed to run gherkin-fmt without arguments");

    assert_eq!(no_args_output.status.code(), Some(0));
    assert_eq!(help_output.stdout, no_args_output.stdout);
}

#[test]
fn prints_help_with_help_flag() {
    gherkin_fmt()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn prints_version_with_version_flag() {
    gherkin_fmt()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

