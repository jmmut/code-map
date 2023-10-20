use macroquad::prelude::info;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::git_churn::{FileChurn, git_churn};
use crate::AnyError;
use crate::tree::Tree;

pub fn git_churn_per_file(folder: PathBuf) -> Result<Tree, AnyError> {
    let file_churns = git_churn(folder.clone())?;
    let tree = file_churns_to_tree(folder, file_churns);
    tree
}

fn file_churns_to_tree(root_path: PathBuf, file_churns: Vec<FileChurn>) -> Result<Tree, AnyError> {
    let nodes = file_churns_to_nodes(file_churns);
    let tree = nodes_to_tree(nodes);
    Ok(tree)
}

fn file_churns_to_nodes(file_churns: Vec<FileChurn>) -> Vec<Tree> {
    file_churns.into_iter().map(|FileChurn{path, count}| {
        Tree::new_from_size(path, count as i64)
    }).collect::<Vec<Tree>>()
}

fn nodes_to_tree(nodes: Vec<Tree>) -> Tree {
    let mut tree = Tree::new_from_children(".".to_string(), nodes);
    tree.get_or_compute_size();
    tree

    /*
    let tree
    for FileChurn { path, count} in file_churns {
        let node = Tree
        let mut path = path.split("/").collect::<Vec<&str>>();
        let mut current_node = &mut tree;
        for subfolder in path {
            assert_ne!(subfolder, "");
            if let Some(child) = current_node.children.iter_mut().find(|c| c.name == name) {
                current_node = child;
            } else {
                let new_node = Tree::new_from_children(name.to_string(), Vec::new());
                current_node.children.push(new_node);
                current_node = current_node.children.last_mut().unwrap();
            }
        }
        current_node.size = count;
    }
    tree.get_or_compute_size();
    info!("tree: {:?}", tree);
    Ok(Some(tree))

     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_churn_tree_creation() {
        #[rustfmt::skip]
        let file_churns = vec![
            FileChurn { path: "./src/arrangements/binary.rs".into(), count: 1 },
            FileChurn { path: "./src/metrics/word_mentions.rs".into(), count: 2 },
            FileChurn { path: "./src/main.rs".into(), count: 3 },
            FileChurn { path: "./src/metrics/bytes_per_file.rs".into(), count: 4 },
        ];

        let tree = file_churns_to_tree(".".into(), file_churns).unwrap();

        #[rustfmt::skip]
        let expected = Tree::new_from_computed_size(".".into(), 10, vec![
            Tree::new_from_computed_size("./src".into(), 10, vec![
                Tree::new_from_computed_size("./src/arrangements".into(), 1, vec![
                    Tree::new_from_computed_size("./src/arrangements/binary.rs".into(), 1, vec![]),
                ]),
                Tree::new_from_computed_size("./src/metrics".into(), 6, vec![
                    Tree::new_from_computed_size("./src/metrics/word_mentions.rs".into(), 2, vec![]),
                    Tree::new_from_computed_size("./src/metrics/bytes_per_file.rs".into(), 4, vec![]),
                ]),
                Tree::new_from_computed_size("./src/main.rs".into(), 3, vec![]),
            ]),
        ]);

        // assert!(tree.recursive_equals(expected), "{:?} != {:?}", tree, expected);
    }
}
