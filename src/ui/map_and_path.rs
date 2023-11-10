use macroquad::color::{Color, BEIGE, BLUE, GREEN, LIME, PINK, PURPLE, SKYBLUE, VIOLET, WHITE};
use macroquad::math::f32;
use macroquad::prelude::{
    draw_rectangle, draw_rectangle_lines, draw_text, is_key_pressed, measure_text, mouse_position,
    KeyCode, MouseButton, Rect, Vec2, BLACK, GRAY,
};

use crate::tree::{Tree, TreeView};
use crate::ui::rect_utils::{is_rect_clicked, round_rect};
use crate::ui::searcher::Searcher;
use crate::ui::set_if_different_or_unset_if_same;

const COLORS: &[Color] = &[
    BEIGE,
    Color::new(1.0, 0.40, 0.40, 1.00),
    PINK,
    PURPLE,
    VIOLET,
    SKYBLUE,
    BLUE,
    LIME,
    GREEN,
    WHITE,
];

pub fn choose_and_draw_map_and_path(
    tree: &Tree,
    units: &str,
    map_rect: Rect,
    font_size: f32,
    searcher: &mut Searcher,
    selected: &mut Option<Vec<TreeView>>,
    level: &mut Option<usize>,
) {
    if let Some(nested_nodes) = searcher.get_new_result() {
        *selected = Some(nested_nodes.clone());
        draw_colored_map_and_path(units, map_rect, font_size, &nested_nodes, level);
    } else if let Some(selected_nodes) = &selected {
        draw_colored_map_and_path(units, map_rect, font_size, &selected_nodes, level);
    } else {
        draw_hovered_nested_nodes(units, &tree, map_rect, font_size, level);
    }

    let Rect { x, y, w, h } = round_rect(map_rect);
    draw_rectangle_lines(x, y, w, h, 2.0, BLACK);
    draw_nodes_lines(&tree, map_rect, font_size, 1.0, BLACK);

    if is_key_pressed(KeyCode::Backspace) {
        if let Some(nested_nodes) = selected {
            nested_nodes.pop();
        }
    }
}

fn draw_colored_map_and_path(
    units: &str,
    map_rect: Rect,
    font_size: f32,
    nested_nodes: &Vec<TreeView>,
    level_opt: &mut Option<usize>,
) {
    if nested_nodes.len() > 0 {
        draw_path(units, map_rect, font_size, nested_nodes, level_opt);
        draw_colored_selected_in_map(nested_nodes, level_opt);
    }
}

fn draw_path(
    units: &str,
    map_rect: Rect,
    font_size: f32,
    nested_nodes: &Vec<TreeView>,
    level_opt: &mut Option<usize>,
) {
    let path_y = map_rect.y + map_rect.h + font_size * 4.5;
    let previous_width = draw_path_color(map_rect, font_size, nested_nodes, level_opt, path_y);
    draw_path_text(
        units,
        map_rect,
        font_size,
        nested_nodes,
        level_opt,
        previous_width,
        path_y,
    );
}

fn draw_path_color(
    map_rect: Rect,
    font_size: f32,
    nested_nodes: &Vec<TreeView>,
    level_opt: &mut Option<usize>,
    path_y: f32,
) -> f32 {
    let mut previous_width = 0.0;
    for (i, node) in nested_nodes.iter().enumerate() {
        let dimensions = measure_text(&node.name, None, font_size as u16, 1.0);
        let rect = Rect::new(
            map_rect.x + previous_width,
            path_y,
            dimensions.width - previous_width,
            1.5 * font_size,
        );
        if level_opt.is_some_and(|level| level < i) {
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                4.0,
                COLORS[i % COLORS.len()],
            );
        } else {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, COLORS[i % COLORS.len()]);
        }
        if is_rect_clicked(&rect, MouseButton::Left) {
            set_if_different_or_unset_if_same(level_opt, i);
        }
        previous_width = dimensions.width;
    }
    previous_width
}

fn draw_path_text(
    units: &str,
    map_rect: Rect,
    font_size: f32,
    nested_nodes: &Vec<TreeView>,
    level_opt: &mut Option<usize>,
    previous_width: f32,
    path_y: f32,
) {
    let path_rect = Rect::new(map_rect.x, path_y, previous_width, 1.5 * font_size);
    if is_rect_clicked(&path_rect, MouseButton::Right) {
        *level_opt = None;
    }
    let deepest_child = nested_nodes.last().unwrap();
    let size = if let Some(level) = level_opt {
        nested_nodes
            .get(*level)
            .map_or(deepest_child.size, |node| node.size)
    } else {
        deepest_child.size
    };
    let text = format!("{}: {} {}", deepest_child.name, size, units);

    draw_text(
        &text,
        map_rect.x,
        path_y + 1.0 * font_size,
        font_size,
        BLACK,
    );
    if let Some(level) = level_opt {
        if let Some(node) = nested_nodes.get(*level) {
            let deepest_text = format!("{}", deepest_child.name);
            draw_text(
                &deepest_text,
                map_rect.x,
                path_y + 1.0 * font_size,
                font_size,
                GRAY,
            );
            let text = format!("{}", node.name);
            draw_text(
                &text,
                map_rect.x,
                path_y + 1.0 * font_size,
                font_size,
                BLACK,
            );
        }
    }
}

fn draw_colored_selected_in_map(nested_nodes: &Vec<TreeView>, level_opt: &mut Option<usize>) {
    for (i, node) in nested_nodes.iter().enumerate() {
        if let Some(node_rect) = node.rect {
            let Rect { x, y, w, h } = round_rect(node_rect);
            if level_opt.is_some_and(|level| i > level) {
                let thickness = w.min(h).min(10.0);
                draw_rectangle_lines(x, y, w, h, thickness, COLORS[i % COLORS.len()]);
            } else {
                draw_rectangle(x, y, w, h, COLORS[i % COLORS.len()]);
            }
        } else {
            // a null rect can happen for empty folders or if a file has size 0
        }
    }
}

fn draw_hovered_nested_nodes(
    units: &str,
    treemap: &Tree,
    map_rect: Rect,
    font_size: f32,
    level: &mut Option<usize>,
) {
    let mouse_position = Vec2::from(mouse_position());
    if map_rect.contains(mouse_position) {
        let nodes_pointed = treemap.get_nested_by_position(mouse_position);
        draw_colored_map_and_path(
            units,
            map_rect,
            font_size,
            &TreeView::from_nodes(&nodes_pointed),
            level,
        );
    }
}

fn draw_nodes_lines(node: &Tree, map_rect: Rect, font_size: f32, thickness: f32, color: Color) {
    if let Some(rect) = node.rect {
        let Rect { x, y, w, h } = round_rect(rect);
        draw_rectangle_lines(x, y, w, h, thickness, color);
        // draw_text(
        //     &node.name,
        //     x + 1.5 * font_size,
        //     y + 1.5 * font_size,
        //     font_size,
        //     BLACK,
        // );
        // draw_text(
        //     &node.size.to_string(),
        //     x + 1.5 * font_size,
        //     y + 3.0 * font_size,
        //     font_size,
        //     BLACK,
        // );
        for child in &node.children {
            draw_nodes_lines(child, map_rect, font_size, thickness, color);
        }
    }
}
