use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_background_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "empty line before `Background` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let background_indices = find_background_line_indices(&lines);

    for background_index in background_indices.into_iter().rev() {
        normalize_blank_lines_before(&mut lines, background_index);
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn normalize_blank_lines_before(lines: &mut Vec<String>, mut background_index: usize) {
    while background_index > 0 && lines[background_index - 1].trim().is_empty() {
        lines.remove(background_index - 1);
        background_index -= 1;
    }

    if background_index > 0 {
        lines.insert(background_index, String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_empty_line_before_background_block() {
        let input = r#"Feature: This is a test input
  Background: This is a background with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Background: This is a background with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn removes_extra_lines_before_background_block() {
        let input = r#"Feature: This is a test input



  Background: This is a Background with the same indentation as the Feature
"#;

        let expected = r#"Feature: This is a test input

  Background: This is a Background with the same indentation as the Feature
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}