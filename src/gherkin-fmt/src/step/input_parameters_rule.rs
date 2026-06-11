use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    format_step_line, is_step_line, join_lines, leading_spaces_count, split_lines, SpacingOptions,
};

pub const RULE_NAME: &str =
    "In Step definitions the content between `\"` and `\"` remains as it is";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let spacing_options = SpacingOptions {
        protect_quotes: true,
        protect_templates: true,
    };

    for line in &mut lines {
        if !is_step_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];
        let formatted = format_step_line(content, &spacing_options);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_single_input_parameter_in_given_step() {
        let input = r#"Given we have this "input parameter"
"#;

        let expected = r#"Given we have this "input parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_outside_quotes_in_given_step() {
        let input = r#"Given we        have           this "input parameter"
"#;

        let expected = r#"Given we have this "input parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_funky_spaces_inside_quotes_in_given_step() {
        let input = r#"Given we        have           this "input              parameter"
"#;

        let expected = r#"Given we have this "input              parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_outside_quotes_in_when_step() {
        let input = r#"When we        have           this "input              parameter"
"#;

        let expected = r#"When we have this "input              parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_outside_quotes_in_then_step() {
        let input = r#"Then we        have           this "input              parameter"
"#;

        let expected = r#"Then we have this "input              parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_outside_quotes_in_and_step() {
        let input = r#"And we        have           this "input              parameter"
"#;

        let expected = r#"And we have this "input              parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_outside_quotes_in_but_step() {
        let input = r#"But we        have           this "input              parameter"
"#;

        let expected = r#"But we have this "input              parameter"
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}