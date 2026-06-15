const STEP_KEYWORDS: &[&str] = &["Given", "When", "Then", "And", "But", "*"];

const SCENARIO_OUTLINE_BLOCK_BOUNDARIES: &[&str] = &[
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

pub(crate) fn leading_spaces_count(line: &str) -> usize {
    line.chars()
        .take_while(|character| *character == ' ')
        .count()
}

pub(crate) fn is_scenario_outline_line(line: &str) -> bool {
    starts_with_scenario_outline_keyword(line.trim_start())
}

pub(crate) fn capitalize_scenario_outline_keyword(content: &str) -> String {
    if !starts_with_scenario_outline_keyword(content) {
        return content.to_string();
    }

    let colon_index = content
        .find(':')
        .expect("scenario outline keyword must contain ':'");
    format!("Scenario Outline:{}", &content[colon_index + 1..])
}

pub(crate) fn find_scenario_outline_line_indices(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| is_scenario_outline_line(line).then_some(index))
        .collect()
}

pub(crate) fn scenario_outline_block_end(lines: &[String], scenario_outline_index: usize) -> usize {
    for index in (scenario_outline_index + 1)..lines.len() {
        if is_scenario_outline_block_boundary(&lines[index]) {
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

fn starts_with_scenario_outline_keyword(line: &str) -> bool {
    let colon_index = match line.find(':') {
        Some(index) => index,
        None => return false,
    };

    let before_colon = &line[..colon_index];
    let mut parts = before_colon.split(' ');
    match (parts.next(), parts.next()) {
        (Some(first), Some(second)) if parts.next().is_none() => {
            first.eq_ignore_ascii_case("Scenario") && second.eq_ignore_ascii_case("Outline")
        }
        _ => false,
    }
}

fn is_scenario_outline_block_boundary(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('@') {
        return true;
    }

    if starts_with_scenario_outline_keyword(trimmed) {
        return true;
    }

    SCENARIO_OUTLINE_BLOCK_BOUNDARIES
        .iter()
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