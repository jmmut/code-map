use crate::log_time;
use crate::tree::{Tree, TreeView};
use crate::ui::buttons::draw_buttons;
use crate::ui::map_and_path::{choose_and_draw_map_and_path, draw_nodes_lines_cached};
use crate::ui::rect_utils::{draw_rect, round_rect};
use crate::ui::searcher::Searcher;
use clipboard_rs::{Clipboard, ClipboardContext};
use macroquad::math::f32;
use macroquad::prelude::{
    BLACK, Color, FilterMode, LIGHTGRAY, MouseButton, Rect, RenderTarget, Vec2, clear_background,
    draw_text, is_mouse_button_pressed, measure_text, mouse_position, screen_height, screen_width,
    vec2,
};

mod buttons;
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
    refresh: bool,
    refresh_lines: bool,
    rendered_lines: RenderTarget,
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

        let render_target = macroquad::prelude::render_target(width as u32, height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
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
            refresh: false,
            refresh_lines: true,
            rendered_lines: render_target,
        }
    }

    pub fn draw(&mut self) {
        if self.refresh_lines {
            log_time!(
                draw_nodes_lines_cached(
                    &self.tree,
                    self.map_rect,
                    self.level,
                    self.font_size,
                    self.width,
                    self.height,
                    &mut self.rendered_lines
                ),
                "draw_nodes_lines"
            );
            self.refresh_lines = false;
        }
        self.maybe_rearrange();
        self.keys.capture_keys_this_frame();

        clear_background(LIGHTGRAY);

        // log_time!(
        choose_and_draw_map_and_path(
                &self.tree,
                &self.units,
                self.map_rect,
                self.font_size,
                &mut self.refresh_lines,
                &mut self.searcher,
                &mut self.selected,
                &mut self.level,
                &mut self.rendered_lines,
            )
        // , "choose_and_draw_map_and_path" )
        ;

        select_node_with_mouse(&self.tree, self.map_rect, &mut self.selected);

        self.searcher
            .draw_search(&self.tree, &self.keys.keycode_event_queue);

        self.act_on_buttons();
        if self.refresh_lines || self.refresh {
            self.draw_regenerate_warning();
        }
    }

    fn draw_regenerate_warning(&mut self) {
        let font_size = self.font_size * 4.0;
        let text = "Re-drawing grid...";
        let measures = measure_text(text, None, font_size as u16, 1.0);
        let horizontal_pad = font_size * 1.0;
        let Vec2 { x, y } =
            self.map_rect.center() - vec2(measures.width * 0.5, 0.0) - horizontal_pad;

        let measure = measure_text(text, None, font_size as u16, 1.0);
        let button_rect = Rect::new(x, y, measure.width + horizontal_pad * 2.0, font_size * 1.5);
        draw_rect(button_rect, Color::new(0.95, 0.95, 0.95, 0.95));
        draw_text(
            text,
            button_rect.x + horizontal_pad,
            button_rect.y + font_size,
            font_size,
            BLACK,
        );
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

            let render_target =
                // log_time!(
                macroquad::prelude::render_target(self.width as u32, self.height as u32)
                // , "reallocate lines texture")
            ;
            render_target.texture.set_filter(FilterMode::Nearest);
            self.rendered_lines = render_target;
            self.refresh = true;
            self.refresh_lines = true;
        }
    }

    fn act_on_buttons(&mut self) {
        let buttons = draw_buttons(self.map_rect, self.font_size);
        if buttons.copied {
            if let Some(parts) = &self.selected {
                let path = parts.last().map_or("", |view| &view.name);
                let ctx = ClipboardContext::new().unwrap();
                // let old = ctx.get_text().unwrap();
                // println!("copying {path} to clipboard, was {old}");
                ctx.set_text(path.to_string()).unwrap();
            }
        }
        self.refresh = buttons.refresh;
        if buttons.squareness {
            println!("squareness: {}", self.tree.compute_squareness())
        }
    }
    pub fn should_refresh(&self) -> bool {
        self.refresh
    }
    pub fn is_searcher_focused(&self) -> bool {
        self.searcher.is_focused()
    }
}

fn get_map_rect(width: f32, height: f32, font_size: f32) -> Rect {
    let small_pad = font_size * 2.5;
    let big_pad = font_size * 12.0;
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
