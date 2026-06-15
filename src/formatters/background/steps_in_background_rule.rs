use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    background_block_end, find_background_line_indices, is_step_line, join_lines,
    leading_spaces_count, split_lines,
};

pub const RULE_NAME: &str = "Background block steps alignment";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let background_indices = find_background_line_indices(&lines);

    for background_index in background_indices {
        let background_indent = leading_spaces_count(&lines[background_index]);
        let step_indent = " ".repeat(background_indent + config.indent_size);
        let block_end = background_block_end(&lines, background_index);

        for line_index in (background_index + 1)..block_end {
            if is_step_line(&lines[line_index]) {
                let trimmed = lines[line_index].trim_start();
                lines[line_index] = format!("{step_indent}{trimmed}");
            }
        }
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indents_single_step_in_background_block() {
        let input = r#"Background: this is a test background
Given this is a not indented step
"#;

        let expected = r#"Background: this is a test background
  Given this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn indents_two_steps_in_background_block() {
        let input = r#"Background: this is a test background
Given this is a not indented step
And this is a not indented step
"#;

        let expected = r#"Background: this is a test background
  Given this is a not indented step
  And this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn indents_three_steps_in_background_block() {
        let input = r#"Background: this is a test background
Given this is a not indented step
When this is a not indented step
Then this is a not indented step
"#;

        let expected = r#"Background: this is a test background
  Given this is a not indented step
  When this is a not indented step
  Then this is a not indented step
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_chaotically_indented_steps_in_background_block() {
        let input = r#"Background: this is a test background
    Given this is a not indented step
          And here is some chaos
    When this is a not indented step
        And you want more
Then this is a not indented step
   And here is more
"#;

        let expected = r#"Background: this is a test background
  Given this is a not indented step
  And here is some chaos
  When this is a not indented step
  And you want more
  Then this is a not indented step
  And here is more
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}