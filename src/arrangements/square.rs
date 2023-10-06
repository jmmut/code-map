use macroquad::prelude::Rect;

use crate::treemap::MapNode;

pub fn arrange(tree: &mut MapNode, rect: Rect) {}

fn squareness(rect: Rect) -> f32 {
    if rect.h == 0.0 {
        0.0
    } else if rect.w <= rect.h {
        rect.w / rect.h
    } else {
        rect.h / rect.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    #[test]
    fn test_squareness() {
        assert_eq!(squareness(Rect::new(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(squareness(Rect::new(0.0, 0.0, 1.0, 2.0)), 0.5);
        assert_eq!(squareness(Rect::new(0.0, 0.0, 2.0, 1.0)), 0.5);
        assert_eq!(squareness(Rect::new(0.0, 0.0, 0.0, 1.0)), 0.0);
        assert_eq!(squareness(Rect::new(0.0, 0.0, 1.0, 0.0)), 0.0);
        assert_eq!(squareness(Rect::new(0.0, 0.0, 0.0, 0.0)), 0.0);
    }
    
    #[test]
    fn test_basic_square() {
        let mut children = Vec::new();
        for i in 1..=10 {
            children.push(Node::new_from_size(format!("child_{}", i), i));
        }
        let tree = Node::new_from_children(
            "parent".to_string(),
            children,
        );
        let mut tree = MapNode::new(tree);
        arrange(&mut tree, Rect::new(0.0, 0.0, 1.0, 1.0));

    }
}
