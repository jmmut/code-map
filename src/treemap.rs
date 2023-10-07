use crate::node::Node;
use macroquad::prelude::{Rect, Vec2};

#[derive(Debug, Clone)]
pub struct MapNode {
    pub name: String,
    pub size: i64,
    pub rect: Option<Rect>,
    pub children: Vec<MapNode>,
}

pub struct Counts {
    pub total: usize,
    pub leafs: usize,
}

impl MapNode {
    pub fn new(raw_tree: Node) -> Self {
        let Node {
            name,
            size,
            children,
        } = raw_tree;

        let mut map_node = MapNode {
            name,
            size: size.unwrap(),
            rect: None,
            children: Vec::new(),
        };

        for child in children {
            map_node.children.push(MapNode::new(child));
        }
        map_node
    }

    #[allow(unused)]
    pub fn deepest_child(&self, point: Vec2) -> &MapNode {
        let mut result = self;
        for child in &self.children {
            if child.rect.unwrap().contains(point) {
                result = child.deepest_child(point);
            }
        }
        result
    }
    pub fn get_nested_by_position(&self, point: Vec2) -> Vec<&MapNode> {
        let mut result = Vec::new();
        result.push(self);
        for child in &self.children {
            if child.rect.unwrap().contains(point) {
                result.append(&mut child.get_nested_by_position(point));
            }
        }
        result
    }

    pub fn get_nested_by_name(&self, name: &str) -> Vec<&MapNode> {
        self.get_nested_by_name_recursive(name).1
    }
    fn get_nested_by_name_recursive(&self, name: &str) -> (bool, Vec<&MapNode>) {
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

    pub fn count_if<F: Fn(&MapNode) -> bool>(&self, predicate: &F) -> Counts {
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
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}
