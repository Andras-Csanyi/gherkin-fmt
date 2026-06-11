use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    format_step_line, is_step_line, join_lines, leading_spaces_count, split_lines, SpacingOptions,
};

pub const RULE_NAME: &str = "one space between words in steps";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let spacing_options = SpacingOptions {
        protect_quotes: false,
        protect_templates: false,
    };

    for line in &mut lines {
        if !is_step_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];

        if content.contains('"') || content.contains('<') {
            continue;
        }

        let formatted = format_step_line(content, &spacing_options);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_simple_given_step() {
        let input = r#"Given everything is just fine with this step
"#;

        let expected = r#"Given everything is just fine with this step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_multiple_spaces_in_given_step() {
        let input = r#"Given this step           is not fine
"#;

        let expected = r#"Given this step is not fine
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_multiple_spaces_in_when_step() {
        let input = r#"When this          step           is not           fine
"#;

        let expected = r#"When this step is not fine
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_multiple_spaces_in_then_step() {
        let input = r#"Then this          step           is not              fine
"#;

        let expected = r#"Then this step is not fine
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_multiple_spaces_in_and_step() {
        let input = r#"And this             step           is not            fine
"#;

        let expected = r#"And this step is not fine
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_multiple_spaces_in_but_step() {
        let input = r#"But this                step           is not              fine
"#;

        let expected = r#"But this step is not fine
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn collapses_spaces_across_multiple_steps_and_preserves_trailing_space() {
        let input = r#"Given this is a proper given statement
And this        is       not really
And this                step           is not              fine either
But this one is fine again
When I try to read               these
And I       really       try 
Then I wish I had a good formatter
And I would read these lines easily
But not        these        ones
"#;

        let expected = r#"Given this is a proper given statement
And this is not really
And this step is not fine either
But this one is fine again
When I try to read these
And I really try 
Then I wish I had a good formatter
And I would read these lines easily
But not these ones
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}