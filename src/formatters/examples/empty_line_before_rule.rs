use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{find_examples_line_indices, join_lines, split_lines};

pub const RULE_NAME: &str = "empty line before `Examples` block";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let examples_indices = find_examples_line_indices(&lines);

    for examples_index in examples_indices.into_iter().rev() {
        normalize_blank_lines_before(&mut lines, examples_index);
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn normalize_blank_lines_before(lines: &mut Vec<String>, mut examples_index: usize) {
    while examples_index > 0 && lines[examples_index - 1].trim().is_empty() {
        lines.remove(examples_index - 1);
        examples_index -= 1;
    }

    if examples_index > 0 {
        lines.insert(examples_index, String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_empty_line_before_examples_block() {
        let input = r#"  Scenario Outline: This is a scenario with the same indentation as the Feature
    Examples:
"#;

        let expected = r#"  Scenario Outline: This is a scenario with the same indentation as the Feature

    Examples:
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn removes_extra_lines_before_examples_block() {
        let input = r#"  Scenario Outline: This is a scenario with the same indentation as the Feature




    Examples:
"#;

        let expected = r#"  Scenario Outline: This is a scenario with the same indentation as the Feature

    Examples:
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}