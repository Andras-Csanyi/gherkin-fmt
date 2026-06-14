mod column_width_padding_rule;
mod helpers;
mod indentation_in_steps_rule;
mod one_space_padding_at_left_rule;

use anyhow::Result;

use super::Config;
use super::debug;

pub fn format_block(input: &str, config: &Config, debug_enabled: bool) -> Result<String> {
    debug::log(debug_enabled, "Starting Table block formatting");

    debug::log_rule(debug_enabled, indentation_in_steps_rule::RULE_NAME);
    let mut content = indentation_in_steps_rule::apply(input, config)?;

    debug::log_rule(debug_enabled, one_space_padding_at_left_rule::RULE_NAME);
    content = one_space_padding_at_left_rule::apply(&content, config)?;

    debug::log_rule(debug_enabled, column_width_padding_rule::RULE_NAME);
    content = column_width_padding_rule::apply(&content, config)?;

    debug::log(debug_enabled, "End Table block formatting");

    Ok(content)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn formats_table_integration_case_one() {
        let input = r#"Given this step doesn't have a table
When this step does have a table
|t 1|t 2|
|asdffffff|ga|
|gagggg|aasdfafasdfsasdfasd|
Then we are at the end
"#;

        let expected = r#"Given this step doesn't have a table
When this step does have a table
  | t 1       | t 2                 |
  | asdffffff | ga                  |
  | gagggg    | aasdfafasdfsasdfasd |
Then we are at the end
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn formats_table_integration_case_two() {
        let input = r#"Given this step doesn't have a table
  |a 1|a 2|a 3|
  |as|fassss|fasgaadfasdfawer|
  |lalalal|dfa|hlahsdhfa|
  |akld|fasd|hwoqurakdfja;sdl|
When this step does have a table
|t 1|t 2|
|asdffffff|ga|
|gagggg|aasdfafasdfsasdfasd|
Then we are at the end
"#;

        let expected = r#"Given this step doesn't have a table
  | a 1     | a 2    | a 3              |
  | as      | fassss | fasgaadfasdfawer |
  | lalalal | dfa    | hlahsdhfa        |
  | akld    | fasd   | hwoqurakdfja;sdl |
When this step does have a table
  | t 1       | t 2                 |
  | asdffffff | ga                  |
  | gagggg    | aasdfafasdfsasdfasd |
Then we are at the end
"#;

        assert_eq!(
            format_block(input, &Config::default(), false).unwrap(),
            expected
        );
    }

    #[test]
    fn rule_names_match_spec_rule_keyword() {
        assert_eq!(
            indentation_in_steps_rule::RULE_NAME,
            "Vertical table is indented by 1 level in steps"
        );
        assert_eq!(
            one_space_padding_at_left_rule::RULE_NAME,
            "One space left padding in the cell"
        );
        assert_eq!(column_width_padding_rule::RULE_NAME, "column width padding");
    }
}

