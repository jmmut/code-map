use macroquad::prelude::Rect;

use crate::treemap::MapNode;

pub fn arrange(tree: &mut MapNode, rect: Rect) {}

fn squareness(rect: &Rect) -> f32 {
    if rect.h == 0.0 {
        0.0
    } else if rect.w <= rect.h {
        rect.w / rect.h
    } else {
        rect.h / rect.w
    }
}

fn average_squareness(rectangles: &[Rect]) -> f32 {
    let mut sum = 0.0;
    for rect in rectangles {
        sum += squareness(rect);
    }
    sum / rectangles.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrangements::linear;
    use crate::node::Node;

    #[test]
    fn test_squareness() {
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 1.0, 2.0)), 0.5);
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 2.0, 1.0)), 0.5);
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 0.0, 1.0)), 0.0);
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 1.0, 0.0)), 0.0);
        assert_eq!(squareness(&Rect::new(0.0, 0.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_basic_square() {
        let mut children = Vec::new();
        for i in 1..=10 {
            children.push(Node::new_from_size(format!("child_{}", i), i));
        }
        let tree = Node::new_from_children("parent".to_string(), children);
        let mut tree = MapNode::new(tree);
        linear::arrange(&mut tree, Rect::new(0.0, 0.0, 1.0, 1.0), 0.0);
        let squareness_linear = average_squareness(
            &tree
                .children
                .iter()
                .map(|child| child.rect.unwrap())
                .collect::<Vec<_>>(),
        );

        arrange(&mut tree, Rect::new(0.0, 0.0, 1.0, 1.0));
        let squareness_square = average_squareness(
            &tree
                .children
                .iter()
                .map(|child| child.rect.unwrap())
                .collect::<Vec<_>>(),
        );

        assert!(
            squareness_square < squareness_linear,
            "{} < {}",
            squareness_square,
            squareness_linear
        );
    }
}
