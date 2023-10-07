mod node;
mod treemap;

use crate::treemap::MapNode;
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
    BEIGE, ORANGE, RED, PINK, PURPLE, VIOLET, BLUE, SKYBLUE, GREEN, LIME, WHITE,
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
            available.y + available.h + font_size * 2.0,
            available.w,
            font_size * 1.5,
        ),
        font_size,
    );
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);

        draw_pointed_slice(units, &mut treemap, available, font_size);
        let Rect { x, y, w, h } = round_rect(available);
        draw_rectangle_lines(x, y, w, h + 1.0, 2.0, BLACK);
        draw_nodes(&treemap, available, font_size, 1.0, BLACK);
        if let Some(search_word) = searcher.draw_search() {
            info!("{:?}", treemap.search(&search_word, 20));
        }
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

fn draw_pointed_slice(units: &str, treemap: &mut MapNode, available: Rect, font_size: f32) {
    let mouse_position = Vec2::from(mouse_position());
    if available.contains(mouse_position) {
        let nodes_pointed = treemap.overlapping(mouse_position);
        let deepest_child = nodes_pointed.last().unwrap();
        let text = format!("{}: {} {}", deepest_child.name, deepest_child.size, units);
        // let previous_end = available.x;
        for (i, node) in nodes_pointed.iter().enumerate() {
            let Rect { x, y, w, h } = round_rect(node.rect.unwrap());
            // draw_rectangle_lines(x, y, w, h, 10.0, COLORS[i % COLORS.len()]);
            draw_rectangle(x, y, w, h, COLORS[i % COLORS.len()]);
        }
        let nodes_count = nodes_pointed.len();
        for (i_rev, node) in nodes_pointed.iter().rev().enumerate() {
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
        if is_mouse_button_pressed(MouseButton::Left) {
            debug!("{:#?}", deepest_child)
        }
    }
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
    tag: String,
    tag_pos: Vec2,
    font_size: f32,
    rect: Rect,
    search_word: String,
}
impl Searcher {
    pub fn new(mut rect: Rect, font_size: f32) -> Self {
        let tag = "Search (F): ".to_string();
        let text_dimensions = measure_text(&tag, None, font_size as u16, 1.0);
        let tag_pos = Vec2::new(rect.x, rect.y);
        rect.x += text_dimensions.width;
        rect.w -= text_dimensions.width;
        rect.y -= font_size;
        Self {
            tag,
            tag_pos,
            font_size,
            rect,
            search_word: "".to_string(),
        }
    }
    fn draw_search(&mut self) -> Option<String> {
        let input_id = hash!();
        draw_text(
            &self.tag,
            self.tag_pos.x,
            self.tag_pos.y,
            self.font_size,
            BLACK,
        );
        InputText::new(input_id)
            .position(self.rect.point())
            .size(self.rect.size())
            .ui(&mut root_ui(), &mut self.search_word);
        if is_key_pressed(KeyCode::F) {
            info!("focusing input");
            root_ui().set_input_focus(input_id)
        }
        if self.search_word.is_empty() {
            None
        } else {
            Some(self.search_word.clone())
        }
    }
}
