mod node;
mod treemap;

use crate::treemap::{MapNode, MapNodeView};
use clap::Parser;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::InputText;
use std::path::PathBuf;
use std::time::Instant;

type AnyError = Box<dyn std::error::Error>;

mod arrangements {
    pub mod binary;
    pub mod linear;
}
mod metrics {
    pub mod bytes_per_file;
    pub mod word_mentions;
}

use crate::arrangements::binary;
use crate::node::Node;
use arrangements::linear;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Map";

const FONT_SIZE: f32 = 16.0;

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

/// Plot hierarchical metrics like file sizes in a folder structure.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// plot file sizes under this folder.
    #[arg(default_value = ".")]
    pub input_folder: PathBuf,

    /// Padding in pixels between hierarchies (e.g. 4).
    #[arg(short, long, default_value = "0")]
    pub padding: f32,

    /// arrangement algorithm: linear or binary.
    #[arg(short, long, default_value = "binary")]
    pub arrangement: String,

    /// metric to plot: bytes-per-file (or b), mentions-per-word (or w).
    #[arg(short, long, default_value = "bytes-per-file")]
    pub metric: String,
}

fn log_time_elapsed<R, F: Fn() -> R>(f: F, name: &str) -> R {
    let time_before = Instant::now();
    let result = f();
    let time_after = Instant::now();
    info!("{} took {:?}", name, time_after - time_before);
    result
}

macro_rules! log_time {
    ($e:expr $(,)?) => {{
        let time_before = Instant::now();
        let result = $e;
        let time_after = Instant::now();
        info!("{} took {:?}", stringify!($e), time_after - time_before);
        result
    }};
    ($e:expr, $name:expr $(,)?) => {{
        let time_before = Instant::now();
        let result = $e;
        let time_after = Instant::now();
        info!("{} took {:?}", $name, time_after - time_before);
        result
    }};
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let Cli {
        input_folder,
        padding,
        arrangement,
        metric,
    } = Cli::parse();

    let (tree, units) = log_time!(compute_metrics(&input_folder, &metric), "computing metrics");
    let mut treemap = log_time!(MapNode::new(tree), "converting to MapNode");

    let width = screen_width();
    let height = screen_height();
    let available = round_rect(Rect::new(
        width * 0.05,
        width * 0.05, // yes, width, not height. this makes the padding the same in both directions
        width * 0.9,
        height * 0.75,
    ));

    log_time!(
        arrange(padding, arrangement, &mut treemap, available),
        "arrangement"
    );
    log_time!(log_counts(&treemap));

    let font_size = choose_font_size(width, height);
    let mut searcher = Searcher::new(
        Rect::new(
            available.x,
            available.y + available.h + font_size * 3.0,
            available.w,
            font_size * 1.5,
        ),
        font_size,
    );
    let mut selected = None;
    loop {
        if (is_key_pressed(KeyCode::Q)
            && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)))
            || is_key_down(KeyCode::Escape)
        {
            break;
        }
        clear_background(LIGHTGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = Vec2::from(mouse_position());
            if available.contains(mouse_position) {
                let nodes_pointed = treemap.get_nested_by_position(mouse_position);
                selected = Some(MapNodeView::from_nodes(&nodes_pointed));
            } else {
                selected = None;
            }
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            selected = None;
        }
        if let Some(nested_nodes) = searcher.get_result() {
            selected = Some(nested_nodes.clone());
            draw_nested_nodes(units, available, font_size, &nested_nodes);
        } else if let Some(selected_nodes) = &selected {
            draw_nested_nodes(units, available, font_size, &selected_nodes);
        } else {
            selected = draw_pointed_slice(units, &mut treemap, available, font_size);
        }

        let Rect { x, y, w, h } = round_rect(available);
        draw_rectangle_lines(x, y, w, h + 1.0, 2.0, BLACK);
        draw_nodes(&treemap, available, font_size, 1.0, BLACK);

        searcher.draw_search(&treemap);

        next_frame().await
    }
    // println!("{:#?}", treemap);
    Ok(())
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

fn compute_metrics(input_folder: &PathBuf, metric: &str) -> (Node, &'static str) {
    let (tree, units) = match metric {
        "bytes-per-file" | "b" => (
            metrics::bytes_per_file::bytes_per_file(&input_folder).unwrap(),
            "bytes",
        ),
        "mentions-per-word" | "w" => (
            metrics::word_mentions::word_mentions(&input_folder).unwrap(),
            "mentions",
        ),
        _ => panic!(
            "Unknown metric: {}. valid ones are: bytes-per-file, b, mentions-per-word, w",
            metric
        ),
    };
    (tree, units)
}

fn arrange(padding: f32, arrangement: String, mut treemap: &mut MapNode, available: Rect) {
    if arrangement == "linear" {
        linear::arrange(&mut treemap, available, padding);
    } else if arrangement == "binary" {
        binary::arrange(&mut treemap, available);
    } else {
        panic!(
            "Unknown arrangement algorithm: {}. valid ones are: linear, binary",
            arrangement
        );
    }
}

fn log_counts(treemap: &MapNode) {
    let counts = treemap.count();
    info!(
        "There are {} items. {} including the hierarchy levels",
        counts.leafs, counts.total
    );
    let visible_counts = treemap.count_visible();
    if visible_counts.total != counts.total || visible_counts.leafs != counts.leafs {
        info!(
            "However, only {} items and {} items+hierarchies are visible",
            visible_counts.leafs, visible_counts.total
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

fn draw_pointed_slice<'a>(
    units: &str,
    treemap: &'a mut MapNode,
    available: Rect,
    font_size: f32,
) -> Option<Vec<MapNodeView>> {
    let mouse_position = Vec2::from(mouse_position());
    if available.contains(mouse_position) {
        let nodes_pointed = treemap.get_nested_by_position(mouse_position);
        draw_nested_nodes(
            units,
            available,
            font_size,
            &MapNodeView::from_nodes(&nodes_pointed),
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            let deepest_child = nodes_pointed.last().unwrap();
            debug!("{:#?}", deepest_child);
            return Some(MapNodeView::from_nodes(&nodes_pointed));
        }
    }
    return None;
}

fn draw_nested_nodes(
    units: &str,
    available: Rect,
    font_size: f32,
    nested_nodes: &Vec<MapNodeView>,
) {
    let deepest_child = nested_nodes.last().unwrap();
    let text = format!("{}: {} {}", deepest_child.name, deepest_child.size, units);
    // let previous_end = available.x;

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

fn draw_nodes(node: &MapNode, available: Rect, font_size: f32, thickness: f32, color: Color) {
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

/// I think macroquad will draw blurry pixels if the position or size of a rectangle is not rounded.
fn trunc_rect(rect: Rect) -> Rect {
    Rect::new(
        rect.x.trunc(),
        rect.y.trunc(),
        rect.w.trunc(),
        rect.h.trunc(),
    )
}

pub struct Searcher {
    ui_id: u64,
    tag: String,
    tag_pos: Vec2,
    font_size: f32,
    rect: Rect,
    search_word: String,
    focused: bool,
    result: Option<String>,
    results: Vec<String>,
    nested_results: Option<Vec<MapNodeView>>,
}
impl Searcher {
    pub fn new(mut rect: Rect, font_size: f32) -> Self {
        let tag = "Search (f): ".to_string();
        let text_dimensions = measure_text(&tag, None, font_size as u16, 1.0);
        let tag_pos = Vec2::new(rect.x, rect.y);
        rect.x += text_dimensions.width;
        rect.w -= text_dimensions.width;
        rect.y -= font_size;
        Self {
            ui_id: hash!(rect.x as u64, rect.y as u64, rect.w as u64, rect.h as u64),
            tag,
            tag_pos,
            font_size,
            rect,
            search_word: "".to_string(),
            results: Vec::new(),
            focused: false,
            result: None,
            nested_results: None,
        }
    }
    fn draw_search_box(&mut self) -> Option<String> {
        draw_text(
            &self.tag,
            self.tag_pos.x,
            self.tag_pos.y,
            self.font_size,
            BLACK,
        );

        InputText::new(self.ui_id)
            .position(self.rect.point())
            .size(self.rect.size())
            .ui(&mut root_ui(), &mut self.search_word);

        if is_key_pressed(KeyCode::F) {
            self.set_focus(true);
        } else if is_key_pressed(KeyCode::Enter) {
            self.set_focus(false);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            self.set_focus(self.rect.contains(Vec2::from(mouse_position())));
        }
        if self.focused && !self.search_word.is_empty() {
            Some(self.search_word.clone())
        } else {
            None
        }
    }

    fn set_focus(&mut self, enabled: bool) {
        if enabled {
            self.focused = true;
            root_ui().set_input_focus(self.ui_id);
        } else {
            self.focused = false;
            root_ui().clear_input_focus();
        }
    }

    fn draw_search(&mut self, treemap: &MapNode) {
        let previous_search = self.search_word.clone();
        if let Some(search_word) = self.draw_search_box() {
            if previous_search != search_word {
                self.results = treemap.search(&search_word, 20);
                if let Some(first) = self.results.first() {
                    self.nested_results =
                        Some(MapNodeView::from_nodes(&treemap.get_nested_by_name(first)));
                }
            }
            let results = &self.results;
            let line_height = 1.2 * self.font_size;
            let horizontal_pad = 0.4 * line_height;
            if results.len() > 0 {
                let longest = results.iter().max_by_key(|w| w.len()).unwrap();
                let dimensions = measure_text(longest, None, self.font_size as u16, 1.0);
                let w = dimensions.width + 2.0 * horizontal_pad;
                let h = (results.len() as f32 + 0.5) * line_height;
                let space = 0.0 * line_height;
                draw_rectangle(self.rect.x, self.rect.y - h - space, w, h, LIGHTGRAY);
                draw_rectangle_lines(self.rect.x, self.rect.y - h - space, w, h, 2.0, BLACK);
                draw_rectangle_lines(
                    self.rect.x + horizontal_pad * 0.5,
                    self.rect.y + horizontal_pad * 0.5 - h - space,
                    w - horizontal_pad,
                    line_height,
                    2.0,
                    GRAY,
                );
                for (i, result) in results.iter().enumerate() {
                    draw_text(
                        result,
                        (self.rect.x + horizontal_pad).round(),
                        (self.rect.y - h - space + (i as f32 + 1.0) * line_height).round(),
                        self.font_size,
                        BLACK,
                    );
                }
                self.result = results.first().cloned();
            } else {
                let dimensions = measure_text(&self.search_word, None, self.font_size as u16, 1.0);
                draw_text(
                    "No results",
                    (self.rect.x + dimensions.width + 2.0 * horizontal_pad).round(),
                    (self.tag_pos.y - line_height).round(),
                    self.font_size,
                    GRAY,
                );
                self.result = None
            }
        } else {
            self.result = None;
        }
    }
    fn get_result(&self) -> Option<&Vec<MapNodeView>> {
        if self.focused {
            self.nested_results.as_ref()
        } else {
            None
        }
    }
}
