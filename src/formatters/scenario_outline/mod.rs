mod empty_line_before_rule;
mod helpers;
mod indentation_rule;
mod keyword_is_capital_rule;
mod steps_in_scenario_outline_rule;

use anyhow::Result;

use super::Config;
use super::debug;

pub fn format_block(input: &str, config: &Config, debug_enabled: bool) -> Result<String> {
    debug::log(debug_enabled, "Starting Scenario Outline block formatting");

    debug::log_rule(debug_enabled, keyword_is_capital_rule::RULE_NAME);
    let mut content = keyword_is_capital_rule::apply(input, config)?;

    debug::log_rule(debug_enabled, empty_line_before_rule::RULE_NAME);
    content = empty_line_before_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, indentation_rule::RULE_NAME);
    content = indentation_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, steps_in_scenario_outline_rule::RULE_NAME);
    content = steps_in_scenario_outline_rule::apply(&content, config)?;

    debug::log(debug_enabled, "End Scenario Outline block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn formats_scenario_outline_block_simple_case() {
        let input = r#"Feature: test feature
Scenario Outline: test scenario
Given this is a not indented step
When another unindented step
Then once more
"#;

        let expected = r#"Feature: test feature

  Scenario Outline: test scenario
    Given this is a not indented step
    When another unindented step
    Then once more
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_scenario_outline_block_complex_case() {
        let input = r#"Feature: test feature


Scenario Outline: test scenario
  Given this is a not indented step
       And another one
    When another unindented step
             And another one again
    Then once more
And why not another one again
"#;

        let expected = r#"Feature: test feature

  Scenario Outline: test scenario
    Given this is a not indented step
    And another one
    When another unindented step
    And another one again
    Then once more
    And why not another one again
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_scenario_outline_block_chaos_case() {
        let input = r#"Feature: test feature





Scenario Outline: test scenario
  Given this is a not indented step
       And another one
    When another unindented step
             And another one again
    Then once more
And why not another one again
"#;

        let expected = r#"Feature: test feature

  Scenario Outline: test scenario
    Given this is a not indented step
    And another one
    When another unindented step
    And another one again
    Then once more
    And why not another one again
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn rule_names_match_spec_rule_keyword() {
        assert_eq!(
            keyword_is_capital_rule::RULE_NAME,
            "the `Scenario Outline` keyword is capitalized"
        );
        assert_eq!(
            empty_line_before_rule::RULE_NAME,
            "empty line before `Scenario Outline` block"
        );
        assert_eq!(
            indentation_rule::RULE_NAME,
            "`Scenario Outline` blocks are indented by level 1 to `Feature` blocks"
        );
        assert_eq!(
            steps_in_scenario_outline_rule::RULE_NAME,
            "Scenario Outline block steps alignment"
        );
    }
}