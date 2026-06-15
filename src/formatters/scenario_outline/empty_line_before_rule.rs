use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_scenario_outline_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "empty line before `Scenario Outline` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let scenario_outline_indices = find_scenario_outline_line_indices(&lines);

    for scenario_outline_index in scenario_outline_indices.into_iter().rev() {
        normalize_blank_lines_before(&mut lines, scenario_outline_index);
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn normalize_blank_lines_before(lines: &mut Vec<String>, mut scenario_outline_index: usize) {
    while scenario_outline_index > 0 && lines[scenario_outline_index - 1].trim().is_empty() {
        lines.remove(scenario_outline_index - 1);
        scenario_outline_index -= 1;
    }

    if scenario_outline_index > 0 {
        lines.insert(scenario_outline_index, String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_empty_line_before_scenario_outline_block() {
        let input = r#"Feature: This is a test input
  Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn removes_extra_lines_before_scenario_outline_block() {
        let input = r#"Feature: This is a test input



  Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario Outline: This is a scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}