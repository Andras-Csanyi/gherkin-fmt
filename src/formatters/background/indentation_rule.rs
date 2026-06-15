use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_background_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "`Background` blocks are indented by level 1 to `Feature` blocks";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let background_indices = find_background_line_indices(&lines);
    let indent = " ".repeat(config.indent_size);

    for background_index in background_indices {
        let trimmed = lines[background_index].trim_start();
        lines[background_index] = format!("{indent}{trimmed}");
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_right_a_background_block() {
        let input = r#"Feature: This is a test input

Background: This is a background with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Background: This is a background with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}