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

pub(crate) fn is_scenario_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("Scenario Outline:") || trimmed.starts_with("Scenario:")
}

pub(crate) fn find_scenario_line_indices(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| is_scenario_line(line).then_some(index))
        .collect()
}

pub(crate) fn scenario_block_end(lines: &[String], scenario_index: usize) -> usize {
    for index in (scenario_index + 1)..lines.len() {
        if is_scenario_line(&lines[index]) {
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

