use super::types::CommitInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitGraphNode {
    pub commit: CommitInfo,
    pub graph_line: String,
    pub level: usize,
}

pub fn generate_commit_graph(commits: &[CommitInfo]) -> Vec<CommitGraphNode> {
    commits
        .iter()
        .map(|commit| CommitGraphNode {
            commit: commit.clone(),
            graph_line: "* ".to_string(),
            level: 0,
        })
        .collect()
}

pub fn format_graph_line(node: &CommitGraphNode, show_annotation: bool) -> Vec<String> {
    let mut lines = Vec::new();

    let short_id = if node.commit.id.len() >= 7 {
        &node.commit.id[..7]
    } else {
        &node.commit.id
    };

    let first_line = node.commit.message.lines().next().unwrap_or("");
    let main_line = format!("{}{} {}", node.graph_line, short_id, first_line);
    lines.push(main_line);

    if show_annotation {
        if let Some(annotation) = &node.commit.annotation {
            let indent = " ".repeat(node.graph_line.len() + 2);
            lines.push(format!("{}📝 {}", indent, annotation));
        }
    }

    lines
}
