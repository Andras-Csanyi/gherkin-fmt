use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_scenario_outline_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "`Scenario Outline` blocks are indented by level 1 to `Feature` blocks";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let scenario_outline_indices = find_scenario_outline_line_indices(&lines);
    let indent = " ".repeat(config.indent_size);

    for scenario_outline_index in scenario_outline_indices {
        let trimmed = lines[scenario_outline_index].trim_start();
        lines[scenario_outline_index] = format!("{indent}{trimmed}");
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_right_a_scenario_outline_block() {
        let input = r#"Feature: This is a test input

Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn shifts_right_two_scenario_outline_blocks() {
        let input = r#"Feature: This is a test input

Scenario Outline: This is the first scenario with the same indentation as the Feature
Scenario Outline: This is the second scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario Outline: This is the first scenario with the same indentation as the Feature
  Scenario Outline: This is the second scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_mixed_indentation_scenario_outline_blocks() {
        let input = r#"Feature: This is a test input

        Scenario Outline: This is the first scenario with the same indentation as the Feature
  Scenario Outline: This is the second scenario with the same indentation as the Feature
     Scenario Outline: This is the third scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario Outline: This is the first scenario with the same indentation as the Feature
  Scenario Outline: This is the second scenario with the same indentation as the Feature
  Scenario Outline: This is the third scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}