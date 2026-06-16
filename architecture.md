# Architecture

## Project details

Project name: `gherkin-fmt`
Rust workspace file location: project root
Agents execution directory: project root
Rust source code directory: `{project_root}/src`
Rust workspace file location: project root
Rust workspace resolver: 3
Rust crate editions: 2024
Product: cli to format Gherkin files
Executable binary name: `gherkinfmt`
Initial version: 0.1.0
after successful build the executable cli is under the `src/cli/target/` directory

## Structure

The project is a CLI project.
The cli codebase is a module separated into the `src/cli/` module.
The codebase that contains the formatters is located in the `src/formatters/` module.

## Dependencies

We use the latest versions from the dependencies

- `clap` for cli implementation with `derive` features
- `clap_stdin` to manage the reading from `stdin`
- `anyhow` for error handling
- `assert_cmd` for testing the cli safely in CI

## Formatters module structure

The directory structure in the `{project_root}/spec/formatters` directory
represents the hierarchy and scope of the formatters.
For example the formatters specified in the
`{project_root/spec/formatter/feature-block-formatters` directory with the `*-rule.feature`
filename pattern are for the `Feature` block formatting.
These formatters must be used only for formatting the `Feature` block.

Every formatter function has its own module. For example, the `Feature` block
formatters are located in the `gherkin-fmt/src/feature/` module in the crate.
Every formatter rule has its own file that contains a single public method.
Any helper method in the file must be private.
Every public method of the rule has its own test set.

Every Gherkin specific block, like `Feature` or `Scenario` or `Table`, have two
levels of specifications:

- granular specification stored in the `*-rule.feature` files, where we focus on
  a single formatter functionality
- block level integration to have a clear picture about how the different rules
  work together, these cases are described in the `integration.feature` files.

## The cli module

The cli code is written in Clap's derive style.
The cli is capable of reading from `stdin` and write to `stdout`

### Testing of the CLI

We use `assert_cmd` to test the cli safely and without any environment
specificity.

## Error handling

Errors are bubbling up through the call chain layers where every layer provides information
where and what error happened, so it is easy to trace back where the error
happened and under what circumstances.
The `anyhow` crate provides the proper infrastructure to achieve the error
handling goal.

The error message is human readable and meaningful and also includes the Rust
specifics too.

## Debugging

It is possible to provide a debug flag to the cli and this controls whether the
application prints out debug information or not.
The informative debug message looks like the following:

- "DEBUG: Starting Feature block formatting"
- "DEBUG: File name: {the formatted file name}"
- "DEBUG: Executing {formatting rule}"
- "DEBUG: End Feature block formatting"

The {formatting rule}s are defined in the specific feature files using the
`Rule` gherkin keyword.
The purpose of the DEBUG messages is to provide enough information to see what
rules, when and in what order got executed.

## Testing

## Unit and integration testing

Every public method has its own functional test set.
The input and the expected result text for the formatter tests are in multi-line
raw text blocks, like the example below.
Human readability is paramount in these cases.

```rust
#[test]
fn preserves_multiple_lines_before_feature_block() {
    let input = r#"
# this is the first line
# this is the second line
Feature: this is some testing input
"#;

    let expected = r#"
# this is the first line
# this is the second line
Feature: this is some testing input
"#;

    assert_eq!(apply(input, &Config::default()).unwrap(), expected);
}
```

### Snapshot testing

We also use snapshot testing to get a quick overview if anything changed.
Under the `testing/` directory the same directory structure can be found as
under the `proc/formatters` directory.
Here there are `{filename}.feature` and `{filename}-result.feature` files.
The first represents the input and the latter is the expected result.
The `{project_root}/snapshot_testing.sh` file contains the formatter commands to
format the `{filename}.feature` files like the following:

```bash
./target/debug/gherkinfmt --debug --output test_files/{filename}-result.feature
test_files/{filename}.feature
```

Since both the input and the expected result files are added to the git repo
after a single run we can see if there is any changes in the expected result.
