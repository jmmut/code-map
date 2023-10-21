use std::path::PathBuf;

use crate::git_churn::{git_churn, FileChurn};
use crate::tree::Tree;
use crate::AnyError;

pub fn git_churn_per_file(folder: PathBuf) -> Result<Tree, AnyError> {
    let file_churns = git_churn(folder.clone())?;
    let tree = file_churns_to_tree(folder, file_churns);
    tree
}

fn file_churns_to_tree(folder: PathBuf, file_churns: Vec<FileChurn>) -> Result<Tree, AnyError> {
    let nodes_flat_list = file_churns_to_nodes(file_churns);
    let tree = nodes_flat_list_to_tree(nodes_flat_list, folder);
    tree
}

fn file_churns_to_nodes(file_churns: Vec<FileChurn>) -> Vec<Tree> {
    file_churns
        .into_iter()
        .map(|FileChurn { path, count }| Tree::new_from_size(path, count as i64))
        .collect::<Vec<Tree>>()
}

fn nodes_flat_list_to_tree(nodes: Vec<Tree>, folder: PathBuf) -> Result<Tree, AnyError> {
    let mut top_level_folder = folder.to_string_lossy().to_string();
    if top_level_folder.is_empty() {
        return Err("folder should not be empty".into());
    }

    let mut wrapping_tree = Tree {
        name: top_level_folder.clone(),
        size: None,
        rect: None,
        children: vec![],
    };
    let hierarchy_delimiter = "/";
    if !top_level_folder.ends_with(hierarchy_delimiter) {
        top_level_folder += hierarchy_delimiter;
    }
    for mut node in nodes {
        add_node_to_tree(
            &mut node,
            &mut wrapping_tree,
            &top_level_folder,
            hierarchy_delimiter,
        );
    }
    wrapping_tree.get_or_compute_size();
    Ok(wrapping_tree)
}

fn add_node_to_tree(
    node: &mut Tree,
    wrapping_tree: &mut Tree,
    top_level_folder: &String,
    hierarchy_delimiter: &str,
) {
    let mut path_parts = node
        .name
        .split(hierarchy_delimiter)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    node.name = top_level_folder.clone() + &node.name;
    *path_parts.first_mut().unwrap() = top_level_folder.clone() + path_parts.first().unwrap();
    let mut trees_current_level: *mut Tree = wrapping_tree;
    let mut path_level = "".to_string();
    for path_part in path_parts {
        path_level += &path_part;
        trees_current_level =
            get_existing_or_new_child_node(node, trees_current_level, path_level.clone());
        path_level += hierarchy_delimiter;
    }
}

fn get_existing_or_new_child_node(
    node: &mut Tree,
    trees_current_level: *mut Tree,
    path_level: String,
) -> *mut Tree {
    let found_subtree = find_child_node(&path_level, unsafe { &mut *trees_current_level });
    if let Some(path_level_exists) = found_subtree {
        path_level_exists
    } else {
        add_child_node(node, trees_current_level, path_level)
    }
}

fn find_child_node<'a>(path_level: &String, trees_current_level: &mut Tree) -> Option<*mut Tree> {
    for child in trees_current_level.children.iter_mut() {
        if child.name == *path_level {
            return Some(child);
        }
    }
    None
}

fn add_child_node(
    node: &mut Tree,
    trees_current_level: *mut Tree,
    path_level: String,
) -> *mut Tree {
    let new_node_for_path_level = create_new_node(node, path_level);
    unsafe {
        (*trees_current_level)
            .children
            .push(new_node_for_path_level);
        (*trees_current_level).children.last_mut().unwrap()
    }
}

fn create_new_node(node: &Tree, path_level: String) -> Tree {
    if node.name == path_level {
        create_leaf_node(node, path_level.clone())
    } else {
        create_intermediate_node(path_level)
    }
}

fn create_leaf_node(node: &Tree, path_level: String) -> Tree {
    Tree::new_from_size(path_level, node.size.unwrap())
}

fn create_intermediate_node(path_level: String) -> Tree {
    Tree {
        name: path_level,
        size: None,
        rect: None,
        children: vec![],
    }
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
            FileChurn { path: "src/arrangements/binary.rs".into(), count: 1 },
            FileChurn { path: "src/metrics/word_mentions.rs".into(), count: 2 },
            FileChurn { path: "src/main.rs".into(), count: 3 },
            FileChurn { path: "src/metrics/bytes_per_file.rs".into(), count: 4 },
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
            FileChurn { path: "main.rs".into(), count: 1 },
            FileChurn { path: "lib.rs".into(), count: 2 },
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
            FileChurn { path: "src/main.rs".into(), count: 1 },
            FileChurn { path: "src/lib.rs".into(), count: 2 },
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

    #[test]
    fn test_churn_tree_creation_complex_top_level() {
        #[rustfmt::skip]
            let file_churns = vec![
            FileChurn { path: "src/main.rs".into(), count: 1 },
            FileChurn { path: "src/lib.rs".into(), count: 2 },
        ];

        let tree = file_churns_to_tree("./src/../".into(), file_churns).unwrap();

        #[rustfmt::skip]
            let expected = Tree::new_from_computed_size("./src/../".into(), 3, vec![
            Tree::new_from_computed_size("./src/../src".into(), 3, vec![
                Tree::new_from_size("./src/../src/main.rs".into(), 1),
                Tree::new_from_size("./src/../src/lib.rs".into(), 2),
            ]),
        ]);
        assert_trees_eq(&tree, &expected);
    }
}
