use macroquad::prelude::Rect;
use crate::node::Node;


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
}
