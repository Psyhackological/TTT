use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Enum for Task Type
#[derive(Debug, Serialize, Clone)]
pub enum TaskType {
    #[serde(rename = "section")]
    Section,

    #[serde(rename = "task")]
    Task,
}

#[derive(Debug)]
pub struct TocItem {
    pub task_type: TaskType,
    pub indent: Option<u8>,
    pub content: String,
}

/// Function to extract indentation and content from the input file
pub fn extract_toc_items(filename: &Path) -> Result<Vec<TocItem>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut toc_items = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let indent_level = count_indent(&line);

        // Trim the leading spaces to get the content
        let content = line.trim().to_string();

        let toc_item_to_push = match indent_level {
            1 if content.chars().next().unwrap().is_numeric() => TocItem {
                task_type: TaskType::Section,
                indent: None,
                content,
            },
            _ => TocItem {
                task_type: TaskType::Task,
                indent: Some(indent_level),
                content,
            },
        };

        toc_items.push(toc_item_to_push);
    }

    Ok(toc_items)
}

/// Helper function to count indentation level based on the number of leading spaces
/// 0 spaces -> 1, 4 spaces -> 2, 8 spaces -> 3, etc.
fn count_indent(line: &str) -> u8 {
    let num_spaces = line.chars().take_while(|c| *c == ' ').count();
    // Every 4 spaces equals one indentation level, starting from 1.
    (num_spaces / 4 + 1) as u8
}
