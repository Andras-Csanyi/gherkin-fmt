const STEP_KEYWORDS: &[&str] = &["Given", "When", "Then", "And", "But", "*"];

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

pub(crate) fn is_examples_line(line: &str) -> bool {
    starts_with_examples_keyword(line.trim_start())
}

pub(crate) fn capitalize_examples_keyword(content: &str) -> String {
    const KEYWORD: &str = "Examples:";

    if starts_with_examples_keyword(content) {
        return format!("{}{}", KEYWORD, &content[KEYWORD.len()..]);
    }

    content.to_string()
}

pub(crate) fn find_examples_line_indices(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| is_examples_line(line).then_some(index))
        .collect()
}

pub(crate) fn find_parent_scenario_outline_index(
    lines: &[String],
    examples_index: usize,
) -> Option<usize> {
    for index in (0..examples_index).rev() {
        let line = &lines[index];
        if line.trim().is_empty() || is_examples_line(line) {
            continue;
        }

        if is_scenario_outline_line(line) {
            return Some(index);
        }

        if is_step_line(line) {
            continue;
        }

        return None;
    }

    None
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

fn starts_with_examples_keyword(line: &str) -> bool {
    const KEYWORD: &str = "Examples:";

    if line.len() < KEYWORD.len() {
        return false;
    }

    if !line[..KEYWORD.len()].eq_ignore_ascii_case(KEYWORD) {
        return false;
    }

    line.len() == KEYWORD.len() || line.as_bytes()[KEYWORD.len()] == b' '
}

fn is_scenario_outline_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    let colon_index = match trimmed.find(':') {
        Some(index) => index,
        None => return false,
    };

    let before_colon = &trimmed[..colon_index];
    let mut parts = before_colon.split(' ');
    match (parts.next(), parts.next()) {
        (Some(first), Some(second)) if parts.next().is_none() => {
            first.eq_ignore_ascii_case("Scenario") && second.eq_ignore_ascii_case("Outline")
        }
        _ => false,
    }
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