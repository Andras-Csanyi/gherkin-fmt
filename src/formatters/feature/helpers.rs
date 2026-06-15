use anyhow::{Context, Result};

const SCENARIO_KEYWORDS: &[&str] = &[
    "Scenario Outline:",
    "Scenario:",
    "Background:",
    "Rule:",
    "Examples:",
    "Example:",
];

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

pub(crate) fn is_feature_line(line: &str) -> bool {
    starts_with_feature_keyword(line.trim_start())
}

pub(crate) fn capitalize_feature_keyword(content: &str) -> String {
    const KEYWORD: &str = "Feature:";

    if starts_with_feature_keyword(content) {
        return format!("{}{}", KEYWORD, &content[KEYWORD.len()..]);
    }

    content.to_string()
}

fn starts_with_feature_keyword(line: &str) -> bool {
    const KEYWORD: &str = "Feature:";

    if line.len() < KEYWORD.len() {
        return false;
    }

    if !line[..KEYWORD.len()].eq_ignore_ascii_case(KEYWORD) {
        return false;
    }

    line.len() == KEYWORD.len() || line.as_bytes()[KEYWORD.len()] == b' '
}

pub(crate) fn find_feature_line_index(lines: &[String]) -> Result<usize> {
    lines
        .iter()
        .position(|line| is_feature_line(line))
        .context("no Feature: line found in content")
}

pub(crate) fn is_block_boundary(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('@') {
        return true;
    }

    SCENARIO_KEYWORDS
        .iter()
        .any(|keyword| trimmed.starts_with(keyword))
}

pub(crate) fn feature_block_end(lines: &[String], feature_index: usize) -> usize {
    for index in (feature_index + 1)..lines.len() {
        if is_block_boundary(&lines[index]) {
            return index;
        }
    }

    lines.len()
}