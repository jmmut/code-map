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
    pub fn overlapping(&self, point: Vec2) -> Vec<&MapNode> {
        let mut result = Vec::new();
        result.push(self);
        for child in &self.children {
            if child.rect.unwrap().contains(point) {
                result.append(&mut child.overlapping(point));
            }
        }
        result
    }

    /// Returns the count of leaf nodes (e.g. actual files in bytes-per-file) and total nodes (files + folders)
    pub fn count(&self) -> Counts {
        if self.is_leaf() {
            Counts { total: 1, leafs: 1 }
        } else {
            let mut counts = Counts { total: 1, leafs: 0 };
            for child in &self.children {
                let Counts { total, leafs } = child.count();
                counts.leafs += leafs;
                counts.total += total;
            }
            counts
        }
    }



    /// Returns the count of leaf nodes (e.g. actual files in bytes-per-file) and total nodes (files + folders)
    pub fn count_if<F: Fn(&MapNode) -> bool>(&self, predicate: F) -> Counts {
        let count_self = if predicate(self) { 1 } else { 0 };
        if self.is_leaf() {
            Counts {
                total: count_self,
                leafs: 1,
            }
        } else {
            let mut counts = Counts {
                total: count_self,
                leafs: 0,
            };
            for child in &self.children {
                let Counts { total, leafs } = child.count();
                counts.leafs += leafs;
                counts.total += total;
            }
            counts
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}
