use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use macroquad::prelude::{error, info, warn};

use crate::AnyError;
use crate::tree::Tree;

pub fn bytes_per_files(
    parent_name: String,
    folders: &Vec<PathBuf>,
    exclude: &HashSet<String>,
) -> Result<Tree, AnyError> {
    let mut nodes = Vec::new();
    for entry in folders {
        nodes.push(bytes_per_file(&entry, exclude)?);
    }
    let mut parent = Tree::new_from_children(parent_name, nodes);
    parent.get_or_compute_size();
    Ok(parent)
}
pub fn bytes_per_file(folder: &PathBuf, exclude: &HashSet<String>) -> Result<Tree, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if exclude.contains(&path_str) {
        info!("excluding {}", path_str);
        Ok(Tree::new_from_size(path_str, 0))
    } else if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", path_str);
        Ok(Tree::new_from_size(path_str, 0))
    } else if Path::new(folder).is_file() {
        Ok(Tree::new_from_size(
            path_str,
            fs::metadata(folder)
                .map_err(|e| format!("can not get metadata for {:?}: {}", folder, e))?
                .len() as i64,
        ))
    } else if Path::new(folder).is_dir() {
        let mut entries = Vec::new();
        match fs::read_dir(folder) {
            Ok(entries_dir) => {
                for entry in entries_dir {
                    match entry {
                        Ok(entry) => entries.push(entry.path()),
                        Err(e) => {
                            warn!("{} contains an unreadable file/folder: {}", path_str, e);
                        }
                    }
                }
                bytes_per_files(path_str, &entries, exclude)
            }
            Err(e) => {
                warn!("{} ignored folder due to error: {}", path_str, e);
                Ok(Tree::new_from_size(path_str, 0))
            }
        }
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
    exclude: &HashSet<String>,
) -> Result<Option<Tree>, AnyError> {
    let path = Path::new(folder);
    let path_str = folder.to_string_lossy().to_string();
    if exclude.contains(&path_str) {
        info!("excluding {}", path_str);
        Ok(None)
    } else if path.is_symlink() {
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
            let node_option = bytes_per_file_with_extension(&entry?.path(), extensions, exclude)?;
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
