use std::fs;
use std::path::{Path, PathBuf};

use macroquad::prelude::{error, warn};

use crate::tree::Tree;
use crate::AnyError;

pub fn bytes_per_file(folder: &PathBuf) -> Result<Tree, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
        Ok(Tree::new_from_size(path_str, 0))
    } else if Path::new(folder).is_file() {
        Ok(Tree::new_from_size(
            path_str,
            fs::metadata(folder)?.len() as i64,
        ))
    } else if Path::new(folder).is_dir() {
        let mut nodes = Vec::new();
        for entry in fs::read_dir(folder)? {
            nodes.push(bytes_per_file(&entry?.path())?);
        }
        let mut parent = Tree::new_from_children(path_str, nodes);
        parent.get_or_compute_size();
        Ok(parent)
    } else {
        error!(
            "{} is not a file nor a directory nor a symlink. Ignoring...",
            path_str
        );
        Ok(Tree::new_from_size(path_str, 0))
    }
}

pub fn bytes_per_file_with_extension(
    folder: &PathBuf,
    extensions: &[&str],
) -> Result<Option<Tree>, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
        Ok(None)
    } else if Path::new(folder).is_file() {
        if has_allowed_extension(folder, extensions) {
            Ok(Some(Tree::new_from_size(
                path_str,
                fs::metadata(folder)?.len() as i64,
            )))
        } else {
            Ok(None)
        }
    } else if Path::new(folder).is_dir() {
        let mut nodes = Vec::new();
        for entry in fs::read_dir(folder)? {
            let node_option = bytes_per_file_with_extension(&entry?.path(), extensions)?;
            if let Some(node) = node_option {
                nodes.push(node);
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

pub fn has_allowed_extension(file: &PathBuf, extensions: &[&str]) -> bool {
    if let Some(file_extension_os) = file.extension() {
        if let Some(file_extension) = file_extension_os.to_str() {
            extensions.contains(&file_extension)
        } else {
            false
        }
    } else {
        false
    }
}
