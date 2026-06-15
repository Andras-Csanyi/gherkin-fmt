use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    find_examples_line_indices, find_parent_scenario_outline_index, join_lines, leading_spaces_count,
    split_lines,
};

pub const RULE_NAME: &str = "`Examples` blocks are indented by level 1 to `Scenario Outline` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let examples_indices = find_examples_line_indices(&lines);

    for examples_index in examples_indices {
        let Some(parent_index) = find_parent_scenario_outline_index(&lines, examples_index) else {
            continue;
        };

        let parent_indent = leading_spaces_count(&lines[parent_index]);
        let indent = " ".repeat(parent_indent + config.indent_size);
        let trimmed = lines[examples_index].trim_start();
        lines[examples_index] = format!("{indent}{trimmed}");
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_right_an_examples_block() {
        let input = r#"Scenario Outline: This is a scenario with the same indentation as the Feature
Examples:
"#;

        let expected = r#"Scenario Outline: This is a scenario with the same indentation as the Feature
  Examples:
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn shifts_right_two_examples_blocks() {
        let input = r#"Scenario Outline: This is the first scenario with the same indentation as the Feature
Examples:
Examples:
"#;

        let expected = r#"Scenario Outline: This is the first scenario with the same indentation as the Feature
  Examples:
  Examples:
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_mixed_indentation_examples_blocks() {
        let input = r#"Scenario Outline: This is the first scenario with the same indentation as the Feature
  Examples:
                Examples:
        Examples:
"#;

        let expected = r#"Scenario Outline: This is the first scenario with the same indentation as the Feature
  Examples:
  Examples:
  Examples:
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}