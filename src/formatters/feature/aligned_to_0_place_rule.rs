use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_feature_line_index, join_lines, leading_spaces_count, split_lines};

pub const RULE_NAME: &str = "`Feature` block is aligned to 0 place";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let feature_index = find_feature_line_index(&lines)?;
    lines[feature_index] = align_feature_line(&lines[feature_index]);
    Ok(join_lines(&lines, ends_with_newline))
}

fn align_feature_line(line: &str) -> String {
    if leading_spaces_count(line) == 0 {
        return line.to_string();
    }

    line.trim_start().to_string()
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
}

