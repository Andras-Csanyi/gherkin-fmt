use anyhow::Result;

use super::Config;
use super::background;
use super::examples;
use super::feature;
use super::raw_text;
use super::scenario;
use super::scenario_outline;
use super::step;
use super::table;

pub fn format(
    input: &str,
    config: &Config,
    debug_enabled: bool,
    file_name: Option<&str>,
) -> Result<String> {
    let original = input;
    let content = feature::format_block(input, config, debug_enabled, file_name)?;
    let content = background::format_block(&content, config, debug_enabled)?;
    let content = scenario_outline::format_block(&content, config, debug_enabled)?;
    let content = scenario::format_block(&content, config, debug_enabled)?;
    let content = step::format_block(&content, config, debug_enabled)?;
    let content = examples::format_block(&content, config, debug_enabled)?;
    let content = table::format_block(&content, config, debug_enabled)?;
    raw_text::format_block(&content, original, config, debug_enabled)
}

#[cfg(test)]
mod tests {
    use crate::formatters::Config;

    use super::*;

    #[test]
    fn format_applies_feature_block_rules() {
        let input = r#"Feature: test
user story
"#;

        let expected = r#"Feature: test

  user story
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_feature_and_scenario_block_rules() {
        let input = r#"Feature: test feature
Scenario: test scenario
Given a step
"#;

        let expected = r#"Feature: test feature

  Scenario: test scenario
    Given a step
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_feature_scenario_and_table_block_rules() {
        let input = r#"Feature: test
Scenario: example
When a table step
|a|b|
"#;

        let expected = r#"Feature: test

  Scenario: example
    When a table step
      | a | b |
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_feature_scenario_step_and_table_block_rules() {
        let input = r#"Feature: test
Scenario: example
given extra   spaces here
When a table step
|a|b|
"#;

        let expected = r#"Feature: test

  Scenario: example
    Given extra spaces here
    When a table step
      | a | b |
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_feature_background_and_scenario_block_rules() {
        let input = r#"Feature: test feature
Background: test background
Given a background step
Scenario: test scenario
Given a scenario step
"#;

        let expected = r#"Feature: test feature

  Background: test background
    Given a background step

  Scenario: test scenario
    Given a scenario step
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_feature_and_scenario_outline_block_rules() {
        let input = r#"Feature: test feature
Scenario Outline: test scenario outline
Given an outline step
Scenario: test scenario
Given a scenario step
"#;

        let expected = r#"Feature: test feature

  Scenario Outline: test scenario outline
    Given an outline step

  Scenario: test scenario
    Given a scenario step
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_applies_scenario_outline_examples_and_table_block_rules() {
        let input = r#"Feature: test feature
Scenario Outline: test scenario outline
Given an outline step
examples:
|header 1|header 2|
|value 1|value 2|
"#;

        let expected = r#"Feature: test feature

  Scenario Outline: test scenario outline
    Given an outline step

    Examples:
      | header 1 | header 2 |
      | value 1  | value 2  |
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }

    #[test]
    fn format_preserves_raw_text_blocks_in_steps() {
        let input = r#"Feature: test
Scenario: raw text
given a step with backticks
  ```
  And   raw   backtick   content
  | not | a | table |
  ```
and a step with quotes
  """
  and   raw   quote   content
  """
"#;

        let expected = r#"Feature: test

  Scenario: raw text
    Given a step with backticks
  ```
  And   raw   backtick   content
  | not | a | table |
  ```
    And a step with quotes
  """
  and   raw   quote   content
  """
"#;

        assert_eq!(
            format(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }
}

