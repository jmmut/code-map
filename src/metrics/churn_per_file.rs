use std::borrow::BorrowMut;
use std::ops::{DerefMut, Range};
use std::path::PathBuf;
use std::rc::Rc;

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
    // let mut trees = Rc::new(Vec::<Tree>::new());
    // let mut trees = Vec::<Tree>::new();
    let mut wrapping_tree = Tree {
        name: "".to_string(),
        size: None,
        rect: None,
        children: vec![],
    };
    let hierarchy_delimiter = "/";
    for node in nodes {
        let path = node.name.split(hierarchy_delimiter).collect::<Vec<&str>>();
        let mut path_level_iter = path.iter();
        let mut path_level = path_level_iter.next().unwrap().to_string();
        let mut trees_current_level: *mut Tree = &mut wrapping_tree;
        loop {
            assert_ne!(path_level, "");

            if let Some(existing_path_level) =
                get_subtree(&mut path_level, unsafe { &mut *trees_current_level })
            {
                trees_current_level = existing_path_level;
            } else {
                if node.name == path_level {
                    let new_node = Tree::new_from_size(path_level.to_string(), node.size.unwrap());
                    unsafe {
                        (*trees_current_level).children.push(new_node);
                    }
                } else {
                    let new_node = Tree {
                        name: path_level.to_string(),
                        size: None,
                        rect: None,
                        children: vec![],
                    };
                    unsafe {
                        (*trees_current_level).children.push(new_node);
                    }
                }

                unsafe {
                    trees_current_level = (*trees_current_level).children.last_mut().unwrap();
                }
            }

            if let Some(next_path_level) = path_level_iter.next() {
                path_level += hierarchy_delimiter;
                path_level += next_path_level;
            } else {
                break;
            }
        }
    }
    assert_eq!(wrapping_tree.children.len(), 1);
    let mut tree = wrapping_tree.children.pop().unwrap();
    tree.get_or_compute_size();
    tree
}

fn get_subtree<'a>(path_level: &String, trees_current_level: &mut Tree) -> Option<*mut Tree> {
    let mut found_subtree: Option<*mut Tree> = None;
    for child in trees_current_level.children.iter_mut() {
        if child.name == *path_level {
            found_subtree = Some(child);
            break;
        }
    }
    found_subtree
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