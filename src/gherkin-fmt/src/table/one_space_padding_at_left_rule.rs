use anyhow::Result;

use crate::config::Config;

use super::helpers::{
    find_table_blocks, format_table_row, join_lines, leading_spaces_count, parse_table_cells,
    split_lines,
};

pub const RULE_NAME: &str = "One space left padding in the cell";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let table_blocks = find_table_blocks(&lines);

    for block in table_blocks {
        for line_index in block.start..block.end {
            let indent = " ".repeat(leading_spaces_count(&lines[line_index]));
            let cells = parse_table_cells(&lines[line_index])?
                .into_iter()
                .map(add_left_padding)
                .collect::<Vec<_>>();
            lines[line_index] = format_table_row(&indent, &cells);
        }
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn add_left_padding(cell: String) -> String {
    let trimmed = cell.trim_start();
    if trimmed.is_empty() {
        return " ".to_string();
    }

    format!(" {trimmed}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_left_padding_to_three_column_table() {
        let input = r#"Given this step includes a vertical table
|header 1|header 2|header 3|
"#;

        let expected = r#"Given this step includes a vertical table
| header 1| header 2| header 3|
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn adds_left_padding_to_two_line_table() {
        let input = r#"Given this step includes a vertical table
|header 1|header 2|header 3|
|value 1|value 2|value 3|
"#;

        let expected = r#"Given this step includes a vertical table
| header 1| header 2| header 3|
| value 1| value 2| value 3|
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn adds_left_padding_to_four_column_table() {
        let input = r#"Given this step includes a vertical table
|header 1|header 2|header 3|header 4|
|value 1|value 2|value 3|value 4|
"#;

        let expected = r#"Given this step includes a vertical table
| header 1| header 2| header 3| header 4|
| value 1| value 2| value 3| value 4|
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}