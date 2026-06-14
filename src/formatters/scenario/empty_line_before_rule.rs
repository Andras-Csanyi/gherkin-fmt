use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_scenario_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "empty line before `Scenario` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let scenario_indices = find_scenario_line_indices(&lines);

    for scenario_index in scenario_indices.into_iter().rev() {
        normalize_blank_lines_before(&mut lines, scenario_index);
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn normalize_blank_lines_before(lines: &mut Vec<String>, mut scenario_index: usize) {
    while scenario_index > 0 && lines[scenario_index - 1].trim().is_empty() {
        lines.remove(scenario_index - 1);
        scenario_index -= 1;
    }

    if scenario_index > 0 {
        lines.insert(scenario_index, String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_empty_line_before_scenario_block() {
        let input = r#"Feature: This is a test input
  Scenario: This is a scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario: This is a scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn removes_extra_lines_before_scenario_block() {
        let input = r#"Feature: This is a test input



  Scenario: This is a scenario with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Scenario: This is a scenario with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}

