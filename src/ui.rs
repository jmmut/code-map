mod input_text;
mod key_queue;
pub mod searcher;

use macroquad::color::{Color, BEIGE, BLUE, GREEN, LIME, PINK, PURPLE, SKYBLUE, VIOLET, WHITE};
use macroquad::math::f32;
use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, draw_text, is_key_pressed,
    is_mouse_button_pressed, measure_text, mouse_position, screen_height, screen_width, KeyCode,
    MouseButton, Rect, Vec2, BLACK, GRAY, LIGHTGRAY,
};

use crate::tree::{Tree, TreeView};
use crate::ui::searcher::Searcher;

const FONT_SIZE: f32 = 16.0;

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

pub struct Ui {
    pub tree: Tree,
    units: String,
    pub map_rect: Rect,
    searcher: Searcher,
    font_size: f32,
    selected: Option<Vec<TreeView>>,
    level: Option<usize>,
    keys: key_queue::OrderedEventHandler,
    arrange: fn(f32, String, &mut Tree, Rect),
    arrangement: String,
    width: f32,
    height: f32,
    padding: f32,
}

impl Ui {
    pub fn new(
        tree: Tree,
        units: &str,
        arrange: fn(f32, String, &mut Tree, Rect),
        arrangement: String,
        padding: f32,
    ) -> Self {
        let width = screen_width();
        let height = screen_height();
        let font_size = choose_font_size(width, height);
        let map_rect = get_map_rect(width, height, font_size);

        let searcher = Searcher::new(get_searcher_rect(map_rect, font_size), font_size);
        Self {
            tree,
            units: units.to_string(),
            map_rect,
            font_size,
            searcher,
            selected: None,
            level: None,
            keys: key_queue::OrderedEventHandler::new(),
            arrange,
            width,
            height,
            padding,
            arrangement,
        }
    }

    pub fn draw(&mut self) {
        self.maybe_rearrange();
        self.keys.capture_keys_this_frame();

        clear_background(LIGHTGRAY);

        choose_and_draw_map_and_path(
            &self.tree,
            &self.units,
            self.map_rect,
            self.font_size,
            &mut self.searcher,
            &mut self.selected,
            &mut self.level,
        );

        select_node_with_mouse(&self.tree, self.map_rect, &mut self.selected);

        self.searcher
            .draw_search(&self.tree, &self.keys.keycode_event_queue);
    }

    fn maybe_rearrange(&mut self) {
        let new_width = screen_width();
        let new_height = screen_height();
        if new_width != self.width || new_height != self.height {
            self.width = new_width;
            self.height = new_height;
            self.map_rect = get_map_rect(self.width, self.height, self.font_size);
            (self.arrange)(
                self.padding,
                self.arrangement.clone(),
                &mut self.tree,
                self.map_rect,
            );
            self.searcher
                .position(get_searcher_rect(self.map_rect, self.font_size));
        }
    }
}

fn get_map_rect(width: f32, height: f32, font_size: f32) -> Rect {
    let small_pad = font_size * 2.5;
    let big_pad = font_size * 9.5;
    let map_rect = round_rect(Rect::new(
        small_pad,
        small_pad,
        width - 2.0 * small_pad,
        height - small_pad - big_pad,
    ));
    map_rect
}

fn get_searcher_rect(map_rect: Rect, font_size: f32) -> Rect {
    Rect::new(
        map_rect.x,
        map_rect.y + map_rect.h + font_size * 3.0,
        map_rect.w,
        font_size * 1.5,
    )
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

fn select_node_with_mouse(tree: &Tree, map_rect: Rect, selected: &mut Option<Vec<TreeView>>) {
    let mouse_position = Vec2::from(mouse_position());
    if map_rect.contains(mouse_position) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let nodes_pointed = tree.get_nested_by_position(mouse_position);
            let new_nodes = TreeView::from_nodes(&nodes_pointed);
            if let Some(selected_nodes) = selected {
                if *selected_nodes == new_nodes {
                    *selected = None;
                } else {
                    *selected = Some(new_nodes);
                }
            } else {
                *selected = Some(new_nodes);
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            *selected = None;
        }
    }
}

fn choose_and_draw_map_and_path(
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
        // draw color background over the node name at the bottom
        let mut previous_width = 0.0;
        let path_y = map_rect.y + map_rect.h + font_size * 4.5;
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
                if let Some(level) = level_opt {
                    if *level == i {
                        *level_opt = None;
                    } else {
                        *level_opt = Some(i);
                    }
                } else {
                    *level_opt = Some(i);
                }
            }
            previous_width = dimensions.width;
        }

        // draw the text of the node name and the units
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

        // draw the color blocks in the nodes rect
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
}

fn is_rect_clicked(rect: &Rect, mouse_button: MouseButton) -> bool {
    is_mouse_button_pressed(mouse_button) && rect.contains(Vec2::from(mouse_position()))
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

/// I think macroquad will draw blurry pixels if the position or size of a rectangle is not rounded.
fn round_rect(rect: Rect) -> Rect {
    let rounded_x = rect.x.round();
    let rounded_y = rect.y.round();
    Rect::new(
        rounded_x,
        rect.y.round(),
        (rect.x + rect.w).round() - rounded_x,
        (rect.y + rect.h).round() - rounded_y,
    )
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
