use macroquad::color::{Color, BEIGE, BLUE, GREEN, LIME, PINK, PURPLE, SKYBLUE, VIOLET, WHITE};
use macroquad::math::f32;
use macroquad::prelude::{
    draw_rectangle, draw_rectangle_lines, draw_text, measure_text, mouse_position, MouseButton,
    Rect, Vec2, BLACK, GRAY,
};
use std::collections::VecDeque;

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
    let node_name_widths = compute_name_widths(nested_nodes, font_size);
    let top_left = Vec2::new(map_rect.x, path_y);
    draw_path_color(top_left, font_size, &node_name_widths, level_opt);
    draw_path_text(
        units,
        top_left,
        font_size,
        nested_nodes,
        level_opt,
        node_name_widths,
    );
}

fn compute_name_widths(nested_nodes: &Vec<TreeView>, font_size: f32) -> VecDeque<f32> {
    nested_nodes
        .iter()
        .map(|node| {
            let dimensions = measure_text(&node.name, None, font_size as u16, 1.0);
            dimensions.width
        })
        .collect()
}

fn draw_path_color(
    top_left: Vec2,
    font_size: f32,
    node_name_widths: &VecDeque<f32>,
    level_opt: &mut Option<usize>,
) {
    let mut previous_width = 0.0;
    for (i, width) in node_name_widths.iter().enumerate() {
        let rect = Rect::new(
            top_left.x + previous_width,
            top_left.y,
            width - previous_width,
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
        previous_width = *width;
    }
}

fn draw_path_text(
    units: &str,
    top_left: Vec2,
    font_size: f32,
    nested_nodes: &Vec<TreeView>,
    level_opt: &mut Option<usize>,
    mut node_name_widths: VecDeque<f32>,
) {
    node_name_widths.push_front(0.0);
    let previous_width = *node_name_widths.back().unwrap();
    let path_rect = Rect::new(top_left.x, top_left.y, previous_width, 1.5 * font_size);
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
    let selected_node_name = if let Some(level) = level_opt {
        if let Some(node) = nested_nodes.get(*level) {
            Some(node.name.clone())
        } else {
            None
        }
    } else {
        None
    };
    let deepest_child_color = if selected_node_name.is_none() {
        BLACK
    } else {
        GRAY
    };

    let deepest_text = format!("{}", deepest_child.name);
    draw_text(
        &deepest_text,
        top_left.x,
        top_left.y + 1.0 * font_size,
        font_size,
        deepest_child_color,
    );

    if let Some(node_name) = selected_node_name {
        let text = format!("{}", node_name);
        draw_text(
            &text,
            top_left.x,
            top_left.y + 1.0 * font_size,
            font_size,
            BLACK,
        );
    }

    let size_text = format!("{} {}", size, units);
    let text_width = measure_text(&size_text, None, font_size as u16, 1.0).width;
    let pad = 0.5 * font_size;
    let index = if let Some(level) = level_opt {
        if *level < node_name_widths.len() {
            *level
        } else {
            node_name_widths.len() - 2
        }
    } else {
        node_name_widths.len() - 2
    };
    let metric_x = node_name_widths.get(index).unwrap();
    draw_rectangle(
        top_left.x + metric_x,
        top_left.y + 1.5 * font_size,
        text_width + 2.0 * pad,
        1.5 * font_size,
        COLORS[index % COLORS.len()],
    );
    draw_text(
        &size_text,
        top_left.x + metric_x + pad,
        top_left.y + 2.5 * font_size,
        font_size,
        BLACK,
    );
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
