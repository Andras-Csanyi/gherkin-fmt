use anyhow::Result;

use super::Config;
use super::feature;
use super::scenario;
use super::step;
use super::table;

pub fn format(
    input: &str,
    config: &Config,
    debug_enabled: bool,
    file_name: Option<&str>,
) -> Result<String> {
    let content = feature::format_block(input, config, debug_enabled, file_name)?;
    let content = scenario::format_block(&content, config, debug_enabled)?;
    let content = step::format_block(&content, config, debug_enabled)?;
    table::format_block(&content, config, debug_enabled)
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
}

