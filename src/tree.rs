use macroquad::prelude::{Rect, Vec2};

#[derive(Debug, Clone)]
pub struct Tree {
    pub name: String,
    pub size: Option<i64>,
    pub rect: Option<Rect>,
    pub children: Vec<Tree>,
}

#[derive(Clone)]
pub struct TreeView {
    pub name: String,
    pub size: i64,
    pub rect: Option<Rect>,
    pub children_count: usize,
}

pub struct Counts {
    pub total: usize,
    pub leafs: usize,
}

impl Tree {
    pub fn new_from_size(name: String, size: i64) -> Tree {
        Tree {
            name,
            size: Some(size),
            rect: None,
            children: Vec::new(),
        }
    }
    pub fn new_from_children(name: String, children: Vec<Tree>) -> Tree {
        let mut node = Tree {
            name,
            size: None,
            rect: None,
            children,
        };
        node.get_or_compute_size();
        node
    }

    pub fn get_or_compute_size(&mut self) -> i64 {
        if let Some(size) = self.size {
            size
        } else {
            let mut size = 0;
            for child in &mut self.children {
                size += child.get_or_compute_size();
            }
            self.size = Some(size);
            size
        }
    }

    #[allow(unused)]
    pub fn deepest_child(&self, point: Vec2) -> &Tree {
        let mut result = self;
        for child in &self.children {
            if child.rect.unwrap().contains(point) {
                result = child.deepest_child(point);
            }
        }
        result
    }
    pub fn get_nested_by_position(&self, point: Vec2) -> Vec<&Tree> {
        let mut result = Vec::new();
        result.push(self);
        for child in &self.children {
            if child.rect.unwrap().contains(point) {
                result.append(&mut child.get_nested_by_position(point));
            }
        }
        result
    }

    pub fn get_nested_by_name(&self, name: &str) -> Vec<&Tree> {
        self.get_nested_by_name_recursive(name).1
    }
    fn get_nested_by_name_recursive(&self, name: &str) -> (bool, Vec<&Tree>) {
        if self.name == name {
            (true, vec![self])
        } else {
            for child in &self.children {
                let (found, mut subnodes) = child.get_nested_by_name_recursive(name);
                if found {
                    let mut nodes = vec![self];
                    nodes.append(&mut subnodes);
                    return (true, nodes);
                }
            }
            (false, vec![])
        }
    }

    /// Returns the count of leaf nodes (e.g. actual files in bytes-per-file) and total nodes (files + folders)
    pub fn count(&self) -> Counts {
        self.count_if(&|_| true)
    }

    pub fn count_visible(&self) -> Counts {
        self.count_if(&|node| node.rect.is_some())
    }

    pub fn count_if<F: Fn(&Tree) -> bool>(&self, predicate: &F) -> Counts {
        let count_self = if predicate(self) { 1 } else { 0 };
        if self.is_leaf() {
            Counts {
                total: count_self,
                leafs: count_self,
            }
        } else {
            let mut counts = Counts {
                total: count_self,
                leafs: 0,
            };
            for child in &self.children {
                let Counts { total, leafs } = child.count_if(predicate);
                counts.leafs += leafs;
                counts.total += total;
            }
            counts
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn search(&self, search_word: &str, limit: usize) -> Vec<String> {
        // let search_word = search_word.to_lowercase();
        let mut result = Vec::new();
        self.search_recursive(&search_word, limit, &mut result);
        result
    }
    fn search_recursive(&self, search_word: &str, limit: usize, result: &mut Vec<String>) {
        if result.len() < limit {
            if self.name.contains(search_word) || self.name.to_lowercase().contains(search_word) {
                result.push(self.name.clone());
            }
            for child in &self.children {
                child.search_recursive(search_word, limit, result);
            }
        }
    }

    fn compute_recursively<R, F: Fn(&Tree, R) -> R>(&self, f: F, initial: R) -> R {
        let current_result = f(self, initial);
        current_result
    }

    pub fn compute_squareness(&self) -> f32 {
        5.0
    }
    pub fn size(&self) -> i64 {
        self.size.unwrap()
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

impl TreeView {
    pub fn from_node(node: &Tree) -> Self {
        Self {
            name: node.name.clone(),
            size: node.size.unwrap(),
            rect: node.rect.clone(),
            children_count: node.children.len(),
        }
    }
    pub fn from_nodes(nodes: &[&Tree]) -> Vec<Self> {
        nodes.iter().map(|n| Self::from_node(n)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrangements::binary;

    #[test]
    fn test_basic_size_computation() {
        let mut tree = Tree::new_from_children(
            "root".to_string(),
            vec![
                Tree::new_from_size("child1".to_string(), 5),
                Tree::new_from_size("child2".to_string(), 7),
            ],
        );

        assert_eq!(tree.get_or_compute_size(), 12);
        assert_eq!(tree.size, Some(12));
    }

    #[test]
    fn test_compute_squareness() {
        let mut tree = Tree::new_from_children(
            "parent".to_string(),
            vec![
                Tree::new_from_size("child_1".to_string(), 5),
                Tree::new_from_size("child_2".to_string(), 7),
            ],
        );
        binary::arrange(&mut tree, Rect::new(0.0, 0.0, 1.0, 1.0));
        let squareness = tree.compute_squareness();
        assert_eq!(squareness, 0.0);
    }
}
