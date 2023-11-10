use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use ignore::gitignore::Gitignore;
use macroquad::prelude::{error, warn};

use crate::metrics::bytes_per_file::has_allowed_extension;
use crate::metrics::word_mentions::{CODE_FILE_EXTENSIONS, TEXT_FILE_EXTENSIONS};
use crate::tree::Tree;
use crate::AnyError;

pub fn lines_per_file(folder: &PathBuf) -> Result<Option<Tree>, AnyError> {
    lines_per_file_recursive(folder, None)
}

pub fn lines_per_file_recursive(
    folder: &PathBuf,
    higher_gitignore: Option<&Gitignore>,
) -> Result<Option<Tree>, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
        Ok(None)
    } else if Path::new(folder).is_file() {
        if has_allowed_extension(folder, CODE_FILE_EXTENSIONS) {
            Ok(Some(Tree::new_from_size(
                path_str,
                count_lines_in_file(folder)? as i64,
            )))
        } else {
            Ok(None)
        }
    } else if Path::new(folder).is_dir() {
        let mut nodes = Vec::new();
        let mut gitignore = None;
        for entry_res in fs::read_dir(folder)? {
            let entry = entry_res?;
            if entry.path().ends_with(".gitignore") {
                let (new_gitignore, error) = Gitignore::new(entry.path());
                if let Some(e) = error {
                    error!("Error parsing .gitignore file: {}", e);
                }
                gitignore = Some(new_gitignore)
            }
        }
        let chosen_gitignore = gitignore.as_ref().or(higher_gitignore);
        for entry_res in fs::read_dir(folder)? {
            let entry = entry_res?;
            let should_ignore = if let Some(gitignore) = chosen_gitignore {
                gitignore
                    .matched(&entry.path(), entry.path().is_dir())
                    .is_ignore()
            } else {
                false
            };
            if !should_ignore {
                let node_option = lines_per_file_recursive(&entry.path(), chosen_gitignore)?;
                if let Some(node) = node_option {
                    nodes.push(node);
                }
            }
        }
        let mut parent = Tree::new_from_children(path_str, nodes);
        parent.get_or_compute_size();
        Ok(Some(parent))
    } else {
        error!(
            "{} is not a file nor a directory nor a symlink. Ignoring...",
            path_str
        );
        Ok(None)
    }
}

fn count_lines_in_file(file: &PathBuf) -> Result<usize, AnyError> {
    if is_text_file(file) {
        let file_handle = File::open(file)?;
        let lines = std::io::BufReader::new(file_handle).lines();
        let mut count = 0;
        for _ in lines {
            // this is so stupid. However, it's not obvious to me that I could write something
            // faster than this, due to the buffering provided by BufReader
            count += 1;
        }
        Ok(count)
    } else {
        Err(format!("Not a text file: {}", file.to_string_lossy().to_string()).into())
    }
}

fn is_text_file(file: &PathBuf) -> bool {
    has_allowed_extension(file, TEXT_FILE_EXTENSIONS)
}
