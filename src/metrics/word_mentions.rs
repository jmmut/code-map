use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use macroquad::prelude::{error, warn};

use crate::metrics::bytes_per_file::has_allowed_extension;
use crate::tree::Tree;
use crate::AnyError;

#[rustfmt::skip]
pub const TEXT_FILE_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "cs", "css", "go", ".gitignore", "h", "hpp", "html",
    "java", "js", "json", "jsx", "m", "mm", "md", "pbproj", "php", "py",
    "rs", "sh", "swift", "ts", "tsx", "txt", "xml", "yaml", "yml",
];

#[rustfmt::skip]
pub const CODE_FILE_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "cs", "css", "go", "h", "hpp", "html",
    "java", "js", "json", "jsx", "m", "mm", "md", "pbproj", "php", "py",
    "rs", "sh", "swift", "ts", "tsx", "yaml", "yml",
];

pub fn word_mentions(folder: &PathBuf) -> Result<Tree, AnyError> {
    let mut mentions = HashMap::new();
    word_mentions_recursive(folder, &mut mentions)?;
    let mut nodes = Vec::new();
    nodes.reserve(mentions.len());
    for (word, count) in mentions {
        nodes.push(Tree::new_from_size(word, count));
    }
    Ok(Tree::new_from_children("".to_string(), nodes))
}

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

fn count_word_mentions_in_file(
    file: &PathBuf,
    mentions: &mut HashMap<String, i64>,
) -> Result<(), AnyError> {
    if is_text_file(file) {
        let file_content = fs::read_to_string(file)?;
        for word in file_content.split_terminator(|c: char| !c.is_alphanumeric() && c != '_') {
            if !word.is_empty() {
                let count = mentions.entry(word.to_string()).or_insert(0);
                *count += 1;
            }
        }
    }
    Ok(())
}

fn is_text_file(file: &PathBuf) -> bool {
    has_allowed_extension(file, TEXT_FILE_EXTENSIONS)
}
