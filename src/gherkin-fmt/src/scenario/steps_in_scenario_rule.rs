use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    find_scenario_line_indices, is_step_line, join_lines, leading_spaces_count,
    scenario_block_end, split_lines,
};

pub const RULE_NAME: &str = "Scenario block steps alignment";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let scenario_indices = find_scenario_line_indices(&lines);

    for scenario_index in scenario_indices {
        let scenario_indent = leading_spaces_count(&lines[scenario_index]);
        let step_indent = " ".repeat(scenario_indent + config.indent_size);
        let block_end = scenario_block_end(&lines, scenario_index);

        for line_index in (scenario_index + 1)..block_end {
            if is_step_line(&lines[line_index]) {
                let trimmed = lines[line_index].trim_start();
                lines[line_index] = format!("{step_indent}{trimmed}");
            }
        }
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indents_single_step_in_scenario_block() {
        let input = r#"Scenario: this is a test scenario
Given this is a not indented step
"#;

        let expected = r#"Scenario: this is a test scenario
  Given this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn indents_two_steps_in_scenario_block() {
        let input = r#"Scenario: this is a test scenario
Given this is a not indented step
And this is a not indented step
"#;

        let expected = r#"Scenario: this is a test scenario
  Given this is a not indented step
  And this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn indents_three_steps_in_scenario_block() {
        let input = r#"Scenario: this is a test scenario
Given this is a not indented step
When this is a not indented step
Then this is a not indented step
"#;

        let expected = r#"Scenario: this is a test scenario
  Given this is a not indented step
  When this is a not indented step
  Then this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_chaotically_indented_steps_in_scenario_block() {
        let input = r#"Scenario: this is a test scenario
    Given this is a not indented step
          And here is some chaos
    When this is a not indented step
        And you want more
Then this is a not indented step
   And here is more
"#;

        let expected = r#"Scenario: this is a test scenario
  Given this is a not indented step
  And here is some chaos
  When this is a not indented step
  And you want more
  Then this is a not indented step
  And here is more
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}