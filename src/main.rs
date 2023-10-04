mod node;
mod bytes_per_file;

use macroquad::prelude::*;

type AnyError = Box<dyn std::error::Error>;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Code Tree";

const FONT_SIZE: f32 = 16.0;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let args: Vec<String> = std::env::args().collect();
    let folder = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    let tree = bytes_per_file::bytes_per_file(&folder).unwrap();
    let width = screen_width();
    let height = screen_height();
    let font_size = choose_font_size(width, height);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        let available_width = width * 0.9;
        let available_height = height * 0.75;

        draw_rectangle_lines(width * 0.05, height * 0.05, available_width, available_height, 1.0, BLACK);

        let mut previous_end = 0.0;
        for child in &tree.children {
            let child_width = child.get_size() as f32 / tree.get_size() as f32 * available_width;
            let child_height = available_height;
            let x = width * 0.05 + previous_end;
            let y = height * 0.05;
            draw_rectangle_lines(x, y, child_width, child_height, 1.0, BLACK);
            draw_text(&child.name, x + 10.0, y + 1.5 * font_size, font_size, BLACK);
            draw_text(&child.get_size().to_string(), x + 10.0, y + 3.0 * font_size, font_size, BLACK);
            previous_end += child_width;
        }
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
