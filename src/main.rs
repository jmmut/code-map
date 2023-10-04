mod node;
mod bytes_per_file;
mod treemap;

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
    let mut treemap = treemap::MapNode::new(tree);
    treemap.arrange(16.0 / 9.0);
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

        for child in &treemap.children {
            let w = child.rect.w * available_width;
            let h = child.rect.h * available_height;
            let x = width * 0.05 + child.rect.x * available_width;
            let y = height * 0.05 + child.rect.y * available_height;
            draw_rectangle_lines(x, y, w, h, 1.0, BLACK);
            draw_text(&child.name, x + 1.5 * font_size, y + 1.5 * font_size, font_size, BLACK);
            draw_text(&child.size.to_string(), x + 1.5 * font_size, y + 3.0 * font_size, font_size, BLACK);
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
