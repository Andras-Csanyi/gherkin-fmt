# gherkin-fmt

A CLI tool to format Gherkin files.

## Features

- Format Gherkin feature files 
- Support for Feature, Scenario, and Table blocks
- Debug mode for tracking formatting rules
- Error handling with meaningful messages

## Installation

```bash
cargo install gherkin-fmt
```

## Usage

```bash
gherkinfmt input.feature [output.feature] --debug
```

## Architecture

This project follows a Rust workspace structure with two crates:

1. `gherkin-fmt-cli` - CLI implementation using clap
2. `gherkin-fmt` - Core formatter logic

The formatter uses specification-driven development with Gherkin feature files to define formatting rules at granular and integration levels.