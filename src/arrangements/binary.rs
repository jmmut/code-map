use macroquad::prelude::Rect;

use crate::treemap::MapNode;

pub fn arrange(node: &mut MapNode, rect: Rect) {
    node.rect = Some(rect);
    let rect = node.rect.unwrap();
    if rect.w * rect.h == 0.0 {
        return;
    } else {
        node.children.sort_by(|a, b| b.size.cmp(&a.size));
        arrange_nodes(&mut node.children, rect);
    }
}

fn arrange_nodes(nodes: &mut [MapNode], rect: Rect) {
    if nodes.len() == 0 {
    } else if nodes.len() == 1 {
        arrange(&mut nodes[0], rect);
    } else if nodes.len() == 2 {
        let (_half_index, half_size_coef) = get_half_size(nodes);
        let aspect_ratio = rect.w / rect.h;
        let (rect_1, rect_2) = if aspect_ratio >= 1.0 {
            divide_rectangle_horizontally(rect, half_size_coef)
        } else {
            divide_rectangle_vertically(rect, half_size_coef)
        };

        arrange(&mut nodes[0], rect_1);
        arrange(&mut nodes[1], rect_2);
    } else {
        let (half_index, half_size_coef) = get_half_size(nodes);
        let aspect_ratio = rect.w / rect.h;
        let (rect_1, rect_2) = if aspect_ratio >= 1.0 {
            divide_rectangle_horizontally(rect, half_size_coef)
        } else {
            divide_rectangle_vertically(rect, half_size_coef)
        };
        assert_ne!(half_index, nodes.len());
        assert_ne!(half_index, 0);

        arrange_nodes(nodes[..half_index].as_mut(), rect_1);
        arrange_nodes(nodes[half_index..].as_mut(), rect_2);
    }
}

fn get_half_size(nodes: &mut [MapNode]) -> (usize, f32) {
    let mut half_index = 1;
    let mut half_size = 0;
    let nodes_size = nodes.iter().map(|node| node.size).sum::<i64>();
    for (i, child) in nodes.iter().enumerate() {
        half_size += child.size;
        if half_size > nodes_size / 2 {
            half_index += i;
            break;
        }
    }
    let half_size_coef = half_size as f32 / nodes_size as f32;
    (half_index, half_size_coef)
}

fn divide_rectangle_horizontally(rect: Rect, coef: f32) -> (Rect, Rect) {
    let width_1 = rect.w * coef;
    let rect_1 = Rect::new(rect.x, rect.y, width_1, rect.h);
    let rect_2 = Rect::new(rect.x + width_1, rect.y, rect.w - width_1, rect.h);
    (rect_1, rect_2)
}

fn divide_rectangle_vertically(rect: Rect, coef: f32) -> (Rect, Rect) {
    let height_1 = rect.h * coef;
    let rect_1 = Rect::new(rect.x, rect.y, rect.w, height_1);
    let rect_2 = Rect::new(rect.x, rect.y + height_1, rect.w, rect.h - height_1);
    (rect_1, rect_2)
}

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
    use crate::arrangements::linear::tests::float_eq;
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
    fn test_basic_binary() {
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
        let area_linear = area(&tree.children);

        arrange(&mut tree, Rect::new(0.0, 0.0, 1.0, 1.0));
        let squareness_binary = average_squareness(
            &tree
                .children
                .iter()
                .map(|child| child.rect.unwrap())
                .collect::<Vec<_>>(),
        );
        let area_binary = area(&tree.children);

        assert!(
            squareness_binary > squareness_linear,
            "{} < {}",
            squareness_binary,
            squareness_linear
        );
        println!("squareness of square::arrange: {}", squareness_binary);
        assert!(
            float_eq(area_binary, area_linear),
            "{} == {}",
            area_binary,
            area_linear
        );
    }

    fn area(rectangles: &[MapNode]) -> f32 {
        rectangles
            .iter()
            .map(|node| node.rect.unwrap().w * node.rect.unwrap().h)
            .sum()
    }
}
