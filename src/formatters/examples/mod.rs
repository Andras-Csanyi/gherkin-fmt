mod empty_line_before_rule;
mod helpers;
mod indentation_rule;
mod keyword_is_capital_rule;

use anyhow::Result;

use super::Config;
use super::debug;

pub fn format_block(input: &str, config: &Config, debug_enabled: bool) -> Result<String> {
    debug::log(debug_enabled, "Starting Examples block formatting");

    debug::log_rule(debug_enabled, keyword_is_capital_rule::RULE_NAME);
    let mut content = keyword_is_capital_rule::apply(input, config)?;

    debug::log_rule(debug_enabled, empty_line_before_rule::RULE_NAME);
    content = empty_line_before_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, indentation_rule::RULE_NAME);
    content = indentation_rule::apply(&content, config)?;

    debug::log(debug_enabled, "End Examples block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn formats_examples_block_simple_case() {
        let input = r#"Scenario Outline: test scenario outline
Given an outline step
Examples:
|header 1|header 2|
|value 1|value 2|
"#;

        let expected = r#"Scenario Outline: test scenario outline
Given an outline step

  Examples:
|header 1|header 2|
|value 1|value 2|
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_examples_block_with_mixed_indentation() {
        let input = r#"Scenario Outline: test scenario outline
  Given an outline step
Examples:
|header 1|header 2|
"#;

        let expected = r#"Scenario Outline: test scenario outline
  Given an outline step

  Examples:
|header 1|header 2|
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_examples_block_with_lowercase_keyword() {
        let input = r#"Scenario Outline: test scenario outline
Given an outline step
examples:
"#;

        let expected = r#"Scenario Outline: test scenario outline
Given an outline step

  Examples:
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
            "the `Examples` keyword is capitalized"
        );
        assert_eq!(
            empty_line_before_rule::RULE_NAME,
            "empty line before `Examples` block"
        );
        assert_eq!(
            indentation_rule::RULE_NAME,
            "`Examples` blocks are indented by level 1 to `Scenario Outline` block"
        );
    }
}