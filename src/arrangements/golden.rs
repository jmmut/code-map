use macroquad::prelude::Rect;

use crate::tree::Tree;

pub fn arrange(node: &mut Tree, rect: Rect) {
    node.rect = Some(rect);
    let rect = node.rect.unwrap();
    if rect.w * rect.h == 0.0 {
        return;
    } else {
        node.children.sort_by(|a, b| b.size().cmp(&a.size()));
        arrange_nodes(&mut node.children, rect);
    }
}

fn arrange_nodes(nodes: &mut [Tree], rect: Rect) {
    if nodes.len() == 0 {
    } else if nodes.len() == 1 {
        arrange(&mut nodes[0], rect);
    } else if nodes.len() == 2 {
        let (_half_index, half_size_coef) = get_half_size(nodes).unwrap();
        let aspect_diff = rect.w - rect.h;
        let (rect_1, rect_2) = if aspect_diff >= 0.0 {
            divide_rectangle_horizontally(rect, half_size_coef)
        } else {
            divide_rectangle_vertically(rect, half_size_coef)
        };

        arrange(&mut nodes[0], rect_1);
        arrange(&mut nodes[1], rect_2);
    } else {
        let (half_index, half_size_coef) = get_half_size(nodes).unwrap();
        let aspect_diff = rect.w - rect.h;
        let (rect_1, rect_2) = if aspect_diff >= 0.0 {
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

fn get_half_size(nodes: &mut [Tree]) -> Result<(usize, f32), String> {
    let mut half_size = 0;
    let nodes_size = nodes.iter().map(|child| child.size()).sum::<i64>();
    if nodes_size == 0 {
        Ok((nodes.len() / 2, 0.5))
    } else {
        for (i, child) in nodes.iter().enumerate() {
            half_size += child.size();
            if half_size as f64 >= (nodes_size as f64 * (1.0 - 0.713)) {
                // started with the golden ratio, but empirically this number is better
                assert_ne!(nodes_size, 0);
                let half_size_coef = half_size as f32 / nodes_size as f32;
                return Ok((i + 1, half_size_coef));
            }
        }
        Err(format!(
            "Can't split in half. half_size: {}, nodes_size: {}, nodes: {:#?}",
            half_size, nodes_size, nodes
        ))
    }
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

pub fn squareness(rect: &Rect) -> f32 {
    if rect.h == 0.0 {
        0.0
    } else if rect.w <= rect.h {
        rect.w / rect.h
    } else {
        rect.h / rect.w
    }
}

#[allow(unused)]
fn average_squareness(rectangles: &[Rect]) -> f32 {
    let mut sum = 0.0;
    for rect in rectangles {
        sum += squareness(rect);
    }
    sum / rectangles.len() as f32
}
