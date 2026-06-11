use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    is_step_line, join_lines, leading_spaces_count, normalize_template_parameters, split_lines,
};

pub const RULE_NAME: &str = "Template parameters in Step definitionsl";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);

    for line in &mut lines {
        if !is_step_line(line) {
            continue;
        }

        let indent = leading_spaces_count(line);
        let content = &line[indent..];
        let formatted = normalize_template_parameters(content);
        *line = format!("{}{formatted}", " ".repeat(indent));
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_correct_template_parameter_in_given_step() {
        let input = r#"Given this has the <parameter> parameter
"#;

        let expected = r#"Given this has the <parameter> parameter
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn trims_left_spacing_in_template_parameter() {
        let input = r#"Given this has the < parameter> parameter
"#;

        let expected = r#"Given this has the <parameter> parameter
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn trims_right_spacing_in_template_parameter() {
        let input = r#"Given this has the <parameter > parameter
"#;

        let expected = r#"Given this has the <parameter> parameter
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn trims_chaotic_spacing_in_template_parameter() {
        let input = r#"Given this has the <       parameter       > parameter
"#;

        let expected = r#"Given this has the <parameter> parameter
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn normalizes_multiple_template_parameters_in_given_step() {
        let input = r#"Given this has the <       parameter       > parameter and another <      one     >
"#;

        let expected = r#"Given this has the <parameter> parameter and another <one>
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn normalizes_template_parameters_in_when_step() {
        let input = r#"When this has the < parameter> parameter and another <one >
"#;

        let expected = r#"When this has the <parameter> parameter and another <one>
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn normalizes_template_parameters_in_then_step() {
        let input = r#"Then this has the <parameter > parameter and another < one>
"#;

        let expected = r#"Then this has the <parameter> parameter and another <one>
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}