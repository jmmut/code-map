use crate::node::Node;
use crate::AnyError;
use macroquad::prelude::{error, warn};
use std::fs;
use std::path::Path;

pub fn bytes_per_file(folder: &str) -> Result<Node, AnyError> {
    let path = Path::new(folder);
    if path.is_symlink() {
        warn!("{} is a symlink and will be ignored", folder);
        Ok(Node::new_from_size(folder.to_string(), 0))
    } else if Path::new(folder).is_file() {
        Ok(Node::new_from_size(
            folder.to_string(),
            fs::metadata(folder)?.len() as i64,
        ))
    } else if Path::new(folder).is_dir() {
        let mut nodes = Vec::new();
        for entry in fs::read_dir(folder)? {
            nodes.push(bytes_per_file(entry?.path().to_str().unwrap())?);
        }
        let mut parent = Node::new_from_children(folder.to_string(), nodes);
        parent.get_or_compute_size();
        Ok(parent)
    } else {
        error!(
            "{} is not a file nor a directory nor a symlink. Ignoring...",
            folder
        );
        Ok(Node::new_from_size(folder.to_string(), 0))
    }
}
