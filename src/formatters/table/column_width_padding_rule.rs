use anyhow::Result;

use crate::formatters::Config;

use super::helpers::{
    find_table_blocks, format_table_row, join_lines, leading_spaces_count, parse_table_cells,
    split_lines,
};

pub const RULE_NAME: &str = "column width padding";

pub fn apply(input: &str, config: &Config) -> Result<String> {
    let _ = config;
    let (mut lines, ends_with_newline) = split_lines(input);
    let table_blocks = find_table_blocks(&lines);

    for block in table_blocks {
        let column_widths = calculate_column_widths(&lines[block.start..block.end])?;

        for line_index in block.start..block.end {
            let indent = " ".repeat(leading_spaces_count(&lines[line_index]));
            let cells = parse_table_cells(&lines[line_index])?
                .into_iter()
                .enumerate()
                .map(|(column_index, cell)| format_cell(&cell, column_widths[column_index]))
                .collect::<Vec<_>>();
            lines[line_index] = format_table_row(&indent, &cells);
        }
    }

    Ok(join_lines(&lines, ends_with_newline))
}

fn calculate_column_widths(table_lines: &[String]) -> Result<Vec<usize>> {
    let mut column_widths = Vec::new();

    for line in table_lines {
        let cells = parse_table_cells(line)?;
        ensure_column_count(&mut column_widths, cells.len())?;

        for (column_index, cell) in cells.iter().enumerate() {
            let padded_width = normalized_cell(cell).len();
            column_widths[column_index] = column_widths[column_index].max(padded_width);
        }
    }

    Ok(column_widths)
}

fn ensure_column_count(column_widths: &mut Vec<usize>, column_count: usize) -> Result<()> {
    if column_widths.is_empty() {
        column_widths.resize(column_count, 0);
        return Ok(());
    }

    if column_widths.len() != column_count {
        anyhow::bail!("table rows must have the same number of columns");
    }

    Ok(())
}

fn format_cell(cell: &str, column_width: usize) -> String {
    let normalized = normalized_cell(cell);
    let padding = column_width.saturating_sub(normalized.len());
    format!("{normalized}{}", " ".repeat(padding))
}

fn normalized_cell(cell: &str) -> String {
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        return "  ".to_string();
    }

    format!(" {trimmed} ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pads_two_column_table() {
        let input = r#"Given this has a table
  | header 1| header 2|
  |value and value 1| values and values and values|
"#;

        let expected = r#"Given this has a table
  | header 1          | header 2                     |
  | value and value 1 | values and values and values |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn pads_three_column_table_with_two_rows() {
        let input = r#"Given this has a table
  | h 1| h 2| h 3|
  |value 1| values and| more values|
"#;

        let expected = r#"Given this has a table
  | h 1     | h 2        | h 3         |
  | value 1 | values and | more values |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }

    #[test]
    fn pads_three_column_table_with_multiple_rows() {
        let input = r#"Given this step has a table
  | h 1| h 2| h 3|
  |value 1| values and| more values|
  |a value| a value ssssssssss| a value|
  |bcddddddd value| bc value| bc value|
  |dcccc value| dcccc value| dcccc value|
  |gggg value| gggg value| ggggdddddddddddd value|
"#;

        let expected = r#"Given this step has a table
  | h 1             | h 2                | h 3                    |
  | value 1         | values and         | more values            |
  | a value         | a value ssssssssss | a value                |
  | bcddddddd value | bc value           | bc value               |
  | dcccc value     | dcccc value        | dcccc value            |
  | gggg value      | gggg value         | ggggdddddddddddd value |
"#;

        assert_eq!(apply(input, &Config::default()).unwrap(), expected);
    }
}

