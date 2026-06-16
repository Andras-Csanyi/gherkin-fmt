#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Delimiter {
    Backtick,
    Quote,
}

pub(crate) struct RawBlock {
    pub content_lines: Vec<String>,
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

pub(crate) fn find_raw_blocks(lines: &[String]) -> Vec<RawBlock> {
    let mut blocks = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let Some(delimiter) = opening_delimiter(&lines[index]) else {
            index += 1;
            continue;
        };

        let opening_index = index;
        index += 1;

        while index < lines.len() {
            if closing_delimiter(&lines[index]) == Some(delimiter) {
                if opening_index + 1 < index {
                    blocks.push(RawBlock {
                        content_lines: lines[(opening_index + 1)..index].to_vec(),
                    });
                }
                index += 1;
                break;
            }

            index += 1;
        }
    }

    blocks
}

pub(crate) fn find_raw_block_delimiter_pairs(lines: &[String]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let Some(delimiter) = opening_delimiter(&lines[index]) else {
            index += 1;
            continue;
        };

        let opening_index = index;
        index += 1;

        while index < lines.len() {
            if closing_delimiter(&lines[index]) == Some(delimiter) {
                if opening_index + 1 < index {
                    pairs.push((opening_index, index));
                }
                index += 1;
                break;
            }

            index += 1;
        }
    }

    pairs
}

fn opening_delimiter(line: &str) -> Option<Delimiter> {
    let trimmed = line.trim_start();

    if trimmed.starts_with("```") {
        return Some(Delimiter::Backtick);
    }

    if trimmed.starts_with("\"\"\"") {
        return Some(Delimiter::Quote);
    }

    None
}

fn closing_delimiter(line: &str) -> Option<Delimiter> {
    let trimmed = line.trim_start();

    if trimmed.starts_with("```") {
        return Some(Delimiter::Backtick);
    }

    if trimmed.starts_with("\"\"\"") {
        return Some(Delimiter::Quote);
    }

    None
}