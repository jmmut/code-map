use crate::treemap::MapNode;
use macroquad::prelude::Rect;

pub fn arrange(node: &mut MapNode, rect: Rect, pad: f32) {
    node.rect = Some(rect);
    let mut rect = node.rect.unwrap();
    let reduction = pad;
    rect.x += 1.0 * reduction;
    rect.y += 1.0 * reduction;
    rect.w -= 2.0 * reduction;
    rect.h -= 2.0 * reduction;

    node.children.sort_by(|a, b| b.size.cmp(&a.size));
    let aspect_ratio = rect.w / rect.h;
    if aspect_ratio > 1.0 {
        // arrange horizontally
        let mut previous_end = rect.x;
        for child in &mut node.children {
            let width = child.size as f32 / node.size as f32 * rect.w;
            arrange(child, Rect::new(previous_end, rect.y, width, rect.h), pad);
            previous_end += width;
        }
    } else {
        // arrange vertically
        let mut previous_end = rect.y;
        for child in &mut node.children {
            let height = child.size as f32 / node.size as f32 * rect.h;
            arrange(child, Rect::new(rect.x, previous_end, rect.w, height), pad);
            previous_end += height;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::node::Node;
    use macroquad::prelude::Vec2;

    #[test]
    fn test_contains() {
        let child_1 = MapNode {
            name: "child1".to_string(),
            size: 50,
            rect: Some(Rect::new(0.0, 0.0, 0.3, 1.0)),
            children: vec![],
        };
        let child_2 = MapNode {
            name: "child2".to_string(),
            size: 50,
            rect: Some(Rect::new(0.3, 0.0, 0.7, 1.0)),
            children: vec![],
        };
        let mut map = MapNode {
            name: "root".to_string(),
            size: 100,
            rect: Some(Rect::new(0.0, 0.0, 1.0, 1.0)),
            children: vec![child_1.clone(), child_2.clone()],
        };
        arrange(&mut map, Rect::new(0.0, 0.0, 1.0, 1.0), 0.0);
        assert_eq!(map.deepest_child(Vec2::new(0.0, 0.0)), &child_1);
        assert_eq!(map.deepest_child(Vec2::new(0.5, 0.5)), &child_2);
    }

    pub fn float_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }
    pub fn assert_float_eq(a: f32, b: f32) {
        assert!(float_eq(a, b), "floats not equal: {:?} == {:?}", a, b);
    }
    fn rect_eq(a: Rect, b: Rect) -> bool {
        float_eq(a.x, b.x) && float_eq(a.y, b.y) && float_eq(a.w, b.w) && float_eq(a.h, b.h)
    }
    fn assert_rect_eq(a: Rect, b: Rect) {
        assert!(rect_eq(a, b), "rects not equal: {:?} == {:?}", a, b);
    }

    #[test]
    fn test_arrange_recursive() {
        let mut map = MapNode::new(Node::new_from_children(
            "root".to_string(),
            vec![
                Node::new_from_children(
                    "child_1".to_string(),
                    vec![
                        Node::new_from_size("child_1_1".to_string(), 5),
                        Node::new_from_size("child_1_2".to_string(), 7),
                        Node::new_from_size("child_1_3".to_string(), 15),
                    ],
                ),
                Node::new_from_children(
                    "child_2".to_string(),
                    vec![
                        Node::new_from_size("child_2_1".to_string(), 3),
                        Node::new_from_size("child_2_2".to_string(), 20),
                        Node::new_from_size("child_2_3".to_string(), 10),
                    ],
                ),
            ],
        ));
        let toplevel_rect = Rect::new(0.0, 0.0, 200.0, 100.0);
        arrange(&mut map, toplevel_rect, 0.0);
        assert_rect_eq(map.rect.unwrap(), toplevel_rect);
        assert_rect_eq(
            map.children[0].rect.unwrap(),
            Rect::new(0.0, 0.0, 33.0 / 60.0 * toplevel_rect.w, 100.0),
        ); // moved the big one to the beginning
        assert_rect_eq(
            map.children[1].rect.unwrap(),
            Rect::new(
                33.0 / 60.0 * toplevel_rect.w,
                0.0,
                27.0 / 60.0 * toplevel_rect.w,
                100.0,
            ),
        );

        let width_0 = 33.0 / 60.0 * toplevel_rect.w;
        let width_00 = width_0 * 20.0 / 33.0;
        let width_01 = width_0 * 10.0 / 33.0;
        let width_02 = width_0 * 3.0 / 33.0;
        assert_rect_eq(
            map.children[0].children[0].rect.unwrap(),
            Rect::new(0.0, 0.0, width_00, 100.0),
        );
        assert_rect_eq(
            map.children[0].children[1].rect.unwrap(),
            Rect::new(width_00, 0.0, width_01, 100.0),
        );
        assert_rect_eq(
            map.children[0].children[2].rect.unwrap(),
            Rect::new(width_00 + width_01, 0.0, width_02, 100.0),
        );

        let width_1 = 27.0 / 60.0 * toplevel_rect.w;
        let height_1 = toplevel_rect.h;
        let height_10 = height_1 * 15.0 / 27.0;
        let height_11 = height_1 * 7.0 / 27.0;
        let height_12 = height_1 * 5.0 / 27.0;
        assert_rect_eq(
            map.children[1].children[0].rect.unwrap(),
            Rect::new(width_0, 0.0, width_1, height_10),
        );
        assert_rect_eq(
            map.children[1].children[1].rect.unwrap(),
            Rect::new(width_0, height_10, width_1, height_11),
        );
        assert_rect_eq(
            map.children[1].children[2].rect.unwrap(),
            Rect::new(width_0, height_10 + height_11, width_1, height_12),
        );
    }
}
