use macroquad::prelude::{Rect, Vec2};
use crate::node::Node;


#[derive(Debug, Clone)]
pub struct MapNode {
    pub name: String,
    pub size: i64,
    pub rect: Rect,
    pub children: Vec<MapNode>,
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
            rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            children: Vec::new(),
        };

        for child in children {
            map_node.children.push(MapNode::new(child));
        }
        map_node
    }

    /// aspect ratio is width / height, e.g. 16 / 9 = 1.7777
    pub fn arrange(&mut self, aspect_ratio: f32) {
        self.children.sort_by(|a, b| b.size.cmp(&a.size));
        if aspect_ratio > 1.0 {
            // arrange horizontally
            let mut previous_end = 0.0;
            for child in &mut self.children {
                let width = child.size as f32 / self.size as f32;
                child.rect = Rect::new(previous_end, 0.0, width, 1.0);
                previous_end += width;
            }
        } else {
            // arrange vertically
            todo!();
        }
    }

    pub fn deepest_child(&self, point: Vec2) -> &MapNode {
        let mut result = self;
        for child in &self.children {
            if child.rect.contains(point) {
                result = child.deepest_child(point);
            }
        }
        result
    }
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
         && self.size == other.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let child_1 = MapNode {
            name: "child1".to_string(),
            size: 50,
            rect: Rect::new(0.0, 0.0, 0.3, 1.0),
            children: vec![],
        };
        let child_2 = MapNode {
            name: "child2".to_string(),
            size: 50,
            rect: Rect::new(0.3, 0.0, 0.7, 1.0),
            children: vec![],
        };
        let map = MapNode {
            name: "root".to_string(),
            size: 100,
            rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            children: vec![
                child_1.clone(),
                child_2.clone(),
            ],
        };
        assert_eq!(map.deepest_child(Vec2::new(0.0, 0.0)), &child_1);
        assert_eq!(map.deepest_child(Vec2::new(0.5, 0.5)), &child_2);
    }
}