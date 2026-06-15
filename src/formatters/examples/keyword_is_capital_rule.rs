use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    capitalize_examples_keyword, is_examples_line, join_lines, leading_spaces_count, split_lines,
};

pub const RULE_NAME: &str = "the `Examples` keyword is capitalized";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);

    for line in &mut lines {
        if !is_examples_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];
        let formatted = capitalize_examples_keyword(content);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalizes_lowercase_examples_keyword() {
        let input = r#"examples: this is a test examples
"#;

        let expected = r#"Examples: this is a test examples
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn preserves_capitalized_examples_keyword() {
        let input = r#"Examples: this is a test examples
"#;

        let expected = r#"Examples: this is a test examples
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}