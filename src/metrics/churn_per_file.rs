use macroquad::prelude::info;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::git_churn::{git_churn, FileChurn};
use crate::tree::Tree;
use crate::AnyError;

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
    file_churns
        .into_iter()
        .map(|FileChurn { path, count }| Tree::new_from_size(path, count as i64))
        .collect::<Vec<Tree>>()
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

    fn assert_trees_eq(left: &Tree, right: &Tree) {
        // using this exact message makes CLion show a great diff UI in the test runner.
        // this is probably brittle and will stop working in the future but for now it's great :D
        assert!(
            left.recursive_equals(right),
            "assertion failed: `(left == right)`\n  left: `{:#?}`,\n right: `{:#?}`",
            left,
            right
        );
    }

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
                    Tree::new_from_size("./src/arrangements/binary.rs".into(), 1),
                ]),
                Tree::new_from_computed_size("./src/metrics".into(), 6, vec![
                    Tree::new_from_size("./src/metrics/word_mentions.rs".into(), 2),
                    Tree::new_from_size("./src/metrics/bytes_per_file.rs".into(), 4),
                ]),
                Tree::new_from_size("./src/main.rs".into(), 3),
            ]),
        ]);
        assert_trees_eq(&tree, &expected);
    }

    #[test]
    fn test_churn_tree_creation_basic() {
        #[rustfmt::skip]
        let file_churns = vec![
            FileChurn { path: "./main.rs".into(), count: 1 },
            FileChurn { path: "./lib.rs".into(), count: 2 },
        ];

        let tree = file_churns_to_tree(".".into(), file_churns).unwrap();

        #[rustfmt::skip]
        let expected = Tree::new_from_computed_size(".".into(), 3, vec![
            Tree::new_from_size("./main.rs".into(), 1),
            Tree::new_from_size("./lib.rs".into(), 2),
        ]);
        assert_trees_eq(&tree, &expected);
    }

    #[test]
    fn test_churn_tree_creation_one_level_deep() {
        #[rustfmt::skip]
        let file_churns = vec![
            FileChurn { path: "./src/main.rs".into(), count: 1 },
            FileChurn { path: "./src/lib.rs".into(), count: 2 },
        ];

        let tree = file_churns_to_tree(".".into(), file_churns).unwrap();

        #[rustfmt::skip]
        let expected = Tree::new_from_computed_size(".".into(), 3, vec![
            Tree::new_from_computed_size("./src".into(), 3, vec![
                Tree::new_from_size("./src/main.rs".into(), 1),
                Tree::new_from_size("./src/lib.rs".into(), 2),
            ]),
        ]);
        assert_trees_eq(&tree, &expected);
    }
}
