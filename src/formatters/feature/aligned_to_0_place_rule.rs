use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{capitalize_feature_keyword, find_feature_line_index, join_lines, split_lines};

pub const RULE_NAME: &str = "`Feature` block is aligned to 0 place";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let feature_index = find_feature_line_index(&lines)?;
    lines[feature_index] = align_feature_line(&lines[feature_index]);
    Ok(join_lines(&lines, ends_with_newline))
}

fn align_feature_line(line: &str) -> String {
    capitalize_feature_keyword(line.trim_start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_at_zero_place_remains_unchanged() {
        let input = r#"Feature: this is already at the 0 place
"#;

        let expected = r#"Feature: this is already at the 0 place
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn feature_at_place_two_is_aligned_to_zero() {
        let input = r#"  Feature: this block is at place 2
"#;

        let expected = r#"Feature: this block is at place 2
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn feature_at_place_three_is_aligned_to_zero() {
        let input = r#"   Feature: this block is at place 3
"#;

        let expected = r#"Feature: this block is at place 3
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn feature_at_place_four_is_aligned_to_zero() {
        let input = r#"    Feature: this block is at place 4
"#;

        let expected = r#"Feature: this block is at place 4
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_feature_keyword_at_zero_place() {
        let input = r#"feature: test feature
"#;

        let expected = r#"Feature: test feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}

