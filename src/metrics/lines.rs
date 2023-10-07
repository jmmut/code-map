use crate::metrics::bytes_per_file::has_allowed_extension;
use crate::metrics::word_mentions::TEXT_FILE_EXTENSIONS;
use crate::node::Node;
use crate::AnyError;
use macroquad::prelude::{error, warn};
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn lines_per_file(folder: &PathBuf) -> Result<Option<Node>, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
        Ok(None)
    } else if Path::new(folder).is_file() {
        if has_allowed_extension(folder, TEXT_FILE_EXTENSIONS) {
            Ok(Some(Node::new_from_size(
                path_str,
                count_lines_in_file(folder)? as i64,
            )))
        } else {
            Ok(None)
        }
    } else if Path::new(folder).is_dir() {
        let mut nodes = Vec::new();
        for entry in fs::read_dir(folder)? {
            let node_option = lines_per_file(&entry?.path())?;
            if let Some(node) = node_option {
                nodes.push(node);
            }
        }
        let mut parent = Node::new_from_children(path_str, nodes);
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
