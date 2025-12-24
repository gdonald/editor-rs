#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLineType {
    Context,
    Addition,
    Deletion,
    Header,
    FileHeader,
    Hunk,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub old_line_num: Option<usize>,
    pub new_line_num: Option<usize>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SideBySideDiff {
    pub left_lines: Vec<DiffLine>,
    pub right_lines: Vec<DiffLine>,
}

impl SideBySideDiff {
    pub fn from_unified_diff(unified_diff: &str) -> Self {
        let mut left_lines = Vec::new();
        let mut right_lines = Vec::new();

        let mut old_line_num = 0;
        let mut new_line_num = 0;

        for line in unified_diff.lines() {
            if line.starts_with("+++") || line.starts_with("---") {
                let file_header = DiffLine {
                    line_type: DiffLineType::FileHeader,
                    old_line_num: None,
                    new_line_num: None,
                    content: line.to_string(),
                };
                left_lines.push(file_header.clone());
                right_lines.push(file_header);
                continue;
            }

            if line.starts_with("@@") {
                if let Some((old_start, new_start)) = parse_hunk_header(line) {
                    old_line_num = old_start;
                    new_line_num = new_start;
                }

                let hunk_header = DiffLine {
                    line_type: DiffLineType::Hunk,
                    old_line_num: None,
                    new_line_num: None,
                    content: line.to_string(),
                };
                left_lines.push(hunk_header.clone());
                right_lines.push(hunk_header);
                continue;
            }

            if line.starts_with("diff --git") || line.starts_with("index ") {
                let header = DiffLine {
                    line_type: DiffLineType::Header,
                    old_line_num: None,
                    new_line_num: None,
                    content: line.to_string(),
                };
                left_lines.push(header.clone());
                right_lines.push(header);
                continue;
            }

            if let Some(stripped) = line.strip_prefix('+') {
                let content = stripped.to_string();

                left_lines.push(DiffLine {
                    line_type: DiffLineType::Addition,
                    old_line_num: None,
                    new_line_num: None,
                    content: String::new(),
                });

                right_lines.push(DiffLine {
                    line_type: DiffLineType::Addition,
                    old_line_num: None,
                    new_line_num: Some(new_line_num),
                    content,
                });

                new_line_num += 1;
            } else if let Some(stripped) = line.strip_prefix('-') {
                let content = stripped.to_string();

                left_lines.push(DiffLine {
                    line_type: DiffLineType::Deletion,
                    old_line_num: Some(old_line_num),
                    new_line_num: None,
                    content,
                });

                right_lines.push(DiffLine {
                    line_type: DiffLineType::Deletion,
                    old_line_num: None,
                    new_line_num: None,
                    content: String::new(),
                });

                old_line_num += 1;
            } else if line.starts_with(' ') || line.is_empty() {
                let content = if line.starts_with(' ') && line.len() > 1 {
                    line[1..].to_string()
                } else if line.is_empty() {
                    String::new()
                } else {
                    line.to_string()
                };

                let context_line = DiffLine {
                    line_type: DiffLineType::Context,
                    old_line_num: Some(old_line_num),
                    new_line_num: Some(new_line_num),
                    content: content.clone(),
                };

                left_lines.push(context_line.clone());
                right_lines.push(context_line);

                old_line_num += 1;
                new_line_num += 1;
            } else {
                let header = DiffLine {
                    line_type: DiffLineType::Header,
                    old_line_num: None,
                    new_line_num: None,
                    content: line.to_string(),
                };
                left_lines.push(header.clone());
                right_lines.push(header);
            }
        }

        Self {
            left_lines,
            right_lines,
        }
    }

    pub fn max_lines(&self) -> usize {
        self.left_lines.len().max(self.right_lines.len())
    }
}

fn parse_hunk_header(header: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let old_part = parts[1].trim_start_matches('-');
    let new_part = parts[2].trim_start_matches('+');

    let old_start = old_part
        .split(',')
        .next()?
        .parse::<usize>()
        .ok()?
        .saturating_sub(1);
    let new_start = new_part
        .split(',')
        .next()?
        .parse::<usize>()
        .ok()?
        .saturating_sub(1);

    Some((old_start, new_start))
}
