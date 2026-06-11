use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    feature_block_end, find_feature_line_index, join_lines, split_lines,
};

pub const RULE_NAME: &str = "`Feature` block's story is indented by 1 level";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (lines, ends_with_newline) = split_lines(input);
    let feature_index = find_feature_line_index(&lines)?;
    let block_end = feature_block_end(&lines, feature_index);

    let mut formatted_lines = lines[..feature_index].to_vec();
    formatted_lines.extend(format_story_section(
        &lines,
        feature_index,
        block_end,
        config.indent_size,
    ));
    formatted_lines.extend_from_slice(&lines[block_end..]);

    Ok(join_lines(&formatted_lines, ends_with_newline))
}

fn format_story_section(
    lines: &[String],
    feature_index: usize,
    block_end: usize,
    indent_size: usize,
) -> Vec<String> {
    let story_lines = collect_story_lines(lines, feature_index + 1, block_end);
    let mut formatted = vec![lines[feature_index].clone()];

    if story_lines.is_empty() {
        formatted.extend_from_slice(&lines[feature_index + 1..block_end]);
        return formatted;
    }

    formatted.push(String::new());

    let indent = " ".repeat(indent_size);
    for story_line in story_lines {
        formatted.push(format!("{indent}{story_line}"));
    }

    formatted
}

fn collect_story_lines(lines: &[String], start: usize, end: usize) -> Vec<String> {
    lines[start..end]
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_user_story_is_indented() {
        let input = r#"Feature: this is a feature block

This is the unaligned single line user story
"#;

        let expected = r#"Feature: this is a feature block

  This is the unaligned single line user story
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn two_line_user_story_is_indented() {
        let input = r#"Feature: this is a feature block

This is the first unaligned line
This is the second unaligned line
"#;

        let expected = r#"Feature: this is a feature block

  This is the first unaligned line
  This is the second unaligned line
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn multi_line_user_story_is_indented() {
        let input = r#"Feature: this is a feature block

This is the first unaligned line
This is the second unaligned line
This is the third unaligned line
This is the fourth unaligned line
"#;

        let expected = r#"Feature: this is a feature block

  This is the first unaligned line
  This is the second unaligned line
  This is the third unaligned line
  This is the fourth unaligned line
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn overindented_one_line_user_story_is_aligned() {
        let input = r#"Feature: this is a feature block

      This is the unaligned single line user story
"#;

        let expected = r#"Feature: this is a feature block

  This is the unaligned single line user story
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn overindented_two_line_user_story_is_aligned() {
        let input = r#"Feature: this is a feature block

    This is the first unaligned line
    This is the second unaligned line
"#;

        let expected = r#"Feature: this is a feature block

  This is the first unaligned line
  This is the second unaligned line
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn overindented_multi_line_user_story_is_aligned() {
        let input = r#"Feature: this is a feature block

        This is the first unaligned line
        This is the second unaligned line
        This is the third unaligned line
        This is the fourth unaligned line
"#;

        let expected = r#"Feature: this is a feature block

  This is the first unaligned line
  This is the second unaligned line
  This is the third unaligned line
  This is the fourth unaligned line
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn mix_indented_multi_line_user_story_is_aligned() {
        let input = r#"Feature: this is a feature block

This is the first unaligned line
        This is the second unaligned line
      This is the third unaligned line
              This is the fourth unaligned line
"#;

        let expected = r#"Feature: this is a feature block

  This is the first unaligned line
  This is the second unaligned line
  This is the third unaligned line
  This is the fourth unaligned line
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}