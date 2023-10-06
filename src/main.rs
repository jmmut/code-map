mod cli_args;
mod node;
mod treemap;

use crate::treemap::MapNode;
use macroquad::prelude::*;
use std::path::PathBuf;
use clap::Parser;

type AnyError = Box<dyn std::error::Error>;

mod arrangements {
    pub mod linear;
    pub mod binary;
}
mod metrics {
    pub mod bytes_per_file;
}

use arrangements::linear;
use crate::arrangements::binary;

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
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let Cli {
        input_folder,
        padding,
        arrangement,
    } = Cli::parse();
    let tree = metrics::bytes_per_file::bytes_per_file(&input_folder).unwrap();
    let units = "bytes";

    let mut treemap = MapNode::new(tree);
    let width = screen_width();
    let height = screen_height();
    let available = round_rect(Rect::new(
        width * 0.05,
        width * 0.05, // yes, width, not height. this makes the padding the same in both directions
        width * 0.9,
        height * 0.75,
    ));
    let time_before = std::time::Instant::now();
    if arrangement == "linear" {
        linear::arrange(&mut treemap, available, padding);
    } else {
        binary::arrange(&mut treemap, available);
    }
    let time_after = std::time::Instant::now();
    println!("arrangement took {:?}", time_after - time_before);

    let font_size = choose_font_size(width, height);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        draw_pointed_slice(units, &mut treemap, available, font_size);
        draw_nodes(&treemap, available, font_size, 1.0, BLACK);
        next_frame().await
    }
    // println!("{:#?}", tree);
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
    }
}

fn draw_nodes(node: &MapNode, available: Rect, font_size: f32, thickness: f32, color: Color) {
    if let Some(rect) = node.rect {
        let Rect { x, y, w, h } = trunc_rect(rect);
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
    Rect::new(
        rect.x.round(),
        rect.y.round(),
        rect.w.round(),
        rect.h.round(),
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
