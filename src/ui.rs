pub mod searcher;

use macroquad::color::{Color, BEIGE, BLUE, GREEN, LIME, PINK, PURPLE, SKYBLUE, VIOLET, WHITE};
use macroquad::math::f32;
use macroquad::prelude::{
    clear_background, debug, draw_rectangle, draw_rectangle_lines, draw_text, is_key_pressed,
    is_mouse_button_pressed, measure_text, mouse_position, screen_height, screen_width, KeyCode,
    MouseButton, Rect, Vec2, BLACK, LIGHTGRAY,
};

use crate::tree::{Tree, TreeView};
use crate::ui::searcher::Searcher;

const FONT_SIZE: f32 = 16.0;

pub struct Ui {
    pub tree: Tree,
    pub units: String,
    pub available: Rect,
    pub searcher: Searcher,
    pub font_size: f32,
    selected: Option<Vec<TreeView>>,
}

impl Ui {
    pub fn new(tree: Tree, units: &str) -> Self {
        let width = screen_width();
        let height = screen_height();
        let available = round_rect(Rect::new(
            width * 0.05,
            width * 0.05, // yes, width, not height. this makes the padding the same in both directions
            width * 0.9,
            height * 0.75,
        ));

        let font_size = choose_font_size(width, height);
        let searcher = Searcher::new(
            Rect::new(
                available.x,
                available.y + available.h + font_size * 3.0,
                available.w,
                font_size * 1.5,
            ),
            font_size,
        );
        Self {
            tree,
            units: units.to_string(),
            available,
            font_size,
            searcher,
            selected: None,
        }
    }

    pub fn draw(&mut self) {
        clear_background(LIGHTGRAY);

        all_draw(
            &mut self.tree,
            &self.units,
            self.available,
            self.font_size,
            &mut self.searcher,
            &mut self.selected,
        );
    }
}

fn choose_font_size(width: f32, height: f32) -> f32 {
    let min_side = width.min(height * 16.0 / 9.0);
    FONT_SIZE
        * if min_side < 1600.0 {
            1.0
        } else if min_side < 2500.0 {
            1.5
        } else {
            2.0
        }
}

pub fn select_node(tree: &mut Tree, available: Rect, selected: &mut Option<Vec<TreeView>>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position = Vec2::from(mouse_position());
        if available.contains(mouse_position) {
            let nodes_pointed = tree.get_nested_by_position(mouse_position);
            *selected = Some(TreeView::from_nodes(&nodes_pointed));
        } else {
            *selected = None;
        }
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        *selected = None;
    }
}

pub fn choose_and_draw_nested_nodes(
    mut tree: &mut Tree,
    units: &str,
    available: Rect,
    font_size: f32,
    searcher: &mut Searcher,
    selected: &mut Option<Vec<TreeView>>,
) {
    if let Some(nested_nodes) = searcher.get_result() {
        *selected = Some(nested_nodes.clone());
        draw_nested_nodes(units, available, font_size, &nested_nodes);
    } else if let Some(selected_nodes) = &selected {
        draw_nested_nodes(units, available, font_size, &selected_nodes);
    } else {
        *selected = draw_pointed_slice(units, &mut tree, available, font_size);
    }
    if is_key_pressed(KeyCode::Backspace) {
        if let Some(nested_nodes) = selected {
            nested_nodes.pop();
        }
    }
}

fn draw_pointed_slice(
    units: &str,
    treemap: &mut Tree,
    available: Rect,
    font_size: f32,
) -> Option<Vec<TreeView>> {
    let mouse_position = Vec2::from(mouse_position());
    if available.contains(mouse_position) {
        let nodes_pointed = treemap.get_nested_by_position(mouse_position);
        draw_nested_nodes(
            units,
            available,
            font_size,
            &TreeView::from_nodes(&nodes_pointed),
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            let deepest_child = nodes_pointed.last().unwrap();
            debug!("{:#?}", deepest_child);
            return Some(TreeView::from_nodes(&nodes_pointed));
        }
    }
    return None;
}

fn draw_nested_nodes(units: &str, available: Rect, font_size: f32, nested_nodes: &Vec<TreeView>) {
    if nested_nodes.len() > 0 {
        let deepest_child = nested_nodes.last().unwrap();
        let text = format!("{}: {} {}", deepest_child.name, deepest_child.size, units);

        // draw the color blocks in the nodes rect
        for (i, node) in nested_nodes.iter().enumerate() {
            let Rect { x, y, w, h } = round_rect(node.rect.unwrap());
            // draw_rectangle_lines(x, y, w, h, 10.0, COLORS[i % COLORS.len()]);
            draw_rectangle(x, y, w, h, COLORS[i % COLORS.len()]);
        }
        let nodes_count = nested_nodes.len();

        // draw color background over the node name at the bottom
        for (i_rev, node) in nested_nodes.iter().rev().enumerate() {
            let dimensions = measure_text(&node.name, None, font_size as u16, 1.0);
            draw_rectangle(
                available.x,
                2.0 * available.y + available.h,
                dimensions.width,
                1.5 * font_size,
                COLORS[(nodes_count - 1 - i_rev) % COLORS.len()],
            );
        }
        draw_text(
            &text,
            available.x,
            2.0 * available.y + available.h + 1.0 * font_size,
            font_size,
            BLACK,
        );
    }
}

const COLORS: &[Color] = &[
    BEIGE,
    Color::new(1.0, 0.40, 0.40, 1.00),
    PINK,
    PURPLE,
    VIOLET,
    BLUE,
    SKYBLUE,
    GREEN,
    LIME,
    WHITE,
];

pub fn draw_nodes(node: &Tree, available: Rect, font_size: f32, thickness: f32, color: Color) {
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
            draw_nodes(child, available, font_size, thickness, color);
        }
    }
}

/// I think macroquad will draw blurry pixels if the position or size of a rectangle is not rounded.
pub fn round_rect(rect: Rect) -> Rect {
    let rounded_x = rect.x.round();
    let rounded_y = rect.y.round();
    Rect::new(
        rounded_x,
        rect.y.round(),
        (rect.x + rect.w).round() - rounded_x,
        (rect.y + rect.h).round() - rounded_y,
    )
}

pub fn all_draw(
    mut tree: &mut Tree,
    units: &str,
    available: Rect,
    font_size: f32,
    mut searcher: &mut Searcher,
    mut selected: &mut Option<Vec<TreeView>>,
) {
    select_node(&mut tree, available, &mut selected);

    choose_and_draw_nested_nodes(
        &mut tree,
        units,
        available,
        font_size,
        &mut searcher,
        &mut selected,
    );

    let Rect { x, y, w, h } = round_rect(available);
    draw_rectangle_lines(x, y, w, h + 1.0, 2.0, BLACK);
    draw_nodes(&tree, available, font_size, 1.0, BLACK);

    searcher.draw_search(&tree);
}
