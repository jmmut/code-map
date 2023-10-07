use crate::node::Node;
use crate::AnyError;
use macroquad::prelude::{error, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const TEXT_FILE_EXTENSIONS: &[&str] = &[
    ".c", ".cc", ".cpp", ".cs", ".css", ".go", ".h", ".hpp", ".html", ".java", ".js", ".json",
    ".jsx", ".md", ".php", ".py", ".rs", ".sh", ".ts", ".tsx", ".txt", ".xml", ".yaml", ".yml",
];

pub fn word_mentions(folder: &PathBuf) -> Result<Node, AnyError> {
    let mut mentions = HashMap::new();
    word_mentions_recursive(folder, &mut mentions)?;
    let mut nodes = Vec::new();
    nodes.reserve(mentions.len());
    for (word, count) in mentions {
        nodes.push(Node::new_from_size(word, count));
    }
    Ok(Node::new_from_children("".to_string(), nodes))
}

#[must_use]
fn word_mentions_recursive(
    folder: &PathBuf,
    mentions: &mut HashMap<String, i64>,
) -> Result<(), AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
    } else if Path::new(folder).is_file() {
        count_word_mentions_in_file(folder, mentions)?;
    } else if Path::new(folder).is_dir() {
        for entry in fs::read_dir(folder)? {
            word_mentions_recursive(&entry?.path(), mentions)?;
        }
    } else {
        error!(
            "{} is not a file nor a directory nor a symlink. Ignoring...",
            path_str
        );
    }
    Ok(())
}

#[must_use]
fn count_word_mentions_in_file(
    file: &PathBuf,
    mentions: &mut HashMap<String, i64>,
) -> Result<(), AnyError> {
    if is_text_file(file) {
        let file_str = file.to_string_lossy().to_string();
        let file_content = fs::read_to_string(file)?;
        for word in file_content.split_terminator(|c :char| {
            !c.is_alphabetic()
        }) {
            if !word.is_empty() {
                let count = mentions.entry(word.to_string()).or_insert(0);
                *count += 1;
            }
        }
    }
    Ok(())
}

fn is_text_file(file: &PathBuf) -> bool {
    let file_str = file.to_string_lossy().to_string();
    for extension in TEXT_FILE_EXTENSIONS {
        if file_str.ends_with(extension) {
            return true;
        }
    }
    false
}
