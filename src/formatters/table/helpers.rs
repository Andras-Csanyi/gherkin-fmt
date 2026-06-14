use anyhow::{Context, Result};

const STEP_KEYWORDS: &[&str] = &["Given", "When", "Then", "And", "But", "*"];

pub(crate) struct TableBlock {
    pub start: usize,
    pub end: usize,
    pub parent_step_index: usize,
}

pub(crate) fn split_lines(input: &str) -> (Vec<String>, bool) {
    let ends_with_newline = input.ends_with('\n');
    let lines = input.lines().map(String::from).collect();
    (lines, ends_with_newline)
}

pub(crate) fn join_lines(lines: &[String], ends_with_newline: bool) -> String {
    let mut result = lines.join("\n");
    if ends_with_newline {
        result.push('\n');
    }
    result
}

pub(crate) fn leading_spaces_count(line: &str) -> usize {
    line.chars().take_while(|character| *character == ' ').count()
}

pub(crate) fn is_table_row(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

pub(crate) fn is_step_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    STEP_KEYWORDS
        .iter()
        .any(|keyword| step_starts_with(trimmed, keyword))
}

pub(crate) fn find_table_blocks(lines: &[String]) -> Vec<TableBlock> {
    let mut blocks = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        if !is_table_row(&lines[index]) {
            index += 1;
            continue;
        }

        let start = index;
        while index < lines.len() && is_table_row(&lines[index]) {
            index += 1;
        }

        if let Some(parent_step_index) = find_parent_step_index(lines, start) {
            blocks.push(TableBlock {
                start,
                end: index,
                parent_step_index,
            });
        }
    }

    blocks
}

pub(crate) fn parse_table_cells(line: &str) -> Result<Vec<String>> {
    let trimmed = line.trim_start();
    let without_leading_pipe = trimmed
        .strip_prefix('|')
        .context("table row must start with '|'")?;

    let mut cells = Vec::new();
    for segment in without_leading_pipe.split('|') {
        cells.push(segment.to_string());
    }

    if cells.last().is_some_and(String::is_empty) {
        cells.pop();
    }

    Ok(cells)
}

pub(crate) fn format_table_row(indent: &str, cells: &[String]) -> String {
    let mut row = indent.to_string();

    for cell in cells {
        row.push('|');
        row.push_str(cell);
    }

    row.push('|');
    row
}

fn find_parent_step_index(lines: &[String], table_start: usize) -> Option<usize> {
    for index in (0..table_start).rev() {
        let line = &lines[index];
        if line.trim().is_empty() {
            continue;
        }

        if is_step_line(line) {
            return Some(index);
        }

        return None;
    }

    None
}

fn step_starts_with(line: &str, keyword: &str) -> bool {
    if !line.starts_with(keyword) {
        return false;
    }

    line.len() == keyword.len() || line.as_bytes()[keyword.len()] == b' '
}