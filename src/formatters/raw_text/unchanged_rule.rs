use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_raw_block_delimiter_pairs, find_raw_blocks, join_lines, split_lines};

pub const RULE_NAME: &str = "the raw text remains unchanged";

pub fn apply(formatted: &str, original: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut formatted_lines, ends_with_newline) = split_lines(formatted);
    let (original_lines, _) = split_lines(original);

    let original_blocks = find_raw_blocks(&original_lines);
    let delimiter_pairs = find_raw_block_delimiter_pairs(&formatted_lines);

    if original_blocks.len() != delimiter_pairs.len() {
        anyhow::bail!(
            "formatted content has {} raw text blocks but original content has {}",
            delimiter_pairs.len(),
            original_blocks.len()
        );
    }

    for (block, (opening_index, closing_index)) in original_blocks
        .iter()
        .zip(delimiter_pairs.iter().copied())
        .rev()
    {
        formatted_lines.splice(
            opening_index + 1..closing_index,
            block.content_lines.iter().cloned(),
        );
    }

    Ok(join_lines(&formatted_lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_backtick_block_content() {
        let original = r#"given a step
  ```
  And   extra   spaces
  | not | a | table |
  ```
"#;

        let formatted = r#"Given a step

  ```
  And extra spaces
  | not|a|table|
  ```
"#;

        let expected = r#"Given a step

  ```
  And   extra   spaces
  | not | a | table |
  ```
"#;

        assert_eq!(
            apply(formatted, original, &Config::default()).unwrap(),
            expected
        );
    }

    #[test]
    fn preserves_triple_quote_block_content() {
        let original = r#"given a step
  """
  and   preserved   spacing
  stays lowercase
  """
"#;

        let formatted = r#"Given a step

  """
  And preserved spacing
  Stays lowercase
  """
"#;

        let expected = r#"Given a step

  """
  and   preserved   spacing
  stays lowercase
  """
"#;

        assert_eq!(
            apply(formatted, original, &Config::default()).unwrap(),
            expected
        );
    }

    #[test]
    fn restores_original_content_when_formatters_added_lines_inside_block() {
        let original = r#"Given this step has raw content
    ```gherkin
    Feature: whatever feature
      Scenario: whatever scenario
        Given whatever step
        And the whole must remain unchanged
    ```
"#;

        let formatted = r#"Given this step has raw content
    ```gherkin

    Feature: whatever feature

      Scenario: whatever scenario
        Given whatever step
        And the whole must remain unchanged
    ```
"#;

        assert_eq!(
            apply(formatted, original, &Config::default()).unwrap(),
            original
        );
    }

    #[test]
    fn leaves_formatted_content_when_no_raw_blocks_exist() {
        let input = r#"Given   a   step
And another   step
"#;

        assert_eq!(apply(input, input, &Config::default()).unwrap(), input);
    }
}