use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    find_table_blocks, join_lines, leading_spaces_count, parse_table_cells, split_lines,
};

pub const RULE_NAME: &str = "Vertical table is indented by 1 level in steps";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let (mut lines, ends_with_newline) = split_lines(input);
    let table_blocks = find_table_blocks(&lines);

    for block in table_blocks {
        let step_indent = leading_spaces_count(&lines[block.parent_step_index]);
        let table_indent = " ".repeat(step_indent + config.indent_size);

        for line_index in block.start..block.end {
            let cells = parse_table_cells(&lines[line_index])?;
            lines[line_index] = super::helpers::format_table_row(&table_indent, &cells);
        }
    }

    Ok(join_lines(&lines, ends_with_newline))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indents_single_line_table_under_step() {
        let input = r#"Given something is given
| header 1 | header 2 | header 3 |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn indents_multiline_table_under_step() {
        let input = r#"Given something is given
| header 1 | header 2 | header 3 |
| what 1   | what 2   | what 3   |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
  | what 1   | what 2   | what 3   |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn keeps_already_aligned_table() {
        let input = r#"Given something is given
  | header 1 | header 2 | header 3 |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn keeps_already_aligned_multiline_table() {
        let input = r#"Given something is given
  | header 1 | header 2 | header 3 |
  | value 1  | value 2  | value 3  |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
  | value 1  | value 2  | value 3  |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_overindented_single_line_table() {
        let input = r#"Given something is given
            | header 1 | header 2 | header 3 |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_overindented_multiline_table() {
        let input = r#"Given something is given
            | header 1 | header 2 | header 3 |
            | value 1  | value 2  | value 3  |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
  | value 1  | value 2  | value 3  |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn aligns_mixed_indentation_table() {
        let input = r#"Given something is given
            | header 1 | header 2 | header 3 |
                  | value 1  | value 2  | value 3  |
            |v 1       | v 2      | v 3      |
      |a 1       | a 2      | a 3      |
"#;

        let expected = r#"Given something is given
  | header 1 | header 2 | header 3 |
  | value 1  | value 2  | value 3  |
  |v 1       | v 2      | v 3      |
  |a 1       | a 2      | a 3      |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}