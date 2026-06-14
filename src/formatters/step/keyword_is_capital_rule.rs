use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    capitalize_step_keyword, is_step_line, join_lines, leading_spaces_count, split_lines,
};

pub const RULE_NAME: &str = "Step definition keywords start with capital letters";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);

    for line in &mut lines {
        if !is_step_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];
        let formatted = capitalize_step_keyword(content);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_correct_given_keyword() {
        let input = r#"Given this is a correct one
"#;

        let expected = r#"Given this is a correct one
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_given_keyword() {
        let input = r#"given this step has another given
"#;

        let expected = r#"Given this step has another given
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_given_in_middle_of_step() {
        let input = r#"Given this step has another given
"#;

        let expected = r#"Given this step has another given
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_when_keyword() {
        let input = r#"when this is a correct one
"#;

        let expected = r#"When this is a correct one
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_when_in_middle_of_step() {
        let input = r#"When and here is another when 
"#;

        let expected = r#"When and here is another when 
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_then_keyword() {
        let input = r#"then this step has another then
"#;

        let expected = r#"Then this step has another then
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_and_keyword() {
        let input = r#"and this step has another and
"#;

        let expected = r#"And this step has another and
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn capitalizes_lowercase_but_keyword() {
        let input = r#"but this step has another but
"#;

        let expected = r#"But this step has another but
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}

