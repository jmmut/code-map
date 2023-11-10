use macroquad::math::f32;
use macroquad::prelude::{
    clear_background, is_mouse_button_pressed, mouse_position, screen_height, screen_width,
    MouseButton, Rect, Vec2, LIGHTGRAY,
};

use crate::tree::{Tree, TreeView};
use crate::ui::map_and_path::choose_and_draw_map_and_path;
use crate::ui::rect_utils::round_rect;
use crate::ui::searcher::Searcher;

mod input_text;
mod key_queue;
mod map_and_path;
pub mod rect_utils;
pub mod searcher;

const FONT_SIZE: f32 = 16.0;

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
            self.selected = None;
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
            set_if_different_or_unset_if_same(selected, new_nodes);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            *selected = None;
        }
    }
}

fn set_if_different_or_unset_if_same<T: PartialEq>(selected: &mut Option<T>, new_nodes: T) {
    if let Some(selected_nodes) = selected {
        if *selected_nodes == new_nodes {
            *selected = None;
        } else {
            *selected = Some(new_nodes);
        }
    } else {
        *selected = Some(new_nodes);
    }
}
