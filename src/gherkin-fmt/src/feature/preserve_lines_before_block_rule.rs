use anyhow::Result;

use crate::config::Config;

pub const RULE_NAME: &str = "preserve lines before `Feature` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::format_block;

    #[test]
    fn preserves_single_line_before_feature_block() {
        let input = r#"# this is some comment
Feature: this is some testing input
"#;

        let expected = r#"# this is some comment
Feature: this is some testing input
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_multiple_lines_before_feature_block() {
        let input = r#"
# this is the first line
# this is the second line
Feature: this is some testing input
"#;

        let expected = r#"
# this is the first line
# this is the second line
Feature: this is some testing input
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_prefix_when_feature_block_is_formatted() {
        let input = r#"# this should be preserver
Feature: this is a test feature
unindented user story
"#;

        let expected = r#"# this should be preserver
Feature: this is a test feature

  unindented user story
"#;

        assert_eq!(
            format_block(input, &Config::default(), false, None).unwrap(),
            expected
        );
    }
}