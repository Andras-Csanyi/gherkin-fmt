use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    capitalize_scenario_outline_keyword, is_scenario_outline_line, join_lines, leading_spaces_count,
    split_lines,
};

pub const RULE_NAME: &str = "the `Scenario Outline` keyword is capitalized";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);

    for line in &mut lines {
        if !is_scenario_outline_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];
        let formatted = capitalize_scenario_outline_keyword(content);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalizes_lowercase_scenario_in_keyword() {
        let input = r#"scenario Outline: this is a test scenario outline
"#;

        let expected = r#"Scenario Outline: this is a test scenario outline
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_outline_in_keyword() {
        let input = r#"Scenario outline: this is a test scenario outline
"#;

        let expected = r#"Scenario Outline: this is a test scenario outline
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_capitalized_scenario_outline_keyword() {
        let input = r#"Scenario Outline: this is a test scenario outline
"#;

        let expected = r#"Scenario Outline: this is a test scenario outline
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}