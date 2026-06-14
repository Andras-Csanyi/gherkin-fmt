use std::ops::Range;

const STEP_KEYWORDS: &[&str] = &["Given", "When", "Then", "And", "But", "*"];

#[derive(Copy, Clone)]
pub(crate) struct SpacingOptions {
    pub protect_quotes: bool,
    pub protect_templates: bool,
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
    line.chars()
        .take_while(|character| *character == ' ')
        .count()
}

pub(crate) fn is_step_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    STEP_KEYWORDS
        .iter()
        .any(|keyword| step_starts_with(trimmed, keyword))
        || step_starts_with_lowercase_keyword(trimmed)
}

pub(crate) fn capitalize_step_keyword(content: &str) -> String {
    let (keyword, remainder) = match split_step_keyword(content) {
        Some(parts) => parts,
        None => return content.to_string(),
    };

    format!("{keyword}{remainder}")
}

pub(crate) fn normalize_template_parameters(content: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = content.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] == '<' {
            if let Some(end) = find_closing_angle_bracket(&chars, index) {
                let inner: String = chars[index + 1..end].iter().collect();
                result.push('<');
                result.push_str(inner.trim());
                result.push('>');
                index = end + 1;
                continue;
            }
        }

        result.push(chars[index]);
        index += 1;
    }

    result
}

pub(crate) fn normalize_step_spacing(content: &str, options: SpacingOptions) -> String {
    let (body, trailing_whitespace) = split_trailing_whitespace(content);
    let protected_ranges = collect_protected_ranges(&body, &options);
    let normalized_body = collapse_spaces_outside_ranges(&body, &protected_ranges);
    format!("{normalized_body}{trailing_whitespace}")
}

pub(crate) fn format_step_line(content: &str, spacing_options: &SpacingOptions) -> String {
    normalize_step_spacing(content, *spacing_options)
}

fn split_step_keyword(content: &str) -> Option<(&'static str, &str)> {
    for keyword in STEP_KEYWORDS {
        if step_starts_with(content, keyword) {
            return Some((keyword, &content[keyword.len()..]));
        }
    }

    for keyword in ["given", "when", "then", "and", "but"] {
        if step_starts_with_ignore_case(content, keyword) {
            let proper = match keyword {
                "given" => "Given",
                "when" => "When",
                "then" => "Then",
                "and" => "And",
                "but" => "But",
                _ => unreachable!(),
            };
            return Some((proper, &content[keyword.len()..]));
        }
    }

    None
}

fn step_starts_with_lowercase_keyword(line: &str) -> bool {
    ["given", "when", "then", "and", "but"]
        .iter()
        .any(|keyword| step_starts_with_ignore_case(line, keyword))
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

fn split_trailing_whitespace(content: &str) -> (&str, &str) {
    let trimmed_end = content.trim_end_matches(' ');
    (trimmed_end, &content[trimmed_end.len()..])
}

fn collect_protected_ranges(body: &str, options: &SpacingOptions) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    if options.protect_quotes {
        ranges.extend(find_quote_ranges(body));
    }

    if options.protect_templates {
        ranges.extend(find_template_ranges(body));
    }

    ranges.sort_by_key(|range| range.start);
    ranges
}

fn find_quote_ranges(text: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let bytes = text.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'"' {
            let start = index;
            index += 1;

            while index < bytes.len() && bytes[index] != b'"' {
                index += 1;
            }

            if index < bytes.len() {
                ranges.push(start..index + 1);
                index += 1;
            }
        } else {
            index += 1;
        }
    }

    ranges
}

fn find_template_ranges(text: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] == '<' {
            if let Some(end) = find_closing_angle_bracket(&chars, index) {
                ranges.push(index..end + 1);
                index = end + 1;
                continue;
            }
        }

        index += 1;
    }

    ranges
}

fn find_closing_angle_bracket(chars: &[char], start: usize) -> Option<usize> {
    for index in (start + 1)..chars.len() {
        if chars[index] == '>' {
            return Some(index);
        }
    }

    None
}

fn collapse_spaces_outside_ranges(body: &str, protected_ranges: &[Range<usize>]) -> String {
    let mut result = String::new();
    let mut last_was_space = false;

    for (index, character) in body.chars().enumerate() {
        if is_index_protected(index, protected_ranges) {
            result.push(character);
            last_was_space = character == ' ';
            continue;
        }

        if character == ' ' {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
            continue;
        }

        result.push(character);
        last_was_space = false;
    }

    result
}

fn is_index_protected(index: usize, protected_ranges: &[Range<usize>]) -> bool {
    protected_ranges
        .iter()
        .any(|range| range.contains(&index))
}