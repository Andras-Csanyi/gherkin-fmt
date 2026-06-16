mod helpers;
mod unchanged_rule;

use anyhow::Result;

use super::Config;
use super::debug;

pub fn format_block(
    formatted: &str,
    original: &str,
    config: &Config,
    debug_enabled: bool,
) -> Result<String> {
    debug::log(debug_enabled, "Starting Raw text block formatting");

    debug::log_rule(debug_enabled, unchanged_rule::RULE_NAME);
    let content = unchanged_rule::apply(formatted, original, config)?;

    debug::log(debug_enabled, "End Raw text block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn preserves_raw_text_blocks_after_formatting_changes() {
        let original = r#"given a step with backticks
  ```
  And   raw   content
  ```
given a step with quotes
  """
  and   more   raw
  """
"#;

        let formatted = r#"Given a step with backticks
  ```
  And raw content
  ```
Given a step with quotes
  """
  And more raw
  """
"#;

        let expected = r#"Given a step with backticks
  ```
  And   raw   content
  ```
Given a step with quotes
  """
  and   more   raw
  """
"#;

        assert_eq!(
            format_block(formatted, original, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn rule_names_match_spec_rule_keyword() {
        assert_eq!(
            unchanged_rule::RULE_NAME,
            "the raw text remains unchanged"
        );
    }
}