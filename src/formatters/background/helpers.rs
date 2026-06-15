const STEP_KEYWORDS: &[&str] = &["Given", "When", "Then", "And", "But", "*"];

const BACKGROUND_BLOCK_BOUNDARIES: &[&str] = &[
    "Background:",
    "Scenario Outline:",
    "Scenario:",
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

pub(crate) fn leading_spaces_count(line: &str) -> usize {
    line.chars()
        .take_while(|character| *character == ' ')
        .count()
}

pub(crate) fn is_background_line(line: &str) -> bool {
    starts_with_background_keyword(line.trim_start())
}

pub(crate) fn capitalize_background_keyword(content: &str) -> String {
    const KEYWORD: &str = "Background:";

    if starts_with_background_keyword(content) {
        return format!("{}{}", KEYWORD, &content[KEYWORD.len()..]);
    }

    content.to_string()
}

fn starts_with_background_keyword(line: &str) -> bool {
    const KEYWORD: &str = "Background:";

    if line.len() < KEYWORD.len() {
        return false;
    }

    if !line[..KEYWORD.len()].eq_ignore_ascii_case(KEYWORD) {
        return false;
    }

    line.len() == KEYWORD.len() || line.as_bytes()[KEYWORD.len()] == b' '
}

pub(crate) fn find_background_line_indices(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| is_background_line(line).then_some(index))
        .collect()
}

pub(crate) fn background_block_end(lines: &[String], background_index: usize) -> usize {
    for index in (background_index + 1)..lines.len() {
        if is_background_block_boundary(&lines[index]) {
            return index;
        }
    }

    lines.len()
}

pub(crate) fn is_step_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    STEP_KEYWORDS
        .iter()
        .any(|keyword| step_starts_with(trimmed, keyword))
        || ["given", "when", "then", "and", "but"]
            .iter()
            .any(|keyword| step_starts_with_ignore_case(trimmed, keyword))
}

fn is_background_block_boundary(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('@') {
        return true;
    }

    if starts_with_background_keyword(trimmed) {
        return true;
    }

    BACKGROUND_BLOCK_BOUNDARIES
        .iter()
        .filter(|keyword| **keyword != "Background:")
        .any(|keyword| trimmed.starts_with(keyword))
}

fn step_starts_with(line: &str, keyword: &str) -> bool {
    if !line.starts_with(keyword) {
        return false;
    }

    line.len() == keyword.len() || line.as_bytes()[keyword.len()] == b' '
}

fn step_starts_with_ignore_case(line: &str, keyword: &str) -> bool {
    if line.len() < keyword.len() {
        return false;
    }

    if !line[..keyword.len()].eq_ignore_ascii_case(keyword) {
        return false;
    }

    line.len() == keyword.len() || line.as_bytes()[keyword.len()] == b' '
}